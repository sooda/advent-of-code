use std::io::{self, BufRead};

fn bsp(input: &str, one: char) -> usize {
    // could also replace the chars with 0 and 1 and parse that...
    input.chars().fold(0, |acc, x| acc * 2 + if x == one { 1 } else { 0 })
}

fn seat_id(boarding_pass: &str) -> usize {
    let row_spec = &boarding_pass[0..7];
    let col_spec = &boarding_pass[7..10];
    let row = bsp(row_spec, 'B');
    let col = bsp(col_spec, 'R');
    // the whole pass could be parsed as a single int because the seat id is formed just so, but
    // maybe part b needs these as separate numbers? so let's be more specific
    row * 8 + col
}

fn main() {
    let boarding_passes: Vec<_> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect();
    // why no coercion with map(seat_id) :(
    let max_seat_id = boarding_passes.iter().map(|p| seat_id(p)).max().unwrap();
    println!("{}", max_seat_id);

    let mut ids: Vec<_> = boarding_passes.iter().map(|p| seat_id(p)).collect();
    ids.sort();
    for pair in ids.windows(2) {
        // note: our place is never in the front or back by definition
        if pair[1] != pair[0] + 1 {
            println!("{}", pair[0] + 1);
            // there should be only one, but continue looping to double check
        }
    }
}
