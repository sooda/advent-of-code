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
    Nop, // invalid instruction after a tgl, looks like it won't change back again with another tgl so no need to preserve args
    Out(Argument)
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

// return program counter diff and sometimes a new reg from the out instruction
fn action(program: &mut [Instruction], pc: Pc, regs: &mut [JustVal; 4]) -> (Pc, Option<JustVal>) {
    //println!(" running {:?}", program[pc as usize]);
    let jnz = |cmp: JustVal, off: JustVal| if cmp != 0 { (pc + off, None) } else { (pc + 1, None) };
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
        Out(Register(reg)) => return (pc + 1, Some(regs[reg])),
        _ => unreachable!()
    }

    (pc + 1, None)
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
        "out" => Out(getsome(1)),
        _ => unreachable!()
    }
}

fn main() {
    let input = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap()).lines().map(Result::unwrap);
    let mut program = input.map(parse).collect::<Vec<_>>();

    for start_reg in 0.. {
        let mut regs = [start_reg, 0, 0, 0];
        let mut pc = 0;
        // repeats in blocks of 12
        let mut outs = [0; 12];
        { // block to drop outs_it after we're done
            let mut outs_it = outs.iter_mut();
            while pc != program.len() as Pc {
                let (pc_, out) = action(&mut program, pc, &mut regs);
                pc = pc_;
                if let Some(out) = out {
                    if let Some(put) = outs_it.next() {
                        *put = out;
                    } else {
                        break;
                    }
                }
            }
        }
        println!("{:?} {:?} {:?}", start_reg, regs, outs);
        if outs == [0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1] {
            break;
        }
    }
}
