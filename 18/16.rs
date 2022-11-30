use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Lines;

use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

const OPCODES: usize = 16;

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

fn similar_to(op: &Opcode, begin: &Machine, end: &Machine, code: &[usize; 4]) -> bool {
    let mut mach = Machine { regs: begin.regs };
    let inst = Instruction { opcode: *op, in_a: code[1], in_b: code[2], out: code[3] };
    step_instruction(&mut mach, &inst);
    mach == *end
}

fn similar_opcodes(begin: &Machine, end: &Machine, code: &[usize; 4]) -> usize {
    let ops = [Addr, Addi, Mulr, Muli, Banr, Bani, Borr, Bori, Setr, Seti, Gtir, Gtri, Gtrr, Eqir, Eqri, Eqrr];
    ops.iter().filter(|&op| similar_to(op, begin, end, code)).count()
}

type Sample = (Machine, Machine, [usize; 4]);

fn behave_like_3_or_more(samples: &[Sample]) -> usize {
    samples.iter().filter(|sample| similar_opcodes(&sample.0, &sample.1, &sample.2) >= 3).count()
}

fn unique_match(i: usize, samples: &[Sample], ops_remaining: &HashSet<Opcode>) -> Option<Opcode> {
    let mut found = None;
    for op in ops_remaining {
        let complete_match = samples.iter()
            .filter(|&(_, _, code)| code[0] == i)
            .all(|(begin, end, code)| similar_to(op, &begin, &end, &code));
        if complete_match {
            if found.is_none() {
                found = Some(*op);
            } else {
                // not unique
                return None;
            }
        }
    }
    found
}

type Coding = [Opcode; OPCODES];

fn deduce_coding(samples: &[Sample]) -> Coding {
    let mut ops_remaining: HashSet<Opcode> = [
        Addr, Addi, Mulr, Muli, Banr, Bani, Borr, Bori, Setr, Seti, Gtir, Gtri, Gtrr, Eqir, Eqri, Eqrr
    ].into_iter().collect();
    let mut found_codes = [None; OPCODES];
    while !ops_remaining.is_empty() {
        for (i, mapping) in found_codes.iter_mut().enumerate().filter(|(_, m)| m.is_none()) {
            if let Some(op) = unique_match(i, samples, &ops_remaining) {
                ops_remaining.remove(&op);
                *mapping = Some(op);
            }
        }
    }

    let mut unwrapped = [Addr; OPCODES];
    for (found, ret) in found_codes.iter().zip(unwrapped.iter_mut()) {
        *ret = found.unwrap();
    }
    unwrapped
}

fn execute(mach: &mut Machine, program: &[Instruction]) {
    for inst in program {
        println!("{:?}", mach.regs);
        step_instruction(mach, inst);
    }
}

fn reg_zero(program: &[Instruction]) -> u32 {
    let mut mach = Machine { regs: [0; 4] };
    execute(&mut mach, program);
    println!("{:?}", mach.regs);

    mach.regs[0]
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

fn parse_sample(inp: &mut Lines<BufReader<File>>) -> Option<Sample> {
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

fn parse_samples(input: &mut Lines<BufReader<File>>) -> Vec<Sample> {
    let mut out = Vec::new();
    while let Some(next) = parse_sample(input) {
        out.push(next);
    }
    out
}

fn parse_program(input: &mut Lines<BufReader<File>>, coding: &Coding) -> Vec<Instruction> {
    let mut program = Vec::new();
    for line in input.map(|lopt| lopt.unwrap()) {
        let instcode = into_quad(line.split(" "));
        program.push(Instruction {
            opcode: coding[instcode[0]],
            in_a: instcode[1],
            in_b: instcode[2],
            out: instcode[3]
        });
    }
    program
}

fn main() {
    let mut input = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap()).lines();
    let samples = parse_samples(&mut input);
    println!("{}", behave_like_3_or_more(&samples));

    let coding = deduce_coding(&samples);
    let test_program = parse_program(&mut input, &coding);
    println!("{}", reg_zero(&test_program));
}
