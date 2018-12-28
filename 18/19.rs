use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Lines;

#[derive(Debug)]
enum Opcode {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}
use Opcode::*;

#[derive(Debug)]
struct OpParseError {}

impl std::str::FromStr for Opcode {
    type Err = OpParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "addr" => Ok(Addr),
            "addi" => Ok(Addi),
            "mulr" => Ok(Mulr),
            "muli" => Ok(Muli),
            "banr" => Ok(Banr),
            "bani" => Ok(Bani),
            "borr" => Ok(Borr),
            "bori" => Ok(Bori),
            "setr" => Ok(Setr),
            "seti" => Ok(Seti),
            "gtir" => Ok(Gtir),
            "gtri" => Ok(Gtri),
            "gtrr" => Ok(Gtrr),
            "eqir" => Ok(Eqir),
            "eqri" => Ok(Eqri),
            "eqrr" => Ok(Eqrr),
            _ => Err(Self::Err {})
        }
    }
}

#[derive(Debug)]
struct Instruction {
    opcode: Opcode,
    in_a: usize,
    in_b: usize,
    out: usize,
}

#[derive(Debug)]
struct Machine {
    regs: [u32; 6],
    ip_reg: usize,
}

fn step_instruction(machine: &mut Machine, inst: &Instruction) {
    let ai = inst.in_a as u32;
    let bi = inst.in_b as u32;
    // or 0: irrelevant for immediate opcodes
    let ar = *machine.regs.get(inst.in_a).unwrap_or_else(|| &0);
    let br = *machine.regs.get(inst.in_b).unwrap_or_else(|| &0);
    let c = &mut machine.regs[inst.out];
    *c = match inst.opcode {
        Addr => ar + br,
        Addi => ar + bi,
        Mulr => ar * br,
        Muli => ar * bi,
        Banr => ar & br,
        Bani => ar & bi,
        Borr => ar | br,
        Bori => ar | bi,
        Setr => ar,
        Seti => ai,
        Gtir => if ai > br { 1 } else { 0 },
        Gtri => if ar > bi { 1 } else { 0 },
        Gtrr => if ar > br { 1 } else { 0 },
        Eqir => if ai == br { 1 } else { 0 },
        Eqri => if ar == bi { 1 } else { 0 },
        Eqrr => if ar == br { 1 } else { 0 },
    };
}

fn execute(mach: &mut Machine, program: &[Instruction]) {
    while mach.regs[mach.ip_reg] < (program.len() as u32) {
        let inst = &program[mach.regs[mach.ip_reg] as usize];
        //println!("ip {}: run {:?} for {:?}", mach.regs[mach.ip_reg], inst, mach.regs);
        step_instruction(mach, inst);
        // should technically break here too for "#ip 0" to be exact
        mach.regs[mach.ip_reg] += 1;
    }
}

fn reg_zero(program: &[Instruction], ip_reg: usize) -> u32 {
    let mut mach = Machine { regs: [0; 6], ip_reg: ip_reg};
    // mach.regs[0] = 1;
    execute(&mut mach, program);
    println!("{:?}", mach.regs);

    mach.regs[0]
}

fn parse_program(input: &mut Lines<BufReader<File>>) -> Vec<Instruction> {
    let mut program = Vec::new();
    for line in input.map(|lopt| lopt.unwrap()) {
        let mut words = line.split(' ');
        program.push(Instruction {
            opcode: words.next().unwrap().parse().unwrap(),
            in_a: words.next().unwrap().parse().unwrap(),
            in_b: words.next().unwrap().parse().unwrap(),
            out: words.next().unwrap().parse().unwrap(),
        });
    }
    program
}

fn parse_ip(line: &str) -> usize {
    assert!(line.as_bytes()[0] == b'#');
    line.split(' ').nth(1).unwrap().parse().unwrap()
}

#[derive(Debug)]
struct M {
    r0: u64,
    r1: u64,
    r2: u64,
    r3: u64,
    r4: u64,
    r5: u64,
}

#[allow(dead_code)]
fn f_orig(m: &mut M) {
    // clearly:
    // r1 goes from 0 to =r5 {
    //   r2 goes from 0 to =r5 {
    //     if r1 * r2 == r5 {
    //       // r1 is a divisor of r5, some combination of its prime factors
    //       r0 += r1;
    //     }
    //   }
    // }
    // r0 ends up being the sum of all things <= r5 that divide r5
    m.r1 = 1;
    m.r2 = 1;
    loop {
        m.r3 = m.r1 * m.r2;
        if m.r5 == m.r3 {
            m.r0 += m.r1;
        }
        m.r2 += 1;
        if m.r2 > m.r5 {
            m.r1 += 1;
            if m.r1 > m.r5 {
                return;
            }
            m.r2 = 1;
        }
    }
}

fn f(m: &mut M) {
    m.r1 = 1;
    m.r2 = 1;
    loop {
        m.r3 = m.r1 * m.r2;
        if m.r5 == m.r3 {
            println!("yes {} * {} = {}", m.r1, m.r2, m.r5);
            m.r0 += m.r1;
        }
        m.r2 += 1;
        // speedup: detect & exit the outer calculation early
        if m.r2 * m.r1 > m.r5 {
            m.r1 += 1;
            if m.r1 > m.r5 {
                return;
            }
            m.r2 = 1;
        }
    }
}

fn a(m: &mut M) {
    m.r5 = (m.r5+2) * (m.r5+2) * 209;
    m.r3 = (m.r3+4) * 22 + 21;
    m.r5 += m.r3;
    m.r3 = 10550400;
    m.r5 += 10550400;
    m.r0 = 0;
    f(m);
}

fn compiled() -> u64 {
    let mut m = M { r0: 1, r1: 0, r2: 0, r3: 0, r4: 0, r5: 0 };
    m.r0 = 1;
    a(&mut m);
    println!("{:?}", m);
    m.r0
}

fn main() {
    let mut input = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap()).lines();
    let ip_reg = parse_ip(&input.next().unwrap().unwrap());
    let program = parse_program(&mut input);
    println!("{}", reg_zero(&program, ip_reg));
    println!("{}", compiled());
}
