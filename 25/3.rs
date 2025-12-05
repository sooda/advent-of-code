use std::io::{self, BufRead};
use std::collections::HashMap;

fn find_max<'a>(bank: &'a [u8], remaining: usize, mem: &mut HashMap<(&'a [u8], usize), i64>) -> Option<i64> {
    if remaining > bank.len() {
        // cannot satisfy
        None
    } else if remaining == 0 {
        // ok, nothing to add to call site
        Some(0)
    } else if let Some(&m) = mem.get(&(bank, remaining)) {
        Some(m)
    } else {
        let use_digit = 10i64.pow(remaining as u32 - 1) * i64::from(bank[0]) + (1..bank.len())
            .flat_map(|i| find_max(&bank[i..], remaining - 1, mem))
            .max()
            .unwrap_or(0);
        let skip_digit = (1..bank.len())
            .flat_map(|i| find_max(&bank[i..], remaining, mem))
            .max()
            .unwrap_or(0);
        let m = use_digit.max(skip_digit);
        mem.insert((bank, remaining), m);
        Some(m)
    }
}

fn max_joltage(bank: &[u8], batteries: usize) -> i64 {
    find_max(bank, batteries, &mut HashMap::new()).unwrap()
}

fn total_joltage(ratings: &[Vec<u8>], batteries: usize) -> i64 {
    ratings.iter()
        .map(|r| max_joltage(r, batteries))
        .sum()
}

fn main() {
    let ratings: Vec<Vec<u8>> = io::stdin().lock().lines()
        .map(|line| line.unwrap().bytes().map(|b| b - b'0').collect())
        .collect();
    println!("{}", total_joltage(&ratings, 2));
    println!("{}", total_joltage(&ratings, 12));
}
