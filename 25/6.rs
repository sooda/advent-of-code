use std::io::{self, BufRead};

fn grand_total_human(input: &[String]) -> i64 {
    let input = input.iter().map(|line| {
        line.split_whitespace()
            .collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    let operators = input.last().unwrap();
    operators.iter()
        .enumerate()
        .map(|(i, op)| {
            let inp = input.iter()
                .take(input.len() - 1)
                .map(|row| row[i].parse::<i64>().unwrap());
            match op as &str {
                "+" => inp.sum(),
                "*" => inp.fold(1, |acc, x| acc * x),
                _ => panic!("odd operator"),
            }
        })
    .sum()
}

fn grand_total_cephalopod(input: &[String]) -> i64 {
    let cols = input[0].len();
    let operators = input.last().unwrap().as_bytes();
    let mut tot = 0;
    let mut sum = 0;
    let mut prod = 1;
    for col in (0..cols).rev() {
        let numb = input.iter()
            .take(input.len() - 1)
            .map(|i| i.as_bytes()[col] as char)
            .collect::<String>();
        if numb.trim() == "" {
            sum = 0;
            prod = 1;
        } else {
            let num = numb.trim().parse::<i64>().unwrap();
            sum += num;
            prod *= num;
            match operators[col] as char {
                '+' => tot += sum,
                '*' => tot += prod,
                ' ' => {},
                _ => panic!("bad operator"),
            };
        }
    }
    tot
}

fn main() {
    let input = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect::<Vec<_>>();
    println!("{}", grand_total_human(&input));
    println!("{}", grand_total_cephalopod(&input));
}
