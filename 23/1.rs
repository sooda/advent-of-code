use std::io::{self, BufRead};

extern crate regex;
use regex::Regex;

fn digitize(x: &str) -> i32 {
    match x {
        "0" | "zero" => 0,
        "1" | "one" => 1,
        "2" | "two" => 2,
        "3" | "three" => 3,
        "4" | "four" => 4,
        "5" | "five" => 5,
        "6" | "six" => 6,
        "7" | "seven" => 7,
        "8" | "eight" => 8,
        "9" | "nine" => 9,
        _ => panic!("bad digit"),
    }
}

fn calibration_value_letters(calories_spec: &str) -> i32 {
    let re_first = Regex::new(r"([0-9]|zero|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let re_last = Regex::new(r".*([0-9]|zero|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let first = re_first.captures(calories_spec).unwrap().get(1).unwrap();
    let last = re_last.captures(calories_spec).unwrap().get(1).unwrap();
    let first_digit = digitize(first.as_str());
    let last_digit = digitize(last.as_str());
    first_digit * 10 + last_digit
}

fn calibration_sum_letters(calories_spec: &[String]) -> i32 {
    calories_spec.iter().map(|x| calibration_value_letters(x)).sum()
}

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
    println!("{}", calibration_sum_letters(&lines));
}
