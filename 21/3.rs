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
}
