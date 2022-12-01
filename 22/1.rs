use std::io::{self, BufRead};

fn max_cals(calories_spec: &[String]) -> usize {
    calories_spec.split(|cs| cs == "").map(|cals| {
        cals.iter().map(|c| c.parse::<usize>().unwrap()).sum()
    }).max().unwrap()
}

fn tot_top_three_cals(calories_spec: &[String]) -> usize {
    let mut tot_cals: Vec<usize> = calories_spec.split(|cs| cs == "").map(|cals| {
        cals.iter().map(|c| c.parse::<usize>().unwrap()).sum()
    }).collect();
    tot_cals.sort_unstable_by(|a, b| b.cmp(a));

    tot_cals[0] + tot_cals[1] + tot_cals[2]
}

fn main() {
    let calories_spec: Vec<String> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect();
    println!("{}", max_cals(&calories_spec));
    println!("{}", tot_top_three_cals(&calories_spec));
}
