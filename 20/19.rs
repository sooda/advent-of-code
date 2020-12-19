use std::io::{self, BufRead};

extern crate regex;
use regex::Regex;

#[derive(Debug, Clone)]
enum Rule {
    Single(char),
    Sequence(Vec<usize>),
    SequenceChoice(Vec<Vec<usize>>),
    RepeatingSequence(Vec<usize>),
}
use Rule::*;

// this can eat a subsequence that's not necessarily any complete rule
fn process_sequence<'a>(rules: &[Rule], sequence: &[usize], message: &'a [u8]) -> Vec<&'a [u8]> {
    let after_first = do_match(rules, sequence[0], message);
    if sequence.len() == 1 {
        after_first
    } else {
        let mut out = Vec::new();
        for next_message in after_first {
            out.append(&mut process_sequence(rules, &sequence[1..], next_message));
        }
        out
    }
}

fn process_sequencechoice<'a>(rules: &[Rule], sequences: &[Vec<usize>], message: &'a [u8]) -> Vec<&'a [u8]> {
    let mut out = Vec::new();
    for sequence in sequences {
        out.append(&mut process_sequence(rules, sequence, message));
    }
    out
}

fn do_match<'a>(rules: &[Rule], current: usize, message: &'a [u8]) -> Vec<&'a [u8]> {
    match &rules[current] {
        &Single(ch) => {
            if message.get(0).map(|&b| b as char) == Some(ch) {
                vec![&message[1..]]
            } else {
                Vec::new()
            }
        }
        Sequence(sequence) => {
            process_sequence(rules, sequence, message)
        },
        SequenceChoice(sequences) => {
            process_sequencechoice(rules, sequences, message)
        },
        RepeatingSequence(_) => {
            // not used here, only for regexes. Could emulate with a self-reference pair though
            panic!()
        }
    }
}

fn message_matches(rules: &[Rule], message: &str) -> bool {
    let remaining_msgs = do_match(rules, 0, message.as_bytes());
    // processed so much that the next submessage after this rule is nothing?
    remaining_msgs.contains(&(&[] as &[u8]))
}

fn build_sequence(rules: &[Rule], seq: &[usize], re_str: &mut [String]) -> String {
    seq.iter().map(|&ri| {
        build_regex(rules, ri, re_str);
        // can't ref re_str[i] because re_str is &mut. so many clones. :(
        // appending to a string with fold would be an option but it reallocates too
        re_str[ri].clone()
    }).collect::<Vec<String>>()
    .join("")
}

fn build_regex(rules: &[Rule], current: usize, re_str: &mut [String]) {
    if re_str[current] != "" {
        return;
    }
    re_str[current] = match &rules[current] {
        Single(ch) => {
            ch.to_string()
        }
        Sequence(seq) => {
            build_sequence(rules, seq, re_str)
        },
        SequenceChoice(seq_choices) => {
            let red = seq_choices.iter()
                .map(|seq| build_sequence(rules, seq, re_str))
                .collect::<Vec<String>>()
                .join("|");
            String::from("(?:") + &red + ")"
        },
        RepeatingSequence(seq) => {
            let red = build_sequence(rules, seq, re_str);
            String::from("(?:") + &red + ")+"
        }
    }
}

fn build_total_regex(rules: &[Rule]) -> Regex {
    let mut re_str = vec![String::new(); rules.len()];
    build_regex(rules, 0, &mut re_str);
    Regex::new(&(String::from("^") + &re_str[0] + "$")).unwrap()
}

fn fix_rules(rules: &mut [Rule]) {
    // 8: 42
    // 8: 42 | 42 8 == 8: 42+
    if let Sequence(seq) = std::mem::replace(&mut rules[8], Single('x')) {
        // could also unroll this loop but this is more fun
        rules[8] = RepeatingSequence(seq);
    } else {
        panic!();
    }
    // 11: 42 31
    // 11: 42 31 | 42 11 31 = 11: 42 31 | 42 42 31 31 | 42 42 42 31 31 31 | ...
    if let Sequence(seq) = std::mem::replace(&mut rules[11], Single('x')) {
        assert!(seq.len() == 2);
        let first = seq[0];
        let second = seq[1];
        // no way it's going to loop more than this
        let options = (1..=10).map(|i| {
            let mut repetition = Vec::new();
            for _ in 0..i {
                repetition.push(first);
            }
            for _ in 0..i {
                repetition.push(second);
            }
            repetition
        }).collect::<Vec<Vec<usize>>>();
        rules[11] = SequenceChoice(options);
    } else {
        panic!();
    }
}

// this wouldn't work with the regexer because of the self-reference
fn fix_rules_native(rules: &mut [Rule]) {
    // 8: 42
    // 8: 42 | 42 8
    if let Sequence(seq) = std::mem::replace(&mut rules[8], Single('x')) {
        let mut rhs = seq.clone();
        rhs.push(8);
        rules[8] = SequenceChoice(vec![seq, rhs]);
    } else {
        panic!();
    }
    // 11: 42 31
    // 11: 42 31 | 42 11 31
    if let Sequence(seq) = std::mem::replace(&mut rules[11], Single('x')) {
        let mut rhs = seq.clone();
        rhs.insert(1, 11);
        rules[11] = SequenceChoice(vec![seq, rhs]);
    } else {
        panic!();
    }
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
        (id, SequenceChoice(vec![a, b]))
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

    // now that they exist, do both algorithms to cross-check
    let regexed = build_total_regex(&rules);
    let good_message_count = messages.iter().filter(|m| regexed.is_match(m)).count();
    println!("{}", good_message_count);
    let good_message_count = messages.iter().filter(|m| message_matches(&rules, m)).count();
    println!("{}", good_message_count);

    let mut rules_native = rules.clone();
    fix_rules(&mut rules);
    fix_rules_native(&mut rules_native);
    let regexed = build_total_regex(&rules);
    let good_message_count = messages.iter().filter(|m| regexed.is_match(m)).count();
    println!("{}", good_message_count);
    let good_message_count = messages.iter().filter(|m| message_matches(&rules_native, m)).count();
    println!("{}", good_message_count);
}
