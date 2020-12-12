use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
enum Action {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}
use Action::*;

#[derive(Debug, Copy, Clone)]
struct Instruction {
    action: Action,
    value: i32, // actually u32 but maybe we'll move much left
}

#[derive(Debug)]
struct InstructionParseError;

impl FromStr for Instruction {
    type Err = InstructionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (act, val) = s.split_at(1);
        let value = val.parse::<i32>().unwrap();
        let action = match act.chars().next().unwrap() {
            'N' => Ok(North),
            'S' => Ok(South),
            'E' => Ok(East),
            'W' => Ok(West),
            'L' => Ok(Left),
            'R' => Ok(Right),
            'F' => Ok(Forward),
            _ => Err(InstructionParseError),
        };
        action.map(|action| Instruction { action, value })
    }
}

fn left(dxdy: (i32, i32), value: i32) -> (i32, i32) {
    let (x, y) = dxdy;
    match value {
         90 => (-y,  x),
        180 => (-x, -y),
        270 => ( y, -x),
        _ => unreachable!()
    }
}

fn right(dxdy: (i32, i32), value: i32) -> (i32, i32) {
    left(dxdy, 360 - value)
}

fn execute(instructions: &[Instruction]) -> (i32, i32) {
    let mut x = 0; // grows east
    let mut y = 0; // grows north
    let mut dxdy = (1, 0);
    for step in instructions {
        match step.action {
            North => y += step.value,
            South => y -= step.value,
            East  => x += step.value,
            West  => x -= step.value,
            Left  => dxdy = left(dxdy, step.value),
            Right => dxdy = right(dxdy, step.value),
            Forward => { x += dxdy.0 * step.value; y += dxdy.1 * step.value; },
        }
    }
    (x, y)
}

fn main() {
    let program: Vec<Instruction> = io::stdin().lock().lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    let endpos = execute(&program);
    println!("{:?} {}", endpos, endpos.0.abs() + endpos.1.abs());
}
