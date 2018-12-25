use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Lines;

#[derive(Debug, Clone, Copy)]
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
struct Instruction {
    opcode: Opcode,
    // 0, 1, 2, or 3 for below
    in_a: usize,
    in_b: usize,
    out: usize,
}

#[derive(Debug, PartialEq)]
struct Machine {
    regs: [u32; 4],
}

fn step_instruction(machine: &mut Machine, inst: &Instruction) {
    let ai = inst.in_a as u32;
    let bi = inst.in_b as u32;
    let ar = machine.regs[inst.in_a];
    let br = machine.regs[inst.in_b];
    let c = &mut machine.regs[inst.out];
    match inst.opcode {
        Addr => 
            *c = ar + br,
        Addi =>
            *c = ar + bi,
        Mulr =>
            *c = ar * br,
        Muli =>
            *c = ar * bi,
        Banr =>
            *c = ar & br,
        Bani =>
            *c = ar & bi,
        Borr =>
            *c = ar | br,
        Bori =>
            *c = ar | bi,
        Setr =>
            *c = ar,
        Seti =>
            *c = ai,
        Gtir =>
            *c = if ai > br { 1 } else { 0 },
        Gtri =>
            *c = if ar > bi { 1 } else { 0 },
        Gtrr =>
            *c = if ar > br { 1 } else { 0 },
        Eqir =>
            *c = if ai == br { 1 } else { 0 },
        Eqri =>
            *c = if ar == bi { 1 } else { 0 },
        Eqrr =>
            *c = if ar == br { 1 } else { 0 },
    }
}

fn similar_opcodes(begin: &Machine, end: &Machine, code: &[usize; 4]) -> usize {
    let ops = [Addr, Addi, Mulr, Muli, Banr, Bani, Borr, Bori, Setr, Seti, Gtir, Gtri, Gtrr, Eqir, Eqri, Eqrr];
    ops.iter().filter(|&op| {
        let mut mach = Machine { regs: begin.regs };
        let inst = Instruction { opcode: *op, in_a: code[1], in_b: code[2], out: code[3] };
        step_instruction(&mut mach, &inst);
        mach == *end
    }).count()
}

fn behave_like_3_or_more(samples: &[(Machine, Machine, [usize; 4])]) -> usize {
    samples.iter().filter(|sample| similar_opcodes(&sample.0, &sample.1, &sample.2) >= 3).count()
}

fn into_quad<T: std::convert::From<u8> + std::marker::Copy + std::str::FromStr>(sp: std::str::Split<'_, &str>) -> [T; 4] {
    let mut out = [T::from(42); 4];
    for (i, o) in sp.zip(out.iter_mut()) {
        match i.parse() {
            Ok(n) => *o = n,
            Err(_) => unreachable!()
        }
    }
    out
}

fn parse_machine(line: &str) -> Machine {
    let regs0 = line.split("[").nth(1).unwrap();
    let regs_str = regs0.split("]").nth(0).unwrap();
    let regs = into_quad(regs_str.split(", "));
    Machine { regs: regs }
}

fn parse_sample(inp: &mut Lines<BufReader<File>>) -> Option<(Machine, Machine, [usize; 4])> {
    let line = inp.next().unwrap().unwrap();
    if line == "" {
        // Two extra lines between samples and test program. Skip the other too
        inp.next();
        None
    } else {
        /*
         * Before: [2, 1, 1, 0]
         * 10 1 3 1
         * After:  [2, 1, 1, 0]
         */
        let a = parse_machine(&line);
        let instcode = into_quad(inp.next().unwrap().unwrap().split(" "));
        let b = parse_machine(&inp.next().unwrap().unwrap());
        let separator = inp.next().unwrap().unwrap();
        assert!(separator == "");

        Some((a, b, instcode))
    }
}

fn parse_samples(input: &mut Lines<BufReader<File>>) -> Vec<(Machine, Machine, [usize; 4])> {
    let mut out = Vec::new();
    while let Some(next) = parse_sample(input) {
        out.push(next);
    }
    out
}

fn parse_program(input: &mut Lines<BufReader<File>>) {
}

fn main() {
    let mut input = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap()).lines();
    let samples = parse_samples(&mut input);
    let test_program = parse_program(&mut input);
    println!("{}", behave_like_3_or_more(&samples));
}
