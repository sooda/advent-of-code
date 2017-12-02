use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn line_checksum(line: &str) -> u32 {
    let min = line.split("\t").map(|s| s.parse::<u32>().unwrap()).min().unwrap();
    let max = line.split("\t").map(|s| s.parse::<u32>().unwrap()).max().unwrap();
    max - min
}

fn main() {
    let input_lines = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap())
        .lines();
    let sum = input_lines.map(|l| line_checksum(&l.unwrap())).sum::<u32>();
    println!("{}", sum);
}
