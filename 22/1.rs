use std::io::{self, BufRead};

fn main() {
    let _: Vec<()> = io::stdin().lock().lines()
        .map(|line| println!("{}", line.unwrap()))
        .collect();
}
