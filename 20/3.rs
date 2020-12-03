use std::io::{self, BufRead};

const TOBOGGAN_SLOPE: usize = 3;
const TREE: u8 = b'#';

type Map = Vec<Vec<u8>>;

fn tree_pattern(map: &Map) -> usize {
    let w = map[0].len();
    let mut x = 0;
    let mut trees_encountered = 0;
    for row in map {
        if row[x % w] == TREE {
            trees_encountered += 1;
        }
        x += TOBOGGAN_SLOPE;
    }
    trees_encountered
}

fn main() {
    let map: Map = io::stdin().lock().lines()
        .map(|line| line.unwrap().into_bytes())
        .collect();
    println!("{}", tree_pattern(&map));
}
