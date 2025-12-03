use std::io::{self, BufRead};

fn max_joltage(bank: &[i32]) -> i32 {
    bank.iter().enumerate()
        .flat_map(|(i, &a)| bank.iter().skip(i + 1).map(move |&b| (a, b)))
        .map(|(a, b)| 10 * a + b)
        .max()
        .unwrap()
}

fn total_joltage(ratings: &[Vec<i32>]) -> i32 {
    ratings.iter()
        .map(|r| max_joltage(r))
        .sum()
}

fn main() {
    let ratings: Vec<Vec<i32>> = io::stdin().lock().lines()
        .map(|line| line.unwrap().bytes().map(|b| (b - b'0').into()).collect())
        .collect();
    println!("{}", total_joltage(&ratings));
}
