use std::io::{self, BufRead};

fn accounting(expenses: &[u32]) -> u32 {
    let pair = expenses.iter().enumerate()
        .flat_map(|(i, a)| expenses.iter().skip(i).map(move |&b| (a, b)))
        .find(|&(x, y)| x + y == 2020).unwrap();
    pair.0 * pair.1
}

fn three_entry_accounting(expenses: &[u32]) -> u32 {
    let pair = expenses.iter().enumerate()
        .flat_map(|(i, a)| expenses.iter().enumerate().skip(i).map(move |(j, &b)| (a, (j, b))))
        .flat_map(|(a, (j, b))| expenses.iter().skip(j).map(move |&c| (a, b, c)))
        .find(|&(x, y, z)| x + y + z == 2020).unwrap();
    pair.0 * pair.1 * pair.2
}

fn main() {
    let expense_report: Vec<u32> = io::stdin().lock().lines()
        .map(|cashline| cashline.unwrap().parse().unwrap())
        .collect();
    println!("{}", accounting(&expense_report));
    println!("{}", three_entry_accounting(&expense_report));
}
