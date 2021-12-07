use std::io::{self, BufRead};

fn arithmetic_seq(n: i32) -> i32 {
    // series goes from 1 to n
    let a = 1 + n;
    (a * a - a) / 2
}

fn move_cost(crabs: &[i32], destination: i32) -> i32 {
    crabs.iter().map(|c| (c - destination).abs()).sum()
}

fn min_fuel(crabs: &[i32]) -> i32 {
    let minx: i32 = crabs.iter().copied().min().unwrap();
    let maxx: i32 = crabs.iter().copied().max().unwrap();
    (minx..=maxx).map(|pos| move_cost(crabs, pos)).min().unwrap()
}

fn move_cost_arith(crabs: &[i32], destination: i32) -> i32 {
    crabs.iter().map(|c| arithmetic_seq((c - destination).abs())).sum()
}

fn min_fuel_arith(crabs: &[i32]) -> i32 {
    let minx: i32 = crabs.iter().copied().min().unwrap();
    let maxx: i32 = crabs.iter().copied().max().unwrap();
    (minx..=maxx).map(|pos| move_cost_arith(crabs, pos)).min().unwrap()
}

fn main() {
    let input: Vec<i32> = io::stdin().lock().lines()
        .next().unwrap().unwrap().split(',')
        .map(|n| n.parse().unwrap())
        .collect();
    println!("{}", min_fuel(&input));
    println!("{}", min_fuel_arith(&input));
}
