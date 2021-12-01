use std::io::{self, BufRead};

fn increase_count(depths: &[u32]) -> usize {
    depths.iter().zip(depths.iter().skip(1))
        .filter(|(prev, next)| next > prev)
        .count()
}

fn increase_count_windowed(depths: &[u32]) -> usize {
    depths.windows(3).zip(depths.windows(3).skip(1))
        .filter(|(prev, next)| {
            next[0] + next[1] + next[2] > prev[0] + prev[1] + prev[2]
        })
        .count()
}

fn main() {
    let sonar_report: Vec<u32> = io::stdin().lock().lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();
    println!("{}", increase_count(&sonar_report));
    println!("{}", increase_count_windowed(&sonar_report));
}
