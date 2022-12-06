use std::io::{self, BufRead};
use std::collections::HashSet;

fn marker_offset(datastream: &[u8], marker_len: usize) -> usize {
    datastream.windows(marker_len).position(|w| {
        w.iter().copied().collect::<HashSet<_>>().len() == marker_len
    }).unwrap() + marker_len
}

fn main() {
    let datastream: String = io::stdin().lock().lines().next().unwrap().unwrap();
    println!("{}", marker_offset(datastream.as_bytes(), 4));
    println!("{}", marker_offset(datastream.as_bytes(), 14));
}
