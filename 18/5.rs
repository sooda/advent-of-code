use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn first_pair(polymer: &str) -> Option<usize> {
    let case_bit = 32;
    polymer.bytes().zip(polymer.bytes().skip(1)).position(|(a, b)| {
        ((a ^ b) == case_bit)
    })
}

fn react_once(polymer: &mut String) -> bool {
    if let Some(idx) = first_pair(polymer) {
        polymer.remove(idx);
        polymer.remove(idx);
        true
    } else {
        false
    }
}

fn react(polymer: &mut String) {
    while react_once(polymer) {
        // work it
    }
}

fn main() {
    let mut polymer = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap())
        .lines().next().unwrap().unwrap();
    react(&mut polymer);
    println!("{} {}", polymer.len(), polymer);
}
