use std::io::{self, BufRead};

const DIGITS: usize = 6;

// could just keep dividing by 10, but indexing is nicer
fn digit(pw: u32, i: usize) -> u32 {
    let div = [100000, 10000, 1000, 100, 10, 1];
    pw / div[i] % 10
}

fn adjacent_digits(pw: u32) -> bool {
    for i in 0..DIGITS-1 {
        if digit(pw, i) == digit(pw, i + 1) {
            return true;
        }
    }

    false
}

fn never_decrease(pw: u32) -> bool {
    for i in 0..DIGITS-1 {
        if digit(pw, i + 1) < digit(pw, i) {
            return false;
        }
    }

    true
}

fn meets_criteria(pw: u32) -> bool {
    adjacent_digits(pw) && never_decrease(pw)
}

fn adjacent_digits_pair(pw: u32) -> bool {
    let mut len = 1;
    for i in 1..DIGITS {
        if digit(pw, i) == digit(pw, i - 1) {
            // same as before, keep counting
            len += 1;
        } else {
            if len == 2 {
                // changed to something else, so this is a pair
                return true;
            }
            len = 1;
        }
    }

    // can't change anymore if at the end, so check separately
    len == 2
}

fn meets_criteria_b(pw: u32) -> bool {
    adjacent_digits_pair(pw) && never_decrease(pw)
}

fn main() {
    assert!(digit(135679, 0) == 1);
    assert!(digit(135679, 1) == 3);
    assert!(digit(135679, 5) == 9);
    assert!(adjacent_digits(122345));
    assert!(never_decrease(111123));
    assert!(never_decrease(135679));

    assert!(meets_criteria(111111));
    assert!(!meets_criteria(223450));
    assert!(!meets_criteria(123789));

    let range: Vec<u32> = io::stdin().lock().lines().next().unwrap().unwrap().split('-')
        .map(|x| x.parse().unwrap())
        .collect();

    // the six-digit key fact is trivial because the puzzle input is contained
    let n = (range[0]..=range[1]).filter(|&pw| meets_criteria(pw)).count();
    println!("{}", n);
    let n = (range[0]..=range[1]).filter(|&pw| meets_criteria_b(pw)).count();
    println!("{}", n);
}
