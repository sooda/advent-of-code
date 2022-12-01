use std::io::{self, BufRead};

fn calorie_totals(calories_spec: &[String]) -> Vec<usize> {
    calories_spec.split(|cs| cs == "").map(|cals| {
        cals.iter().map(|c| c.parse::<usize>().unwrap()).sum()
    }).collect()
}

fn max_cals(cal_totals: &[usize]) -> usize {
    *cal_totals.iter().max().unwrap()
}

fn tot_top_three_cals(mut cal_totals: Vec<usize>) -> usize {
    cal_totals.sort_unstable_by(|a, b| b.cmp(a));

    cal_totals[0] + cal_totals[1] + cal_totals[2]
}

fn main() {
    let calories_spec: Vec<String> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect();
    let elf_totals = calorie_totals(&calories_spec);
    println!("{}", max_cals(&elf_totals));
    println!("{}", tot_top_three_cals(elf_totals));
}
