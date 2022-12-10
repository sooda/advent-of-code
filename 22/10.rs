use std::io::{self, BufRead};

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Noop,
    Addx(i32),
}
use Instruction::*;

struct Machine {
    x: i32,
    pipeline_stage: bool,
}

fn step_instruction(mach: &mut Machine, inst: Instruction) -> bool {
    match (inst, mach.pipeline_stage) {
        ( Noop, _) => (),
        (Addx(_), false) => mach.pipeline_stage = true,
        (Addx(x), true) => { mach.x += x; mach.pipeline_stage = false; },
    };
    mach.pipeline_stage
}

fn signal_strength(program: &[Instruction]) -> i32 {
    let mut mach = Machine { x: 1, pipeline_stage: false };
    let mut sum = 0;
    let mut cycle = 1;
    for &inst in program.iter() {
        loop {
            if cycle <= 120 && cycle == 20 || (cycle - 20) % 40 == 0 {
                let signal_strength = cycle * mach.x;
                sum += signal_strength;
            }

            let delay = step_instruction(&mut mach, inst);
            if false {
                println!("cycle executed {} x {} for {:?}", cycle, mach.x, inst);
            }
            cycle += 1;
            if !delay {
                break;
            }
        }
    }

    sum
}

fn parse_instruction(input: &str) -> Instruction {
    let mut sp = input.split(' ');
    match sp.next().unwrap() {
        "noop" => Noop,
        "addx" => Addx(sp.next().unwrap().parse().unwrap()),
        _ => panic!("bad opcode"),
    }
}

fn main() {
    let program: Vec<_> = io::stdin().lock().lines()
        .map(|line| parse_instruction(&line.unwrap()))
        .collect();
    println!("{}", signal_strength(&program));
}
