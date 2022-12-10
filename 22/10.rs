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

fn execute(program: &[Instruction]) -> (i32, String) {
    let mut mach = Machine { x: 1, pipeline_stage: false };
    let mut strength_sum = 0;
    let mut cycle = 1;
    let mut gfx = Vec::<char>::new();
    for &inst in program.iter() {
        loop {
            if cycle <= 120 && cycle == 20 || (cycle - 20) % 40 == 0 {
                let signal_strength = cycle * mach.x;
                strength_sum += signal_strength;
            }

            let xpos = (cycle - 1) % 40;
            gfx.push(if (xpos - mach.x).abs() <= 1 {
                '#'
            } else {
                '.'
            });
            if xpos == 39 {
                gfx.push('\n');
            }

            let delay = step_instruction(&mut mach, inst);
            cycle += 1;
            if !delay {
                break;
            }
        }
    }

    (strength_sum, gfx.iter().collect())
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
    let (signal_strength, gfx) = execute(&program);
    println!("{}", signal_strength);
    println!("{}", gfx);
}
