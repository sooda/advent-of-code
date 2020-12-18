use std::io::{self, BufRead};

fn calc(stream: &[u8], operator_precedence: bool) -> (u64, usize) {
    let mut pos = 0;
    // start with nothing plus something since the expr starts with a value and assumes things
    let mut result = 0u64;
    let mut current_op = b'+';
    let mut current_num = 0u64;
    let apply_op = |a, op, b| {
        match op {
            b'+' => a + b,
            b'*' => a * b,
            _ => panic!()
        }
    };
    while pos < stream.len() {
        let ch = stream[pos];
        match ch {
            b'0' ..= b'9' => {
                current_num = (current_num << 1) + (ch - b'0') as u64;
            },
            b' ' => {
                // this space means that a number or subexpression just ended, op already exists
                result = apply_op(result, current_op, current_num);
                current_num = 0;
            },
            b'*' if operator_precedence => {
                if stream.get(pos + 1) != Some(&b' ') {
                    panic!();
                }
                // with this simple trick (!) we'll have two operators of different precedence:
                // recurse into computing the rest of this expression before applying this one.
                let (subexpr, parselen) = calc(&stream[pos + 2..], operator_precedence);
                // (note: the length again shouldn't matter like at the end of this function
                // but keep it anyway for consistency)
                return (result * subexpr, pos + 2 + parselen);
            },
            b'+' | b'*' => {
                current_op = ch;
                pos += 1; // skip the trailing space because that's special for numbers only
                // that's an epic dirty hack so make sure (could also store current in Option<>)
                if stream.get(pos) != Some(&b' ') {
                    panic!();
                }
            },
            b'(' => {
                let (subexpr, parselen) = calc(&stream[pos + 1..], operator_precedence);
                // this isn't evaluated yet but interpreted as if a literal number was just found
                current_num = subexpr;
                pos += parselen;
                // dirty hack so make sure (this can be the last char in the stream though)
                if let Some(&ch) = stream.get(pos + 1) {
                    if ch != b' ' && ch != b')' {
                        panic!();
                    }
                }
            }
            b')' => {
                // end of recursion
                result = apply_op(result, current_op, current_num);
                return (result, pos + 1);
            },
            x => panic!("what is {}", x as char),
        }
        pos += 1;
    }
    result = apply_op(result, current_op, current_num);
    // note: the pos here shouldn't matter because the topmost caller does not use it and recursion
    // always stops when a ')' has been found
    (result, pos)
}

fn evaluate(expression: &str) -> u64 {
    calc(&expression.as_bytes(), false).0
}

fn evaluate2(expression: &str) -> u64 {
    calc(&expression.as_bytes(), true).0
}

fn main() {
    let expressions: Vec<String> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect();
    println!("{}", expressions.iter().map(|e| evaluate(e)).sum::<u64>());
    println!("{}", expressions.iter().map(|e| evaluate2(e)).sum::<u64>());
}
