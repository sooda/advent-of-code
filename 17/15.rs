use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn matches_in_pairs(mut a: u64, mut b: u64, num: usize) -> usize {
    let mut matches = 0;
    for _ in 0..num {
        a = a * 16807 % 2147483647;
        b = b * 48271 % 2147483647;
        if a & 0xffff == b & 0xffff {
            matches += 1;
        }
    }

    matches
}

fn main() {
    assert!(matches_in_pairs(65, 8921, 5) == 1);
    assert!(matches_in_pairs(65, 8921, 40*1000*1000) == 588);

    let mut input = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap())
        .lines();
    // "Generator A starts with <number>"
    let a = input.next().unwrap().unwrap().split(" ").skip(4).next().unwrap().parse::<u64>().unwrap();
    let b = input.next().unwrap().unwrap().split(" ").skip(4).next().unwrap().parse::<u64>().unwrap();

    println!("{}", matches_in_pairs(a, b, 40*1000*1000));
}
