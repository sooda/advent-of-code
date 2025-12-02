use std::io::{self, BufRead};
use std::iter::repeat;

fn total_clicks(instructions: &[i32]) -> usize {
    instructions.iter()
        .flat_map(|step| repeat(step.signum()).take(step.abs() as usize))
        .fold((50, 0), |(dial, zeros), step| {
            let next = (dial + step + 1000) % 100;
            (next, zeros + (if next == 0 { 1 } else { 0 }))
        }).1
}

fn total_zeros(instructions: &[i32]) -> usize {
    instructions.iter()
        .fold((50, 0), |(dial, zeros), step| {
            let next = (dial + step + 1000) % 100;
            (next, zeros + (if next == 0 { 1 } else { 0 }))
        }).1
}

fn parse(inp: &str) -> i32 {
    let (dir, step) = inp.split_at(1);
    let step = step.parse().unwrap();
    match dir {
        "R" => step,
        "L" => -step,
        _ => panic!("bad input"),
    }
}

fn main() {
    let instructions: Vec<i32> = io::stdin().lock().lines()
        .map(|line| parse(&line.unwrap()))
        .collect();
    println!("{}", total_zeros(&instructions));
    println!("{}", total_clicks(&instructions));
}
