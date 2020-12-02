use std::io::{self, BufRead};

extern crate regex;
use regex::Regex;

#[derive(Debug)]
struct CorporatePolicy {
    a: u32,
    b: u32,
    ch: char,
}

fn parse_pwline(input: &str) -> (CorporatePolicy, String) {
    let re = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
    let cap = re.captures(input).unwrap();
    let a = cap.get(1).unwrap().as_str().parse().unwrap();
    let b = cap.get(2).unwrap().as_str().parse().unwrap();
    let ch = cap.get(3).unwrap().as_str().chars().nth(0).unwrap();
    let pw = cap.get(4).unwrap().as_str();
    (CorporatePolicy { a, b, ch }, pw.to_owned())
}

fn validate_pw((policy, pw): &&(CorporatePolicy, String)) -> bool {
    let n = pw.chars().filter(|&ch| ch == policy.ch).count() as u32;
    n >= policy.a && n <= policy.b
}

fn main() {
    let database: Vec<_> = io::stdin().lock().lines()
        .map(|pwline| parse_pwline(&pwline.unwrap()))
        .collect();
    println!("{:?}", database.iter().filter(validate_pw).count());
}
