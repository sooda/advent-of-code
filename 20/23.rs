use std::io::{self, BufRead};

// note!! 1-based indexing
fn from_labeling(labeling: &str) -> Vec<u8> {
    labeling.as_bytes().iter().map(|b| b - b'1').collect()
}

fn to_labeling(cups: &[u8]) -> String {
    cups.iter().map(|c| (c + b'1') as char).collect()
}

fn simulate(labeling: &str, n: usize) -> String {
    // current cup always in the front
    let mut cups = from_labeling(labeling);
    let ncups = 9 as u8;
    assert!(cups.len() == ncups as usize);

    let find_destination = |mut value: u8, cups: &[u8]| {
        loop {
            // current minus one, wrapping
            value = (value + ncups - 1) % ncups;
            // exists in the remaining cups?
            if let Some(i) = cups.iter().position(|&c| c == value) {
                return i;
            }
        }
    };
    for _move in 1..=n {
        let current = cups[0];
        let pickup = (cups.remove(1), cups.remove(1), cups.remove(1));
        let dest_idx = 1 + find_destination(current, &cups[1..]);
        cups.insert(dest_idx + 1, pickup.2);
        cups.insert(dest_idx + 1, pickup.1);
        cups.insert(dest_idx + 1, pickup.0);
        cups.rotate_left(1);
    }

    let one = cups.iter().position(|&c| c == 0).unwrap();
    cups.rotate_left(one);
    to_labeling(&cups[1..])
}

fn main() {
    let input: String = io::stdin().lock().lines()
        .map(|line| line.unwrap().parse().unwrap())
        .next().unwrap();
    println!("{}", simulate(&input, 100));
}
