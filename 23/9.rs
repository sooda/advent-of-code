use std::io::{self, BufRead};

fn find_prev_next(history: &[i32]) -> (i32, i32) {
    if history.iter().all(|&x| x == 0) {
        (0, 0)
    } else {
        let deltas = history.iter()
            .zip(history.iter().skip(1))
            .map(|(x0, x1)| x1 - x0)
            .collect::<Vec<i32>>();
        let next = find_prev_next(&deltas);
        (history.first().unwrap() - next.0,
         history.last().unwrap() + next.1)
    }
}

fn prev_next_sum(histories: &[Vec<i32>]) -> (i32, i32) {
    histories.iter()
        .map(|h| find_prev_next(h))
        .fold((0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1))
}

fn parse_numbers(line: &str) -> Vec<i32> {
    line.split(' ').map(|x| x.parse().unwrap()).collect()
}

fn main() {
    let histories: Vec<Vec<i32>> = io::stdin().lock().lines()
        .map(|line| parse_numbers(&line.unwrap()))
        .collect();
    let pn = prev_next_sum(&histories);
    println!("{}", pn.1);
    println!("{}", pn.0);
}
