use std::io::{self, Read};

extern crate regex;
use regex::Regex;

fn multiplications(program: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut total_sum = 0;
    for (_, [a, b]) in re.captures_iter(program).map(|c| c.extract()) {
        total_sum += a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap();
    }
    total_sum
}

fn main() {
    let mut file = String::new();
    io::stdin().read_to_string(&mut file).unwrap();
    println!("{}", multiplications(&file));
}

