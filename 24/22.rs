#![feature(array_windows)]

use std::io::{self, BufRead};
use std::collections::{HashMap, HashSet};

fn simulate(mut secret: i64, n: usize) -> Vec<i64> {
    let mut output = Vec::new();
    output.push(secret);
    for _ in 0..n {
        secret = ((secret * 64) ^ secret) % 16777216;
        secret = ((secret / 32) ^ secret) % 16777216;
        secret = ((secret * 2048) ^ secret) % 16777216;
        output.push(secret);
    }
    output
}

fn secret_sums(secrets: &[i64]) -> i64 {
    secrets.iter().map(|&s| *simulate(s, 2000).last().unwrap()).sum()
}

fn most_bananas(secrets: &[i64]) -> i64 {
    let mut seq_scores = HashMap::<[i8; 4], i64>::new();
    for series in secrets.iter().map(|&s| simulate(s, 2000)) {
        let deltas = series.iter()
            .zip(series.iter().skip(1))
            .map(|(a, b)| (b % 10 - a % 10) as i8).collect::<Vec<_>>();

        let mut seen = HashSet::new();
        for (secret, diffwin) in series.iter().skip(4).zip(deltas.array_windows::<4>()) {
            if seen.insert(diffwin) {
                *seq_scores.entry(*diffwin).or_insert(0) += *secret % 10;
            }
        }
    }

    *seq_scores.values().max().unwrap()
}

fn main() {
    let secrets: Vec<_> = io::stdin().lock().lines()
        .map(|line| line.unwrap().parse::<i64>().unwrap())
        .collect();
    println!("{}", secret_sums(&secrets));
    println!("{}", most_bananas(&secrets));
}
