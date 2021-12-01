use std::io::{self, BufRead};

fn increase_count(depths: &[u32]) -> usize {
    depths.iter().zip(depths.iter().skip(1))
        .filter(|(prev, next)| next > prev)
        .count()
}

fn main() {
    let sonar_report: Vec<u32> = io::stdin().lock().lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();
    println!("{}", increase_count(&sonar_report));
}
