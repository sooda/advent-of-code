use std::io::{self, BufRead};

fn grand_total(input: &[Vec<String>]) -> i64 {
    let mut tot = 0;
    for (i, op) in input.last().unwrap().iter().enumerate() {
        let inp = input.iter()
            .take(input.len() - 1)
            .map(|row| row[i].parse::<i64>().unwrap());
        tot += match op as &str {
            "+" => inp.sum(),
            "*" => inp.fold(1, |acc, x| acc * x),
            _ => panic!("odd operator"),
        }
    }
    tot
}

fn main() {
    let input = io::stdin().lock().lines()
        .map(|line| line.unwrap()
             .split_whitespace()
             .map(|a| a.to_owned()).collect()
         )
        .collect::<Vec<_>>();
    println!("{}", grand_total(&input));
}
