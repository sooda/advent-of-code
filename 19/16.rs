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

fn fft_100_message(mut digits: Vec<i32>) -> String {
    for _ in 0..100 {
        digits = fft_once(&digits);
    }
    digits.into_iter().take(8).map(|d| (d as u8 + b'0') as char).collect::<String>()
}

/*
 * p: positive 1
 * n: negative 1
 * after n = len / 2 for big n, nth pattern is n+1 zeros and rest ones
 *    0 1 2 3 4 5 6 7 8 n
 * 0: 0 p 0 n 0 p 0 n 0
 * 1: 0 0 p p 0 0 n n 0
 * 2: 0 0 0 p p p 0 0 0
 * 3: 0 0 0 0 p p p p 0
 * 4: 0 0 0 0 0 p p p p
 * 5: 0 0 0 0 0 0 p p p
 * 6: 0 0 0 0 0 0 0 p p
 * 7: 0 0 0 0 0 0 0 0 p
 */
fn fft_100_message_real(digits: &[i32]) -> String {
    let offset = digits.iter().take(7).fold(0, |acc, x| acc * 10 + x) as usize;
    let mut signal = Vec::with_capacity(10000 * digits.len());

    for _ in 0..10000 {
        signal.extend_from_slice(digits);
    }

    let len = signal.len();
    assert!(offset >= len / 2);

    for _ in 0..100 {
        let mut next = vec![0; len];
        let mut acc = 0;
        // start from the end of the digit sequence, keep accumulating the sum in reverse
        for (digit, n) in signal.into_iter().zip(next.iter_mut()).rev().take(len / 2) {
            acc += digit;
            *n = acc % 10;
        }
        // 0..len/2 stay zeros

        signal = next;
    }
    let result_digits = signal.into_iter().skip(offset).take(8);
    result_digits.map(|d| (d as u8 + b'0') as char).collect::<String>()
}

fn main() {
    let digits: Vec<i32> = io::stdin().lock().lines().next().unwrap().unwrap()
        .bytes().map(|b| (b - b'0') as i32).collect();

    println!("{}", fft_100_message(digits.clone()));
    println!("{}", fft_100_message_real(&digits));
}
