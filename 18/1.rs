use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashSet;

fn calibrate(diffs: &[i32]) -> i32 {
    let mut history = HashSet::new();
    let mut current_freq = 0;

    loop {
        for diff in diffs {
            history.insert(current_freq);
            current_freq += diff;

            if history.contains(&current_freq) {
                return current_freq;
            }
        }
    }
}

fn main() {
    let parsed_diffs = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap())
        .lines().map(|x| x.unwrap().parse::<i32>().unwrap()).collect::<Vec<_>>();

    let x = parsed_diffs.iter().fold(0, |acc, x| acc + x);
    println!("{}", x);

    println!("{}", calibrate(&parsed_diffs));
}
