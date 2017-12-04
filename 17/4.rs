use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use std::collections::HashSet;
use std::iter::FromIterator;

fn valid_passphrase(line: &str) -> bool {
    let words_in_order = line.split(" ").collect::<Vec<_>>();
    let word_set: HashSet<&str> = HashSet::from_iter(words_in_order.iter().cloned());
    words_in_order.len() == word_set.len()
}

fn main() {
    assert!(valid_passphrase("aa bb cc dd ee"));
    assert!(!valid_passphrase("aa bb cc dd aa"));
    assert!(valid_passphrase("aa bb cc dd aaa"));
    let input_lines = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap())
        .lines().map(|x| x.unwrap()).collect::<Vec<_>>();
    let sum = input_lines.iter().map(|l| valid_passphrase(&l)).filter(|&x| x).count();
    println!("{}", sum);
}


