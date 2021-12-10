use std::io::{self, BufRead};

fn syntax_error_score(input: &str) -> u64 {
    let mut stack = Vec::new();
    let mut illegal_sum = 0;
    for ch in input.chars() {
        if let Some(illegal_score) = match ch {
            '(' | '[' | '{' | '<' => {
                stack.push(ch);
                None
            },
            ')' => {
                if stack.pop() == Some('(') {
                    None
                } else {
                    Some(3)
                }
            },
            ']' => {
                if stack.pop() == Some('[') {
                    None
                } else {
                    Some(57)
                }
            },
            '}' => {
                if stack.pop() == Some('{') {
                    None
                } else {
                    Some(1197)
                }
            },
            '>' => {
                if stack.pop() == Some('<') {
                    None
                } else {
                    Some(25137)
                }
            },
            _ => panic!("bad line")
        } {
            illegal_sum += illegal_score;
        }
    }
    illegal_sum
}

fn total_syntax_error_score(input: &[String]) -> u64 {
    input.iter().map(|i| syntax_error_score(i)).sum()
}

fn main() {
    let lines: Vec<_> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect();
    println!("{}", total_syntax_error_score(&lines));
}

