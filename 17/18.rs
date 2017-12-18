use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

// Can't index a vector of registers with char, and indexin a mut hashmap is annoying, so the a-z
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
    Snd(Arg),
    Set(Reg, Arg),
    Add(Reg, Arg),
    Mul(Reg, Arg),
    Mod(Reg, Arg),
    Rcv(Arg),
    Jgz(Arg, Arg)
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
        "snd" => Snd(read_arg(1)),
        "set" => Set(read_reg(1), read_arg(2)),
        "add" => Add(read_reg(1), read_arg(2)),
        "mul" => Mul(read_reg(1), read_arg(2)),
        "mod" => Mod(read_reg(1), read_arg(2)),
        "rcv" => Rcv(read_arg(1)),
        "jgz" => Jgz(read_arg(1), read_arg(2)),
        _ => unreachable!()
    }
}

struct Machine {
    pc: usize,
    regs: Vec<Val>,
    last_snd: Val,
    last_rcv: Val
}

fn execute(machine: &mut Machine, program: &[Instruction]) {
    match program[machine.pc] {
        Snd(RegisterArg(snd)) =>
            machine.last_snd = machine.regs[snd],
        Snd(ValueArg(snd)) =>
            machine.last_snd = snd,
        Set(dst, RegisterArg(src)) =>
            machine.regs[dst] = machine.regs[src],
        Set(dst, ValueArg(val)) =>
            machine.regs[dst] = val,
        Add(dst, RegisterArg(src)) =>
            machine.regs[dst] += machine.regs[src],
        Add(dst, ValueArg(val)) =>
            machine.regs[dst] += val,
        Mul(dst, RegisterArg(src)) =>
            machine.regs[dst] *= machine.regs[src],
        Mul(dst, ValueArg(val)) =>
            machine.regs[dst] *= val,
        Mod(dst, RegisterArg(src)) =>
            machine.regs[dst] %= machine.regs[src],
        Mod(dst, ValueArg(val)) =>
            machine.regs[dst] %= val,
        Rcv(RegisterArg(reg)) =>
            if machine.regs[reg] != 0 { machine.last_rcv = machine.last_snd },
        Rcv(ValueArg(val)) =>
            if val != 0 { machine.last_rcv = machine.last_snd },
        Jgz(ref cmp, ref off) => {
            let cmp = match *cmp {
                RegisterArg(reg) => machine.regs[reg],
                ValueArg(arg) => arg
            };
            let off = match *off {
                RegisterArg(reg) => machine.regs[reg],
                ValueArg(arg) => arg
            };
            if cmp > 0 {
                machine.pc = (machine.pc as Val + off) as usize;
                return;
            }
        }
    }

    machine.pc += 1;
}

fn first_rcv(program: &[Instruction]) -> Val {
    // char iteration is kludgy unfortunately
    let regs = vec![0; (b'z' - b'a' + 1) as usize];
    let mut machine = Machine { pc: 0, regs: regs, last_snd: 0, last_rcv: 0 };
    while machine.pc < program.len() && machine.last_rcv == 0 {
        execute(&mut machine, program);
    }

    machine.last_rcv
}

fn main() {
    let program = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap())
        .lines().map(|x| parse_line(&x.unwrap())).collect::<Vec<_>>();
    println!("{}", first_rcv(&program));
}
