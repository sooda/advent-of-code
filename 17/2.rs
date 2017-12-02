use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn line_checksum_a(line: &str) -> u32 {
    let min = line.split("\t").map(|s| s.parse::<u32>().unwrap()).min().unwrap();
    let max = line.split("\t").map(|s| s.parse::<u32>().unwrap()).max().unwrap();
    max - min
}

fn line_checksum_b(line: &str) -> u32 {
    let values = line.split("\t").map(|s| s.parse::<u32>().unwrap()).collect::<Vec<_>>();
    for i in &values {
        for j in &values {
            if i != j && i % j == 0 {
                return i / j;
            }
        }
    }
    unreachable!()
}

fn main() {
    let input_lines = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap())
        .lines().map(|x| x.unwrap()).collect::<Vec<_>>();
    let sum = input_lines.iter().map(|l| line_checksum_a(&l)).sum::<u32>();
    println!("{}", sum);
    let sum = input_lines.iter().map(|l| line_checksum_b(&l)).sum::<u32>();
    println!("{}", sum);
}
