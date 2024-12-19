use std::io::{self, Read};
use std::collections::HashMap;

fn possible<'a>(towels: &[String], pattern: &'a str, mem: &mut HashMap<&'a str, bool>) -> bool {
    if let Some(&x) = mem.get(&pattern) {
        return x;
    }
    if pattern.len() == 0 {
        mem.insert(pattern, true);
        return true;
    }
    for t in towels {
        if pattern.starts_with(t) && possible(towels, &pattern[t.len()..], mem) {
            mem.insert(pattern, true);
            return true;
        }
    }
    mem.insert(pattern, false);
    false
}

fn possible_designs(towels: &[String], patterns: &[String]) -> usize {
    patterns.iter().filter(|pattern| possible(towels, &pattern, &mut HashMap::new())).count()
}

fn parse(file: &str) -> (Vec<String>, Vec<String>) {
    /*
     * r, wr, b, g, bwu, rb, gb, br
     *
     * brwrr
     * bggr
     */
    let mut sp = file.split("\n\n");
    let towels = sp.next().unwrap()
        .split(", ")
        .map(|t| t.to_string())
        .collect();
    let patterns = sp.next().unwrap()
        .lines().map(|l| l.to_string()).collect();
    (towels, patterns)
}

fn main() {
    let mut file = String::new();
    io::stdin().read_to_string(&mut file).unwrap();
    let (towels, patterns) = parse(&file);

    println!("{}", possible_designs(&towels, &patterns));
}

