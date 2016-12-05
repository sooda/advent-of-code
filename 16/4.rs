use std::fs::File;
use std::io::Read;

// rustc -L foo/deps 4.rs
extern crate regex;
use regex::Regex;

use std::collections::HashMap;
use std::cmp::Ordering;

fn readfile(name: &str) -> String {
    let mut f = File::open(name).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    s
}

// 0 for non-real rooms
fn sector_id(row: &str) -> u32 {
    // aaaaa-bbb-z-y-x-123[abxyz]
    let re = Regex::new(r"((?:[a-z]+-)*[a-z]+)+-(\d+)\[([a-z]+)\]").unwrap();
    let cap = re.captures(row).unwrap();
    let encrypted_name = cap.at(1).unwrap().chars().filter(|&x| x != '-').collect::<String>();
    let sector_id = cap.at(2).unwrap().parse::<u32>().unwrap();
    let checksum = cap.at(3).unwrap();
    let decrypted = encrypted_name.chars().map(
        |x| (
            ('a' as u32) + (
                ((x as u32) - ('a' as u32) + sector_id) % 26
                )
            ) as u8 as char).collect::<String>();

    // found this manually, ha
    if decrypted == "northpoleobjectstorage" {
        println!("{}", sector_id);
    }

    // first collect alphas into map
    let mut freqs = HashMap::new();
    for c in encrypted_name.chars() {
            *freqs.entry(c).or_insert(0) += 1;
    }

    // collect into vec to sort and stuff
    let mut freqs: Vec<_> = freqs.iter().collect();
    // .0 is ch, .1 is count
    // sort by count first, reversed, then by char, normally
    freqs.sort_by(|a, b|
                   match b.1.cmp(a.1) {
                       Ordering::Equal => a.0.cmp(b.0),
                       rest => rest
                   });

    // concat five most recent chars
    let most_common = freqs.iter().take(5).fold(String::new(),
            |mut sum, &(&c, _)| { sum.push(c); sum });
    let real_room = most_common == checksum;

    if real_room {
        sector_id
    } else {
        0
    }
}

fn main() {
    let src = readfile(&std::env::args().nth(1).unwrap());
    let sum = src.trim().split("\n").map(sector_id).sum::<u32>();
    println!("{}", sum);
}


