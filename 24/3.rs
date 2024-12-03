use std::io::{self, Read};

extern crate regex;
use regex::Regex;

fn multiplications(program: &str, use_do: bool) -> i32 {
    let re = Regex::new(r"(?:(mul)\((\d+),(\d+)\))|((d)(o)\(\))|((d)(o)n't\(\))").unwrap();
    let mut total_sum = 0;
    let mut enabled = true;
    for (_, [op, a, b]) in re.captures_iter(program).map(|c| c.extract()) {
        match op {
            "do()" => enabled = true,
            "don't()" => enabled = false || !use_do,
            "mul" if enabled => total_sum += a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap(),
            "mul" if !enabled => (),
            _ => panic!("unhandled op")
        }
    }
    total_sum
}

fn main() {
    let mut file = String::new();
    io::stdin().read_to_string(&mut file).unwrap();
    println!("{}", multiplications(&file, false));
    println!("{}", multiplications(&file, true));
}
