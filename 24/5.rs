use std::io::{self, Read};
use std::cmp::Ordering;

type Rule = (i32, i32);
type Update = Vec<i32>;

fn passes_rule(update: &Update, rule: &Rule) -> bool {
    for (i, &a) in update.iter().enumerate() {
        for &b in update.iter().skip(i) {
            if a == rule.1 && b == rule.0 {
                return false;
            }
        }
    }
    true
}

fn update_ok(update: &Update, rules: &[Rule]) -> bool {
    rules.iter().all(|r| passes_rule(update, r))
}

fn middles_of_corrects(rules: &[Rule], updates: &[Update]) -> i32 {
    updates.iter()
        .filter(|u| update_ok(u, rules))
        .map(|u| u[u.len() / 2])
        .sum()
}

fn repair(mut update: Update, rules: &[Rule]) -> Update {
    update.sort_by(|&a, &b| {
        if rules.iter().any(|r| a == r.0 && b == r.1) {
            Ordering::Less
        } else if rules.iter().any(|r| a == r.1 && b == r.0) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });
    update
}

fn middles_of_repaired(rules: &[Rule], updates: &[Update]) -> i32 {
    updates.iter()
        .filter(|u| !update_ok(u, rules))
        .map(|u| repair(u.clone(), rules))
        .map(|u| u[u.len() / 2])
        .sum()
}

fn parse(file: &str) -> (Vec<Rule>, Vec<Update>) {
    let mut sp = file.split("\n\n");
    let rules = sp.next().unwrap()
        .lines()
        .map(|l| {
            let mut rsp = l.split('|');
            (rsp.next().unwrap().parse().unwrap(),
            rsp.next().unwrap().parse().unwrap())
        }).collect();
    let updates = sp.next().unwrap()
        .lines()
        .map(|l| l.split(',').map(|n| n.parse().unwrap()).collect())
        .collect();
    (rules, updates)
}

fn main() {
    let mut file = String::new();
    io::stdin().read_to_string(&mut file).unwrap();
    let (rules, updates) = parse(&file);
    println!("{:?}", middles_of_corrects(&rules, &updates));
    println!("{:?}", middles_of_repaired(&rules, &updates));
}
