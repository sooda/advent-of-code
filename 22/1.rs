use std::io::{self, BufRead};

fn max_cals(calories_spec: &[String]) -> usize {
    calories_spec.split(|cs| cs == "").map(|cals| {
        cals.iter().map(|c| c.parse::<usize>().unwrap()).sum()
    }).max().unwrap()
}

fn main() {
    let calories_spec: Vec<String> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect();
    println!("{}", max_cals(&calories_spec));
}
