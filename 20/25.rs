use std::io::{self, BufRead};

fn reverse_engineer_loops(pubkey: u64, divider: u64) -> u64 {
    let mut key = 1;
    for loopsz in 1u64.. {
        key = (key * 7) % divider;
        if key == pubkey {
            return loopsz;
        }
    }
    panic!()
}

fn transform(subject: u64, loops: u64, divider: u64) -> u64 {
    (0..loops).fold(1, |acc, _| (subject * acc) % divider)
}

fn solve(card_pubkey: u64, door_pubkey: u64) -> u64 {
    let divider = 20201227;
    let card_loops = reverse_engineer_loops(card_pubkey, divider);
    let door_loops = reverse_engineer_loops(door_pubkey, divider);
    let card_encryption_key = transform(card_pubkey, door_loops, divider);
    let door_encryption_key = transform(door_pubkey, card_loops, divider);
    assert!(card_encryption_key == door_encryption_key);
    card_encryption_key
}

fn main() {
    let card_pubkey: u64 = io::stdin().lock().lines().next().unwrap().unwrap().parse().unwrap();
    let door_pubkey: u64 = io::stdin().lock().lines().next().unwrap().unwrap().parse().unwrap();
    println!("{}", solve(card_pubkey, door_pubkey));
}
