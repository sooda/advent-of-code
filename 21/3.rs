use std::io::{self, BufRead};

fn power_consumption(diagnostic_report: &[u32], length: usize) -> u32 {
    let n = diagnostic_report.len();
    let mut gamma_rate = 0;
    for bitpos in 0..length {
        let ones = diagnostic_report.iter().filter(|&&x| (x & (1 << bitpos)) != 0).count();
        if ones > n / 2 {
            gamma_rate |= 1 << bitpos;
        }
    }
    let mask = (1 << length) - 1;
    let epsilon_rate = !gamma_rate & mask;
    gamma_rate * epsilon_rate
}

fn rating(diagnostic_report: &[u32], length: usize, bit_criteria: u32) -> u32 {
    let mut desired_topbits = 0;
    let mut topbits_mask = 0;
    for bitpos in (0..length).rev() {
        let mut filtered_report = diagnostic_report.iter()
            .filter(move |&&x| (x & topbits_mask == desired_topbits));
        let n_remaining = filtered_report.clone().count();
        if n_remaining == 1 {
            // ?
            return *filtered_report.next().unwrap();
        }
        let ones = filtered_report.clone().filter(|&&x| (x & (1 << bitpos)) != 0).count();
        desired_topbits |= if bit_criteria == 1 {
            if ones >= n_remaining - ones {
                1 << bitpos
            } else {
                0
            }
        } else {
            if ones < n_remaining - ones {
                1 << bitpos
            } else {
                0
            }
        };
        topbits_mask |= 1 << bitpos;
        let mut filtered_report = diagnostic_report.iter()
            .filter(move |&&x| (x & topbits_mask == desired_topbits));
        if filtered_report.clone().count() == 1 {
            return *filtered_report.next().unwrap();
        }
    }
    panic!()
}

fn life_support(diagnostic_report: &[u32], length: usize) -> u32 {
    let len_mask = (1 << length) - 1;
    let oxygen_generator_rating = rating(diagnostic_report, length, 1);
    // todo: !
    let co2_scrubber_rating = rating(diagnostic_report, length, 0) & len_mask;
    oxygen_generator_rating * co2_scrubber_rating
}

fn main() {
    let mut length: usize = 0;
    let diagnostic_report: Vec<u32> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .map(|line| {
            length = line.len();
            u32::from_str_radix(&line, 2).unwrap()
        })
        .collect();
    println!("{}", power_consumption(&diagnostic_report, length));
    println!("{}", life_support(&diagnostic_report, length));
}
