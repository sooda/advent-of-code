use std::io::{self, BufRead};
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}
use Instruction::*;

#[derive(Debug)]
struct InstructionParseError;

impl FromStr for Instruction {
    type Err = InstructionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sp = s.split(' ');
        let op = sp.next().unwrap();
        let arg = sp.next().unwrap().parse::<i32>().unwrap();
        match op {
            "acc" => Ok(Acc(arg)),
            "jmp" => Ok(Jmp(arg)),
            "nop" => Ok(Nop(arg)),
            _ => Err(InstructionParseError),
        }
    }
}

fn execute(program: &[Instruction]) -> i32 {
    let mut pc = 0;
    let mut accumulator = 0;
    let mut visited = HashSet::new();
    loop {
        if visited.contains(&pc) {
            return accumulator;
        }
        visited.insert(pc);

        match program[pc as usize] {
            Acc(n) => accumulator += n,
            Jmp(n) => {
                pc += n;
                continue;
            }
            Nop(_) => {},
        }
        pc += 1;
    }
}

fn main() {
    let program: Vec<Instruction> = io::stdin().lock().lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();
    println!("{}", execute(&program));
}
