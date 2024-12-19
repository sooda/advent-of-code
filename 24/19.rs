use std::io::{self, Read};
use std::collections::HashMap;

fn possible<'a>(towels: &[String], pattern: &'a str, mem: &mut HashMap<&'a str, usize>) -> usize {
    if let Some(&x) = mem.get(&pattern) {
        x
    } else {
        let n = if pattern.len() == 0 {
            1
        } else {
            towels.iter()
                .filter(|&t| pattern.starts_with(t))
                .map(|t| possible(towels, &pattern[t.len()..], mem))
                .sum()
        };
        mem.insert(pattern, n);
        n
    }
}

fn possible_designs(towels: &[String], patterns: &[String]) -> usize {
    patterns.iter().filter(|pattern| possible(towels, &pattern, &mut HashMap::new()) > 0).count()
}

fn possible_design_ways(towels: &[String], patterns: &[String]) -> usize {
    patterns.iter().map(|pattern| possible(towels, &pattern, &mut HashMap::new())).sum()
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
    println!("{}", possible_design_ways(&towels, &patterns));
}
