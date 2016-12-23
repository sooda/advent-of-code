use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

type JustVal = i32;
type JustReg = usize;

#[derive(Clone, Copy, Debug)]
enum Argument {
    Immediate(JustVal),
    Register(JustReg)
}
use Argument::*;

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Cpy(Argument, JustReg), // first goes to second
    Inc(JustReg),
    Dec(JustReg),
    Jnz(Argument, Argument), // if first { pc += second }
    Tgl(Argument),
    Nop // invalid instruction after a tgl, looks like it won't change back again with another tgl so no need to preserve args
}
use Instruction::*;

// cast to usize when indexing. can't cast negative values to usizes so this is signed
type Pc = JustVal;

fn tgl(program: &mut [Instruction], off: Pc) -> Instruction {
    match program[off as usize] {
        // one-arg insns
        Inc(arg) => Dec(arg),
        Dec(arg) => Inc(arg),
        Tgl(Register(arg)) => Inc(arg),
        Tgl(Immediate(_)) => Nop, // cannot inc immediate

        // two-arg
        Jnz(_, Immediate(_)) => Nop, // cannot copy to immediate
        Jnz(cmp_src, Register(off_dst)) => Cpy(cmp_src, off_dst),
        Cpy(src_cmp, dst_off) => Jnz(src_cmp, Register(dst_off)),
        _ => unreachable!()
    }
}

// return program counter diff
fn action(program: &mut [Instruction], pc: Pc, regs: &mut [JustVal; 4]) -> Pc {
    //println!(" running {:?}", program[pc as usize]);
    let jnz = |cmp: JustVal, off: JustVal| if cmp != 0 { pc + off } else { pc + 1 };
    match program[pc as usize] {
        Cpy(Immediate(src), dest) => regs[dest] = src,
        Cpy(Register(src), dest) => regs[dest] = regs[src],
        Inc(reg) => regs[reg] += 1,
        Dec(reg) => regs[reg] -= 1,
        Jnz(Immediate(cmp), Immediate(off)) => return jnz(cmp, off),
        Jnz(Immediate(cmp), Register(off)) => return jnz(cmp, regs[off]),
        Jnz(Register(cmp), Immediate(off)) => return jnz(regs[cmp], off),
        Jnz(Register(cmp), Register(off)) => return jnz(regs[cmp], regs[off]),
        Tgl(Register(off)) => { // no "tgl <immediate>" there apparently
            let i = pc + regs[off];
            if i >= 0 && (i as usize) < program.len() {
                program[i as usize] = tgl(program, i);
            }
        },
        _ => unreachable!()
    }

    pc + 1
}

fn parse(input: String) -> Instruction {
    let ops = input.split(" ").collect::<Vec<_>>();

    let getreg = |word: usize| (ops[word].as_bytes()[0] - ('a' as u8)) as usize;
    let getsome = |word: usize| {
        if let Ok(not_reg_but_number) = ops[word].parse::<JustVal>() {
            Immediate(not_reg_but_number)
        } else {
            Register(getreg(word))
        }
    };

    match ops[0] {
        "cpy" => Cpy(getsome(1), getreg(2)),
        "inc" => Inc(getreg(1)),
        "dec" => Dec(getreg(1)),
        "jnz" => Jnz(getsome(1), getsome(2)),
        "tgl" => Tgl(getsome(1)),
        _ => unreachable!()
    }
}

fn main() {
    let input = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap()).lines().map(Result::unwrap);
    let mut program = input.map(parse).collect::<Vec<_>>();
    let mut program2 = program.clone();

    let mut regs = [7, 0, 0, 0];
    let mut pc = 0;
    while pc != program.len() as Pc {
        //println!("{:?} {:?} {:?}", pc, regs, program);
        pc = action(&mut program, pc, &mut regs);
    }
    println!("{:?}", regs);

    let mut regs = [12, 0, 0, 0];
    let mut pc = 0;
    while pc != program2.len() as Pc {
        //println!("{:?} {:?} {:?}", pc, regs, program);
        pc = action(&mut program2, pc, &mut regs);
    }
    println!("{:?}", regs);
}
