use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

// rustc -L foo/deps 15.rs
extern crate regex;
use regex::Regex;

// 0 for non-real rooms
fn parse_disc(row: String) -> (u32, u32) {
    let re = Regex::new(r"Disc #. has (\d+) positions; at time=0, it is at position (\d+).").unwrap();
    let cap = re.captures(&row).unwrap();
    let positions = cap.at(1).unwrap().parse().unwrap();
    let startpos = cap.at(2).unwrap().parse().unwrap();

    (positions, startpos)
}

fn first_time(discs: &Vec<(u32, u32)>) -> u32 {
    let mut offs_sizes = discs.iter().enumerate().map(
        |(i, &(size, start))| ((start + (i as u32)) % size, size)
        ).collect::<Vec<_>>();
    println!("{:?}", offs_sizes);
    let mut i = 0;
    loop {
        let mut offs = 0;
        for mut disk in offs_sizes.iter_mut() {
            disk.0 = (disk.0 + 1) % disk.1;
            offs += disk.0;
        }
        if offs == 0 { break; }
        i += 1;
    }
    println!("{:?}", offs_sizes);
    i
}

fn main() {
    let input = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap()).lines().map(Result::unwrap);
    let discs = input.map(parse_disc).collect::<Vec<_>>();
    println!("{}", first_time(&discs));
}
