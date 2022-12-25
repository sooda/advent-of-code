use std::io::{self, BufRead};

fn to_snafu(mut num: i64) -> String {
    let mut chs: Vec<char> = Vec::new();
    loop {
        let mut digit = num % 5;
        if digit > 2 {
            digit -= 5;
            num += 5;
        }
        chs.push(match digit {
            2 => '2',
            1 => '1',
            0 => '0',
            -1 => '-',
            -2 => '=',
            _ => panic!("math error")
        });
        num /= 5;
        if num == 0 {
            break;
        }
    }

    chs.into_iter().rev().collect()
}

fn from_snafu(snafu: &str) -> i64 {
    let num = snafu.chars().rev().enumerate()
        .map(|(i, digit)| {
            let value = match digit {
                '2' => 2,
                '1' => 1,
                '0' => 0,
                '-' => -1,
                '=' => -2,
                _ => panic!("not a snafu")
            };
            value * 5i64.pow(i as u32)
        })
        .sum();
    assert_eq!(to_snafu(num), snafu);
    num
}

fn main() {
    let snafus: Vec<_> = io::stdin().lock().lines()
        .map(|line| from_snafu(&line.unwrap()))
        .collect();
    println!("{}", to_snafu(snafus.iter().sum()));
}
