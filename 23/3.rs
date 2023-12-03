use std::io::{self, BufRead};
use std::collections::HashMap;

extern crate regex;
use regex::Regex;

fn symbol(ch: u8) -> bool {
    ch != b'.' && !(ch >= b'0' && ch <= b'9')
}

fn gear(ch: u8) -> bool {
    ch == b'*'
}

type GearMap = HashMap<(usize, usize), Vec<u32>>;

fn line_sum(inp: &[String], i: usize, gear_map: &mut GearMap) -> u32 {
    let re = Regex::new(r"\d+").unwrap();
    let mut sum = 0;
    for found in re.find_iter(&inp[i]) {
        let value: u32 = found.as_str().parse().unwrap();
        let mut append_gear = |y, x| gear_map.entry((y, x)).or_insert(Vec::new()).push(value);
        let (a, b) = (found.start(), found.end() - 1);
        let front = a > 0 && symbol(inp[i].as_bytes()[a-1]);
        let back = b < inp[i].len() - 1 && symbol(inp[i].as_bytes()[b+1]);
        let gear_front = a > 0 && gear(inp[i].as_bytes()[a-1]);
        let gear_back = b < inp[i].len() - 1 && gear(inp[i].as_bytes()[b+1]);
        if gear_front {
            append_gear(i, a-1);
        }
        if gear_back {
            append_gear(i, b+1);
        }
        let a = (a.max(1)) - 1;
        let b = (b.min(inp[i].len()-2)) + 2;
        let above = i > 0 && (a..b).map(|j| inp[i-1].as_bytes()[j]).any(symbol);
        let beyond = i < inp.len() - 1 && (a..b).map(|j| inp[i+1].as_bytes()[j]).any(symbol);
        if i > 0 {
            (a..b)
                .filter(|&j| gear(inp[i-1].as_bytes()[j]))
                .for_each(|j| append_gear(i-1, j));
        }
        if i < inp.len() - 1 {
            (a..b)
                .filter(|&j| gear(inp[i+1].as_bytes()[j]))
                .for_each(|j| append_gear(i+1, j));
        }

        if above || beyond || front || back {
            sum += value;
        }

    }
    sum
}

fn part_number_sum(inp: &[String], gear_map: &mut GearMap) -> u32 {
    (0..inp.len()).map(|i| line_sum(inp, i, gear_map)).sum()
}

fn gear_ratios(map: &GearMap) -> u32 {
    map.iter().filter(|(_, v)| v.len() == 2).map(|(_, v)| v[0] * v[1]).sum()
}

fn main() {
    let lines: Vec<String> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect();
    let mut map = HashMap::new();
    println!("{}", part_number_sum(&lines, &mut map));
    println!("{}", gear_ratios(&map));
}

