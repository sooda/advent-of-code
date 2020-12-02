use std::io::{self, BufRead};

fn accounting(expenses: &[u32]) -> u32 {
    let pair = expenses.iter().enumerate()
        .flat_map(|(i, a)| expenses.iter().skip(i).map(move |b| (a, b)))
        .find(|&(x, y)| x + y == 2020).unwrap();
    pair.0 * pair.1
}

fn main() {
    let expense_report: Vec<u32> = io::stdin().lock().lines()
        .map(|cashline| cashline.unwrap().parse().unwrap())
        .collect();
    println!("{}", accounting(&expense_report));
}
