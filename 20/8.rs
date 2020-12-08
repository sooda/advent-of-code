use std::io::{self, BufRead};
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
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

fn execute(program: &[Instruction]) -> (i32, bool) {
    let mut pc = 0;
    let mut accumulator = 0;
    let mut visited = HashSet::new();
    while pc < program.len() as i32 {
        if visited.contains(&pc) {
            return (accumulator, false);
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
    (accumulator, true)
}

fn execute_fixed_program(program: &[Instruction]) -> i32 {
    for (i, &ins) in program.iter().enumerate() {
        if let Some(inverted) = match ins {
            Acc(_) => None,
            Jmp(n) => Some(Nop(n)),
            Nop(n) => Some(Jmp(n)),
        } {
            let mut attempt = program.to_vec();
            attempt[i] = inverted;
            let result = execute(&attempt);
            if result.1 {
                return result.0;
            }
        }
    }
    panic!()
}

fn main() {
    let program: Vec<Instruction> = io::stdin().lock().lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();
    println!("{}", execute(&program).0);
    println!("{}", execute_fixed_program(&program));
}
