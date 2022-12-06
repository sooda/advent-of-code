use std::io::{self, BufRead};
use std::collections::HashSet;

fn marker_offset(datastream: &[u8]) -> usize {
    datastream.windows(4).position(|w| {
        w.iter().copied().collect::<HashSet<_>>().len() == 4
    }).unwrap() + 4
}

fn main() {
    let datastream: String = io::stdin().lock().lines().next().unwrap().unwrap();
    println!("{}", marker_offset(datastream.as_bytes()));
}
