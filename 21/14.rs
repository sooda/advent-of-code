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

    count_map.values().max().unwrap() - count_map.values().min().unwrap()
}

fn expanse(pair_counts: HashMap<(char, char), usize>, rules: &HashMap<(char, char), char>) -> HashMap<(char, char), usize> {
    let mut output = HashMap::new();
    for (pair, count) in pair_counts.into_iter() {
        if let Some(&polymerization) = rules.get(&pair) {
            // NN -> NC + CB
            *output.entry((pair.0, polymerization)).or_insert(0) += count;
            *output.entry((polymerization, pair.1)).or_insert(0) += count;
        } else {
            // just NN -> NN
            *output.entry(pair).or_insert(0) += count;
        }
    }
    output
}

fn result_quantity_code_fast(polymer: Vec<char>, rules: &HashMap<(char, char), char>, repetitions: usize) -> usize {
    let mut pair_counts: HashMap<(char, char), usize> = polymer
        .iter().copied()
        .zip(polymer.iter().copied().skip(1))
        .fold(HashMap::new(), |mut map, pair| {
            *map.entry(pair).or_insert(0) += 1;
            map
        });

    for _ in 0..repetitions {
        pair_counts = expanse(pair_counts, rules);
    }

    let mut count_map = pair_counts.iter().fold(HashMap::new(), |mut map, ((a, b), count)| {
        *map.entry(a).or_insert(0) += count;
        *map.entry(b).or_insert(0) += count;
        map
    });
    // NCNBCHB would be NC, CN, NB, BC, CH, HB
    // each middle element gets counted twice, so compensate the ends (that exist at this point) to be consistent
    *count_map.get_mut(polymer.first().unwrap()).unwrap() += 1;
    *count_map.get_mut(polymer.last().unwrap()).unwrap() += 1;

    (count_map.values().max().unwrap() - count_map.values().min().unwrap()) / 2
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
    println!("{:?}", result_quantity_code(template.clone(), &rules, 10));
    println!("{:?}", result_quantity_code_fast(template.clone(), &rules, 10));
    println!("{:?}", result_quantity_code_fast(template, &rules, 40));
}
