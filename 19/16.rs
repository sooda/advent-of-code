use std::io::{self, BufRead};
use std::iter;

fn fft_once(digits: &[i32]) -> Vec<i32> {
    let n = digits.len();
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let pattern0 = iter::repeat(0).take(i + 1);
        let pattern1 = iter::repeat(1).take(i + 1);
        let pattern2 = iter::repeat(0).take(i + 1);
        let pattern3 = iter::repeat(-1).take(i + 1);
        let pattern = pattern0.chain(pattern1).chain(pattern2).chain(pattern3).cycle().skip(1);
        let out_digit = digits.iter().zip(pattern).map(|(&d, pat)| d * pat).sum::<i32>();
        out.push(out_digit.abs() % 10);
    }
    out
}

fn fft_100(mut digits: Vec<i32>) -> Vec<i32> {
    for _ in 0..100 {
        digits = fft_once(&digits);
    }
    digits
}

fn main() {
    let digits: Vec<i32> = io::stdin().lock().lines().next().unwrap().unwrap()
        .bytes().map(|b| (b - b'0') as i32).collect();

    let first_8 = &fft_100(digits.clone())[0..8];
    let as_string = first_8.into_iter().map(|&i| ((i as u8) + b'0') as char).collect::<String>();
    println!("{}", as_string);
}
