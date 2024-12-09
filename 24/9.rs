#![feature(let_chains)]

use std::io::{self, BufRead};

fn checksum(diskmap: &[u8]) -> usize {
    let mut disk = Vec::new();
    for (id2, &blocks) in diskmap.iter().enumerate() {
        if id2 & 1 == 0 {
            disk.extend(std::iter::repeat(Some(id2 / 2)).take(blocks as usize));
        } else {
            disk.extend(std::iter::repeat(None).take(blocks as usize));
        }
    }
    let mut pos = disk.iter().position(|data| data.is_none()).unwrap();
    loop {
        let last = disk.pop().unwrap();
        disk[pos] = last;

        if let Some(off) = disk[pos..].iter().position(|data| data.is_none()) {
            pos += off;
        } else {
            break;
        }
    }
    disk.iter().enumerate().map(|(i, id)| i * id.unwrap()).sum()
}

fn main() {
    let diskmap = io::stdin().lock().lines()
        .next().unwrap()
        .unwrap()
        .bytes()
        .map(|b| b - b'0')
        .collect::<Vec<_>>();
    println!("{}", checksum(&diskmap));
}
