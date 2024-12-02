use std::io::{self, BufRead};

fn safe(report: &[i32]) -> bool {
    let mut pairs = report.iter()
        .zip(report.iter().skip(1))
        .map(|(a, b)| (a - b));
    let same_direction = pairs.clone()
        .map(|delta| delta.signum())
        .sum::<i32>().abs() == report.len() as i32 - 1;
    let good_diffs = pairs
        .all(|delta| delta.abs() >= 1 && delta.abs() <= 3);
    same_direction && good_diffs
}

fn safe_count(reports: &[Vec<i32>]) -> usize {
    reports.iter().filter(|r| safe(r)).count()
}

fn dampened_safe(report: &[i32]) -> bool {
    (0..report.len()).any(|i| {
        let mut r = report.to_vec();
        r.remove(i);
        safe(&r)
    })
}

fn safe_dampened_count(reports: &[Vec<i32>]) -> usize {
    reports.iter()
        .filter(|r| safe(r) || (r.len() > 1 && dampened_safe(r)))
        .count()
}

fn main() {
    let reports: Vec<Vec<i32>> = io::stdin().lock().lines()
        .map(|line| line.unwrap()
             .split(" ")
             .map(|x| x.parse::<i32>().unwrap()).collect()
            ).collect();
    println!("{}", safe_count(&reports));
    println!("{}", safe_dampened_count(&reports));
}
