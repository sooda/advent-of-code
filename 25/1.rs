use std::io::{self, BufRead};

fn total_clicks(instructions: &[i32]) -> usize {
    let mut dial = 50;
    let mut zeros = 0;
    for &step in instructions {
        for _ in 0..step.abs() {
            dial = (dial + step.signum() + 1000) % 100;
            if dial == 0 {
                zeros += 1;
            }
        }
    }
    zeros
}

fn total_zeros(instructions: &[i32]) -> usize {
    let mut dial = 50;
    let mut zeros = 0;
    for step in instructions {
        dial = (dial + step + 1000) % 100;
        if dial == 0 {
            zeros += 1;
        }
    }
    zeros
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
