use std::io::{self, BufRead};

fn deepenings<I, V>(it: I) -> usize
where I: Iterator<Item = V> + Clone, V: std::cmp::PartialOrd {
    it.clone().zip(it.skip(1))
        .filter(|(prev, next)| next > prev)
        .count()
}

fn increase_count(depths: &[u32]) -> usize {
    deepenings(depths.iter())
}

fn increase_count_windowed(depths: &[u32]) -> usize {
    deepenings(depths.windows(3).map(|w| w[0] + w[1] + w[2]))
}

fn main() {
    let sonar_report: Vec<u32> = io::stdin().lock().lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();
    println!("{}", increase_count(&sonar_report));
    println!("{}", increase_count_windowed(&sonar_report));
}
