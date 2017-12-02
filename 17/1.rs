use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn solve(input: &str) -> u32 {
    let a = input.chars();
    let b = input.chars().cycle().skip(1);
    a.zip(b).map(
        |(i, j)| if i == j { i as u32 - '0' as u32 } else { 0 }).sum()
}

fn main() {
    assert!(solve("1122") == 3);
    assert!(solve("1111") == 4);
    assert!(solve("1234") == 0);
    assert!(solve("91212129") == 9);
    let input = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap())
        .lines().next().unwrap().unwrap();
    println!("{}", solve(&input));
}
