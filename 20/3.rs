use std::io::{self, BufRead};

const TREE: u8 = b'#';

type Map = Vec<Vec<u8>>;

fn tree_pattern(map: &Map, x_slope: usize, y_slope: usize) -> usize {
    let w = map[0].len();
    let mut x = 0;
    let mut trees_encountered = 0;
    for row in map.iter().step_by(y_slope) {
        if row[x % w] == TREE {
            trees_encountered += 1;
        }
        x += x_slope;
    }
    trees_encountered
}

fn main() {
    let map: Map = io::stdin().lock().lines()
        .map(|line| line.unwrap().into_bytes())
        .collect();
    println!("{}", tree_pattern(&map, 3, 1));
    let slopes = &[
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2)
    ];
    let tree_product = slopes.iter()
        .fold(1, |prod, &(x, y)| prod * tree_pattern(&map, x, y));
    println!("{}", tree_product);
}
