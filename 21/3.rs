#![feature(iter_partition_in_place)]

use std::io::{self, BufRead};
use std::ops::{BitAnd, Shl};
use std::cmp::PartialEq;

// no idea what I am doing
fn one_bits<T>(numbers: &[T], bitpos: T) -> usize
where T: BitAnd<Output = T> + Shl<Output = T> + PartialEq + From<u8> + Copy {
    numbers.iter().filter(|&&x| (x & (T::from(1u8) << bitpos)) != 0u8.into()).count()
}

fn power_consumption(diagnostic_report: &[u32], length: usize) -> u32 {
    let n = diagnostic_report.len();
    let mut gamma_rate = 0;
    for bitpos in 0..(length as u32) {
        let ones = one_bits(diagnostic_report, bitpos);
        if ones > n / 2 {
            gamma_rate |= 1 << bitpos;
        }
    }
    let mask = (1 << length) - 1;
    let epsilon_rate = !gamma_rate & mask;
    gamma_rate * epsilon_rate
}

fn rating(mut diagnostic_report: &mut [u32], length: usize, bit_criteria: u32) -> u32 {
    for bitpos in (0..length as u32).rev() {
        let ones = one_bits(diagnostic_report, bitpos);
        let n_remaining = diagnostic_report.len();
        let desired_bit = if (ones >= n_remaining - ones) == (bit_criteria == 1) {
            1 << bitpos
        } else {
            0
        };
        let split = diagnostic_report.iter_mut()
            .partition_in_place(|&n| n & (1 << bitpos) == desired_bit);
        if split == 1 {
            return diagnostic_report[0];
        }
        diagnostic_report = &mut diagnostic_report[..split];
    }
    panic!("did not reduce")
}

fn life_support(mut diagnostic_report: &mut [u32], length: usize) -> u32 {
    let oxygen_generator_rating = rating(&mut diagnostic_report, length, 1);
    // note: can &mut again because the order of elements in the report does not matter
    let co2_scrubber_rating = rating(&mut diagnostic_report, length, 0);
    oxygen_generator_rating * co2_scrubber_rating
}

fn main() {
    let mut length: usize = 0;
    let mut diagnostic_report: Vec<u32> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .map(|line| {
            length = line.len();
            u32::from_str_radix(&line, 2).unwrap()
        })
        .collect();
    println!("{}", power_consumption(&diagnostic_report, length));
    println!("{}", life_support(&mut diagnostic_report, length));
}
