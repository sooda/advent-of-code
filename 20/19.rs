use std::io::{self, BufRead};

extern crate regex;
use regex::Regex;

#[derive(Debug, Clone)]
enum Rule {
    Single(char),
    Sequence(Vec<usize>),
    SequenceChoice(Vec<usize>, Vec<usize>),
}
use Rule::*;

fn build_sequence(rules: &[Rule], seq: &[usize], re_str: &mut [String]) -> String {
    let mut out = Vec::new();
    for &ri in seq {
        build_regex(rules, ri, re_str);
        // can't ref re_str[i] because re_str is &mut. so many clones. :(
        // appending to a string would be an option but it reallocates too
        out.push(re_str[ri].clone());
    }
    out.join("")
}

fn build_regex(rules: &[Rule], current: usize, re_str: &mut [String]) {
    if re_str[current] != "" {
        return;
    }
    match &rules[current] {
        Single(ch) => {
            re_str[current] = ch.to_string();
        }
        Sequence(seq) => {
            re_str[current] = build_sequence(rules, seq, re_str);
        },
        SequenceChoice(a, b) => {
            let a_str = build_sequence(rules, a, re_str);
            let b_str = build_sequence(rules, b, re_str);
            re_str[current] = format!("(?:{}|{})", a_str, b_str);
        },
    }
}

fn build_total_regex(rules: &[Rule]) -> Regex {
    let mut re_str = vec![String::new(); rules.len()];
    build_regex(rules, 0, &mut re_str);
    Regex::new(&(String::from("^") + &re_str[0] + "$")).unwrap()
}

// regex was a mistake, just look at this
fn parse_rule(input: &str) -> (usize, Rule) {
    // (should build these only once instead of per each input but parsing input isn't the point)
    let single_re = Regex::new(r#"^(\d+): "([a-z])"$"#).unwrap();
    // lol hack, there's at most three things
    let sequence_re = Regex::new(r#"^(\d+): (\d+)(?: (\d+))?(?: (\d+))?$"#).unwrap();
    let sequencechoice_re = Regex::new(r"^(\d+): (\d+)(?: (\d+))? \| (\d+)(?: (\d+))?$").unwrap();

    if let Some(cap) = single_re.captures(input) {
        let id = cap.get(1).unwrap().as_str().parse().unwrap();
        let b = cap.get(2).unwrap().as_str().chars().next().unwrap();
        (id, Single(b))
    } else if let Some(cap) = sequence_re.captures(input) {
        let id = cap.get(1).unwrap().as_str().parse().unwrap();
        let them = cap.iter().skip(2).filter_map(|num| num.map(|n| n.as_str().parse().unwrap())).collect();
        (id, Sequence(them))
    } else if let Some(cap) = sequencechoice_re.captures(input) {
        let id = cap.get(1).unwrap().as_str().parse().unwrap();
        let (mut a, mut b) = (Vec::new(), Vec::new());
        a.push(cap.get(2).unwrap().as_str().parse().unwrap());
        if let Some(n) = cap.get(3) {
            a.push(n.as_str().parse().unwrap());
        }
        b.push(cap.get(4).unwrap().as_str().parse().unwrap());
        if let Some(n) = cap.get(5) {
            b.push(n.as_str().parse().unwrap());
        }
        (id, SequenceChoice(a, b))
    } else {
        panic!("?? {}", input)
    }
}

fn main() {
    let input: Vec<_> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect();
    let mut parts = input.split(|x| x == "");
    let rules_s = parts.next().unwrap();
    let messages = parts.next().unwrap();

    // the indices seem to match the number of rules, so have vec instead of hashmap
    let mut rules = vec![Single('x'); rules_s.len()];
    for (i, r) in rules_s.into_iter().map(|r| parse_rule(r)) {
        rules[i] = r;
    }

    let regexed = build_total_regex(&rules);
    let good_message_count = messages.iter().filter(|m| regexed.is_match(m)).count();
    println!("{}", good_message_count);
}
