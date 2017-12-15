use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn matches_in_pairs(a0: u64, b0: u64, num: usize) -> usize {
    let a_gen = (0..).scan(a0, |a, _| { *a = *a * 16807 % 2147483647; Some(*a) });
    let b_gen = (0..).scan(b0, |b, _| { *b = *b * 48271 % 2147483647; Some(*b) });
    a_gen.zip(b_gen).take(num).filter(|&(a, b)| a & 0xffff == b & 0xffff).count()
}

fn picky_matches_in_pairs(a0: u64, b0: u64, num: usize) -> usize {
    let a_gen = (0..).scan(a0, |a, _| { *a = *a * 16807 % 2147483647; Some(*a) }).filter(|a| a & 3 == 0);
    let b_gen = (0..).scan(b0, |b, _| { *b = *b * 48271 % 2147483647; Some(*b) }).filter(|b| b & 7 == 0);
    a_gen.zip(b_gen).take(num).filter(|&(a, b)| a & 0xffff == b & 0xffff).count()
}

fn main() {
    assert!(matches_in_pairs(65, 8921, 5) == 1);
    assert!(matches_in_pairs(65, 8921, 40*1000*1000) == 588);
    assert!(picky_matches_in_pairs(65, 8921, 5*1000*1000) == 309);

    let mut input = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap())
        .lines();
    // "Generator A starts with <number>"
    let a = input.next().unwrap().unwrap().split(" ").skip(4).next().unwrap().parse::<u64>().unwrap();
    let b = input.next().unwrap().unwrap().split(" ").skip(4).next().unwrap().parse::<u64>().unwrap();

    println!("{} {}", matches_in_pairs(a, b, 40*1000*1000), picky_matches_in_pairs(a, b, 5*1000*1000));
}
