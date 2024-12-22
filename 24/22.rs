use std::io::{self, BufRead};

fn simulate(mut secret: u64, n: usize) -> u64 {
    for _ in 0..n {
        secret = ((secret * 64) ^ secret) % 16777216;
        secret = ((secret / 32) ^ secret) % 16777216;
        secret = ((secret * 2048) ^ secret) % 16777216;
    }
    secret
}

fn secret_sums(secrets: &[u64], n: usize) -> u64 {
    secrets.iter().map(|&s| simulate(s, n)).sum()
}

fn main() {
    let secrets: Vec<_> = io::stdin().lock().lines()
        .map(|line| line.unwrap().parse::<u64>().unwrap())
        .collect();
    println!("{}", secret_sums(&secrets, 2000));
}
