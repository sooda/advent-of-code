use std::io::{self, BufRead};

fn move_cost(crabs: &[i32], destination: i32) -> i32 {
    crabs.iter().map(|c| (c - destination).abs()).sum()
}

fn min_fuel(crabs: &[i32]) -> i32 {
    let minx: i32 = crabs.iter().copied().min().unwrap();
    let maxx: i32 = crabs.iter().copied().max().unwrap();
    (minx..=maxx).map(|pos| move_cost(crabs, pos)).min().unwrap()
}

fn main() {
    let input: Vec<i32> = io::stdin().lock().lines()
        .next().unwrap().unwrap().split(',')
        .map(|n| n.parse().unwrap())
        .collect();
    println!("{}", min_fuel(&input));
}
