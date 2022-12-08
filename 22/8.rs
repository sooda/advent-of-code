use std::io::{self, BufRead};

type Trees = [Vec<u8>];

fn visible(x: usize, y: usize, tree: u8, trees: &Trees) -> bool {
    let h = trees.len();
    let w = trees[0].len();
    let left_blocking = (0..x).any(|xi| trees[y][xi] >= tree);
    let top_blocking = (0..y).any(|yi| trees[yi][x] >= tree);
    let right_blocking = (x+1..w).any(|xi| trees[y][xi] >= tree);
    let bottom_blocking = (y+1..h).any(|yi| trees[yi][x] >= tree);

    !(left_blocking && top_blocking && right_blocking && bottom_blocking)
}

fn visible_outside(trees: &Trees) -> usize {
    trees.iter().enumerate().flat_map(|(y, row)| {
        row.iter().enumerate().filter(move |(x, tree)| visible(*x, y, **tree, trees))
    }).count()
}

fn main() {
    let trees: Vec<_> = io::stdin().lock().lines()
        .map(|line| line.unwrap().as_bytes().to_vec())
        .collect();
    println!("{}", visible_outside(&trees));
}

