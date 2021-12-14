use std::io::{self, BufRead};
use std::collections::HashMap;

fn naive_expanse(polymer: Vec<char>, rules: &HashMap<(char, char), char>) -> Vec<char> {
    let mut output = Vec::new();
    output.push(*polymer.first().unwrap());
    for (&a, &b) in polymer.iter().zip(polymer.iter().skip(1)) {
        if let Some(&polymerization) = rules.get(&(a, b)) {
            output.push(polymerization);
        }
        output.push(b);
    }
    output
}

fn result_quantity_code(mut polymer: Vec<char>, rules: &HashMap<(char, char), char>, repetitions: usize) -> usize {
    for _ in 0..repetitions {
        polymer = naive_expanse(polymer, rules);
    }

    let count_map = polymer.iter().fold(HashMap::new(), |mut map, ch| {
        *map.entry(ch).or_insert(0) += 1;
        map
    });
    let mut counts: Vec<usize> = count_map.values().copied().collect();
    counts.sort_unstable();

    counts.last().unwrap() - counts.first().unwrap()
}

fn parse_poly(input: &[String]) -> (Vec<char>, HashMap<(char, char), char>) {
    let mut sp = input.split(|l| l == "");
    let template = sp.next().unwrap()[0].to_owned();
    let rules: Vec<_> = sp.next().unwrap().iter().map(|r| {
        let mut rsp = r.split(" -> ");
        let mut pair = rsp.next().unwrap().chars();
        let mut insertion = rsp.next().unwrap().chars();
        ((pair.next().unwrap(), pair.next().unwrap()), insertion.next().unwrap())
    }).collect();

    (template.chars().collect(), rules.into_iter().collect())
}

fn main() {
    let input: Vec<_> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect();
    let (template, rules) = parse_poly(&input);
    println!("{:?}", result_quantity_code(template, &rules, 10));
}
