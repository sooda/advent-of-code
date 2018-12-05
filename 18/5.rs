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
    let polymer = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap())
        .lines().next().unwrap().unwrap();
    let mut orig_reacted = polymer.clone();
    react(&mut orig_reacted);
    println!("{} {}", orig_reacted.len(), orig_reacted);

    println!("{:?}", (b'a'..=b'z').map(|c| {
        let mut mutant = polymer.replace(c as char, "").replace((c - 32) as char, "");
        react(&mut mutant);
        (mutant.len(), c as char)
    }).min().unwrap());
}
