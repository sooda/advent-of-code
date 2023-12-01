use std::io::{self, BufRead};

fn calibration_value(calories_spec: &str) -> i32 {
    let first_digit = (calories_spec.bytes().find(|&x| x >= b'0' && x <= b'9').unwrap() - b'0') as i32;
    let last_digit = (calories_spec.bytes().rev().find(|&x| x >= b'0' && x <= b'9').unwrap() - b'0') as i32;
    first_digit * 10 + last_digit
}

fn calibration_sum(calories_spec: &[String]) -> i32 {
    calories_spec.iter().map(|x| calibration_value(x)).sum()
}

fn main() {
    let lines: Vec<String> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect();
    println!("{}", calibration_sum(&lines));
}
