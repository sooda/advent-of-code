use std::io::{self, BufRead};
use std::collections::HashSet;

fn parse_list(s: &str) -> HashSet<u32> {
    // filter_map because "" isn't a number, so skip them
    s.split(' ').filter_map(|n| n.parse::<u32>().ok()).collect()
}

fn parse_for_points(card_spec: &str) -> u32 {
    let mut sp = card_spec.split(" | ");
    let left = sp.next().unwrap();
    let hand_str = sp.next().unwrap();
    let mut sp = left.split(": ");
    let winning_str = sp.nth(1).unwrap();
    let winning = parse_list(winning_str);
    let hand = parse_list(hand_str);

    let n = hand.intersection(&winning).count();

    if n == 0 { 0 } else { 1 << (n - 1) }
}

fn main() {
    let points: Vec<u32> = io::stdin().lock().lines()
        .map(|line| parse_for_points(&line.unwrap()))
        .collect();
    println!("{}", points.iter().sum::<u32>());
}

