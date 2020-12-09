use std::io::{self, BufRead};
use std::cmp::Ordering;

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

fn contiguous_sum(input: &[u64], x: u64) -> u64 {
    // With this we'd need a special case for the first element. Could also accumulate the sum of
    // above elements not including the loop elem, but then the last one would be special.
    let sum_upto: Vec<_> = input.iter().scan(0, |total, &elem| {
        *total += elem;
        Some(*total)
    }).collect();

    // this is the range we're looking for, probably won't start from the top. Could also
    // chain std::iter::once() with input.iter() above and adjust the loop.
    let mut a = 1;
    let mut b = 2;

    // This could be done without the allocation above but splitting the problem makes it a bit
    // easier to read, and the input is just 1000 things long.
    loop {
        // minus one to delete the stuff above this range, but don't touch the first of this range
        let range_sum = sum_upto[b] - sum_upto[a - 1];
        match range_sum.cmp(&x) {
            Ordering::Equal => {
                let range = &input[a..=b];
                return range.iter().min().unwrap() + range.iter().max().unwrap();
            },
            Ordering::Less => b += 1,
            Ordering::Greater => a += 1,
        }
    }
}

fn main() {
    let numbers: Vec<u64> = io::stdin().lock().lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();
    let a5 = first_with_bad_property(&numbers, 5);
    let a25 = first_with_bad_property(&numbers, 25);
    println!("{:?}", a5);
    println!("{:?}", a25);
    println!("{:?}", contiguous_sum(&numbers, a5.unwrap()));
    println!("{:?}", a25.map(|a25| contiguous_sum(&numbers, a25)));
}
