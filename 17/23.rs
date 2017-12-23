use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

// Can't index a vector of registers with char, and indexing a mut hashmap is annoying, so the a-h
// range is modified to just numbers.
type Reg = usize;
type Val = i64;

// some instructions can take either a register or an immediate value
#[derive(Debug)]
enum Arg {
    RegisterArg(Reg),
    ValueArg(Val)
}
use Arg::*;

#[derive(Debug)]
enum Instruction {
    Set(Reg, Arg),
    Sub(Reg, Arg),
    Mul(Reg, Arg),
    Jnz(Arg, Arg)
}
use Instruction::*;

fn parse_line(line: &str) -> Instruction {
    let words = line.split(" ").collect::<Vec<_>>();

    let read_reg = |idx: usize| (words[idx].as_bytes()[0] - b'a') as Reg;
    let read_arg = |idx: usize| {
        if let Ok(number) = words[idx].parse::<Val>() {
            ValueArg(number)
        } else {
            RegisterArg(read_reg(idx))
        }
    };

    match words[0] {
        "set" => Set(read_reg(1), read_arg(2)),
        "sub" => Sub(read_reg(1), read_arg(2)),
        "mul" => Mul(read_reg(1), read_arg(2)),
        "jnz" => Jnz(read_arg(1), read_arg(2)),
        _ => unreachable!()
    }
}

struct Coprocessor {
    pc: usize,
    regs: Vec<Val>,
    mul_invocations: usize
}

fn execute_debugmode(machine: &mut Coprocessor, program: &[Instruction]) {
    match program[machine.pc] {
        Set(dst, RegisterArg(src)) =>
            machine.regs[dst] = machine.regs[src],
        Set(dst, ValueArg(val)) =>
            machine.regs[dst] = val,
        Sub(dst, RegisterArg(src)) =>
            machine.regs[dst] -= machine.regs[src],
        Sub(dst, ValueArg(val)) =>
            machine.regs[dst] -= val,
        Mul(dst, RegisterArg(src)) => {
            machine.regs[dst] *= machine.regs[src];
            machine.mul_invocations += 1;
        },
        Mul(dst, ValueArg(val)) => {
            machine.regs[dst] *= val;
            machine.mul_invocations += 1;
        },
        Jnz(ref cmp, ref off) => {
            let cmp = match *cmp {
                RegisterArg(reg) => machine.regs[reg],
                ValueArg(arg) => arg
            };
            let off = match *off {
                RegisterArg(reg) => machine.regs[reg],
                ValueArg(arg) => arg
            };
            if cmp != 0 {
                machine.pc = (machine.pc as Val + off) as usize;
                return;
            }
        }
    }

    machine.pc += 1;
}

fn mul_count(program: &[Instruction]) -> usize {
    // char iteration is kludgy unfortunately
    let regs = vec![0; (b'h' - b'a' + 1) as usize];
    let mut machine = Coprocessor { pc: 0, regs: regs, mul_invocations: 0 };
    while machine.pc < program.len() {
        execute_debugmode(&mut machine, program);
    }

    machine.mul_invocations
}

fn main() {
    let program = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap())
        .lines().map(|x| parse_line(&x.unwrap())).collect::<Vec<_>>();
    println!("{}", mul_count(&program));
}
