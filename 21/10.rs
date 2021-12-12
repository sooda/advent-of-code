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

fn total_syntax_error_score(input: &[String]) -> u64 {
    input.iter().map(|i| syntax_error_score(i)).sum()
}

fn main() {
    let lines: Vec<_> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect();
    println!("{}", total_syntax_error_score(&lines));
}
