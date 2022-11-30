use std::fs::File;
use std::io::Read;

// rustc -L foo/deps 8.rs
extern crate regex;
use regex::Regex;

fn readfile(name: &str) -> String {
    let mut f = File::open(name).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    s
}

fn update(input: &str, mut screen: [u64; 6]) -> [u64; 6] {
    println!("{}", input);
    let re_rect = Regex::new(r"rect (\d+)x(\d)").unwrap();
    let re_rrow = Regex::new(r"rotate row y=(\d) by (\d+)").unwrap();
    let re_rcol = Regex::new(r"rotate column x=(\d+) by (\d+)").unwrap();
    if let Some(cap) = re_rect.captures(input) {
        let w = cap.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let h = cap.get(2).unwrap().as_str().parse::<usize>().unwrap();
        for y in 0..h {
            screen[y] |= (1u64 << w) - 1;
        }
    } else if let Some(cap) = re_rrow.captures(input) {
        let row = cap.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let by = cap.get(2).unwrap().as_str().parse::<u32>().unwrap();
        assert!(by < 50);

        let orig = screen[row];
        // note: shifts are to the other direction than in the instructions: lowest bit is in the
        // left, so the screen is mirrored internally.
        //
        // left part
        screen[row] = (screen[row] << by) & (1 << 50) - 1;
        // right part
        screen[row] |= orig >> (50 - by);
    } else if let Some(cap) = re_rcol.captures(input) {
        let col = cap.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let by = cap.get(2).unwrap().as_str().parse::<usize>().unwrap();
        assert!(by < 6);

        let on = 1 << col;
        let off = !on;
        let orig = screen.clone();
        for y in 0usize..6 {
            screen[y] = (screen[y] & off) | (orig[(y + 6 - by) % 6] & on);
        }
    }

    for row in screen.iter() {
        for i in 0..50 {
            if row & (1u64 << i) != 0 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }

    screen
}

fn main() {
    let src = readfile(&std::env::args().nth(1).unwrap());
    let mut screen = [0u64; 6];
    for row in src.trim().split("\n") {
        screen = update(row, screen);
    }
    let ones: u32 = screen.iter().map(|x| x.count_ones()).sum();
    println!("{}", ones);
}


