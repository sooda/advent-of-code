use std::io::{self, BufRead};

fn find_next(history: &[i32]) -> i32 {
    if history.iter().all(|&x| x == 0) {
        0
    } else {
        let deltas = history.iter()
            .zip(history.iter().skip(1))
            .map(|(x0, x1)| x1 - x0)
            .collect::<Vec<i32>>();
        history.last().unwrap() + find_next(&deltas)
    }
}

fn next_sum(histories: &[Vec<i32>]) -> i32 {
    histories.iter().map(|h| find_next(h)).sum::<i32>()
}

fn parse_numbers(line: &str) -> Vec<i32> {
    line.split(' ').map(|x| x.parse().unwrap()).collect()
}

fn main() {
    let histories: Vec<Vec<i32>> = io::stdin().lock().lines()
        .map(|line| parse_numbers(&line.unwrap()))
        .collect();
    println!("{}", next_sum(&histories));
}
