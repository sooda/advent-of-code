use std::io::{self, BufRead};

fn safe(report: &[i32]) -> bool {
    let same_direction = report.iter()
        .zip(report.iter().skip(1))
        .map(|(a, b)| (a - b).signum())
        .sum::<i32>().abs() == (report.len() as i32 - 1);
    let good_diffs = report.iter()
        .zip(report.iter().skip(1))
        .map(|(a, b)| (a - b).abs())
        .filter(|&n| n >= 1 && n <= 3)
        .count() == report.len() - 1;
    same_direction && good_diffs
}

fn safe_count(reports: &[Vec<i32>]) -> usize {
    reports.iter().filter(|r| safe(r)).count()
}

fn main() {
    let reports: Vec<Vec<i32>> = io::stdin().lock().lines()
        .map(|line| line.unwrap()
             .split(" ")
             .map(|x| x.parse::<i32>().unwrap()).collect()
            ).collect();
    println!("{}", safe_count(&reports));
}

