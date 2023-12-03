use std::io::{self, BufRead};

extern crate regex;
use regex::Regex;

fn symbol(ch: u8) -> bool {
    ch != b'.' && !(ch >= b'0' && ch <= b'9')
}

fn line_sum(inp: &[String], i: usize) -> u32 {
    let re = Regex::new(r"\d+").unwrap();
    let mut sum = 0;
    for found in re.find_iter(&inp[i]) {
        let value: u32 = found.as_str().parse().unwrap();
        let (a, b) = (found.start(), found.end() - 1);
        let front = a > 0 && symbol(inp[i].as_bytes()[a-1]);
        let back = b < inp[i].len() - 1 && symbol(inp[i].as_bytes()[b+1]);
        let a = (a.max(1)) - 1;
        let b = (b.min(inp[i].len()-2)) + 2;
        let above = i > 0 && (a..b).map(|j| inp[i-1].as_bytes()[j]).any(symbol);
        let beyond = i < inp.len() - 1 && (a..b).map(|j| inp[i+1].as_bytes()[j]).any(symbol);
        if above || beyond || front || back {
            sum += value;
        }
    }
    sum
}

fn part_number_sum(inp: &[String]) -> u32 {
    (0..inp.len()).map(|i| line_sum(inp, i)).sum()
}

fn main() {
    let lines: Vec<String> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect();
    println!("{}", part_number_sum(&lines));
}

