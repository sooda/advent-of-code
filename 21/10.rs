use std::io::{self, BufRead};

fn syntax_error_score(input: &str) -> u64 {
    let mut stack = Vec::new();
    for ch in input.chars() {
        match ch {
            '(' | '[' | '{' | '<' => {
                stack.push(ch);
            },
            ')' => {
                if stack.pop() != Some('(') {
                    return 3;
                }
            },
            ']' => {
                if stack.pop() != Some('[') {
                    return 57;
                }
            },
            '}' => {
                if stack.pop() != Some('{') {
                    return 1197;
                }
            },
            '>' => {
                if stack.pop() != Some('<') {
                    return 25137;
                }
            },
            _ => panic!("bad line")
        }
    }
    0
}

fn incompleteness_score(input: &str) -> u64 {
    let mut stack = Vec::new();
    for ch in input.chars() {
        match ch {
            '(' | '[' | '{' | '<' => {
                stack.push(ch);
            },
            ')' | ']' | '}' | '>' => {
                // guaranteed to match
                stack.pop().unwrap();
            }
            _ => panic!("bad line")
        };
    }
    let score = stack.iter().rev().fold(0, |score, ch| {
        5 * score + match ch {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => unreachable!("this was not pushed")
        }
    });
    score
}

fn total_syntax_error_score(input: &[String]) -> u64 {
    input.iter().map(|i| syntax_error_score(i)).sum()
}

fn incompleteness_winner_score(input: &[String]) -> u64 {
    let mut scores: Vec<_> = input.iter()
        .filter(|i| syntax_error_score(i) == 0)
        .map(|i| incompleteness_score(i))
        .collect();
    scores.sort_unstable();

    scores[scores.len() / 2]
}

fn main() {
    let lines: Vec<_> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect();
    println!("{}", total_syntax_error_score(&lines));
    println!("{}", incompleteness_winner_score(&lines));
}
