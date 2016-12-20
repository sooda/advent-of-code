use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

// rules gets generated into a sorted (by start) list of possibly-overlapping blacklist ranges
fn parseline(input: &str, rules: &mut Vec<(u32, u32)>) {
    let mut sp = input.split("-");
    let start = sp.next().unwrap().parse::<u32>().unwrap();
    let end = sp.next().unwrap().parse::<u32>().unwrap();

    let first_larger = rules.iter().position(|&(start_i, _)| start < start_i);
    if let Some(first_larger) = first_larger {
        rules.insert(first_larger, (start, end));
    } else {
        rules.push((start, end));
    }
}

// merge overlapping ranges
fn filter(rules: &mut Vec<(u32, u32)>) {
    let mut into = 0;
    while into < rules.len() - 1 {
        // next's start less than or equal to our end? overlap, merge
        if rules[into + 1].0 - 1 <= rules[into].1 {
            if rules[into + 1].1 <= rules[into].1 {
                // next is completely inside this, so delete it
                rules.remove(into + 1);
            } else {
                // merge these two and delete the next
                rules[into].1 = rules[into + 1].1;
                rules.remove(into + 1);
            }
            // into doesn't advance
        } else {
            // all good, move on
            into += 1;
        }
    }
}

fn first_allowed(rules: &[(u32, u32)]) -> u32 {
    if rules[0].0 != 0 {
        // not gonna happen, lol.
        0
    } else {
        rules[0].1 + 1
    }
}

fn num_allowed(rules: &[(u32, u32)], total: u64) -> u64 {
    let denied = rules.iter().map(|&(start, end)| end as u64 - start as u64 + 1).sum::<u64>();
    total - denied
}

fn main() {
    let input = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap()).lines().map(Result::unwrap);
    let mut rules = Vec::new();
    for line in input {
        parseline(&line, &mut rules);
    }
    filter(&mut rules);
    println!("{}", first_allowed(&rules));
    println!("{}", num_allowed(&rules, 10));
    println!("{}", num_allowed(&rules, std::u32::MAX as u64 + 1));
}
