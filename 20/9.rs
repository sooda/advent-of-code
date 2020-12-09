use std::io::{self, BufRead};

fn no_sum_of_two(input: &[u64], n: usize, i: usize) -> bool {
    let x = input[i];
    // j=0..n, plus start offset
    for (m, (j, &y)) in input.iter().enumerate().skip(i - n).take(n).enumerate() {
        // m=0: k=1..n; m=1: k=2..n, etc.
        for (k, &z) in input.iter().enumerate().skip(j + 1).take(n - m) {
            if x == y + z && j != k {
                return false;
            }
        }
    }
    true
}
fn first_with_bad_property(input: &[u64], n: usize) -> Option<u64> {
    for (i, &x) in input.iter().enumerate().skip(n) {
        if no_sum_of_two(input, n, i) {
            return Some(x);
        }
    }
    None
}

fn main() {
    let numbers: Vec<u64> = io::stdin().lock().lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();
    println!("{:?}", first_with_bad_property(&numbers, 5));
    println!("{:?}", first_with_bad_property(&numbers, 25));
}
