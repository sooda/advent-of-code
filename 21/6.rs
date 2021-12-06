use std::io::{self, BufRead};

fn fish_population(school: &[u64], sim_length: usize) -> u64 {
    let mut current = [0u64; 9]; // count per age
    for &age in school {
        current[age as usize] += 1;
    }
    for _ in 0..sim_length {
        let mut next = [0u64; 9];
        for (i, &n) in current.iter().enumerate().skip(1) {
            next[i - 1] = n;
        }
        next[8] = current[0];
        next[6] += current[0];
        current = next;
    }
    current.iter().sum()
}

fn main() {
    let input: Vec<u64> = io::stdin().lock().lines()
        .next().unwrap().unwrap().split(',')
        .map(|n| n.parse().unwrap())
        .collect();
    println!("{}", fish_population(&input, 80));
    println!("{}", fish_population(&input, 256));
}
