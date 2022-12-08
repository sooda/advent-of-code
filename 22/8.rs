use std::io::{self, BufRead};
use std::iter::repeat;

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

fn scenic_score(x: usize, y: usize, tree: u8, trees: &Trees) -> usize {
    let h = trees.len();
    let w = trees[0].len();
    let scan_until_occluded = |tallest: &mut bool, (xi, yi): (usize, usize)| {
        if *tallest {
            None
        } else if trees[yi][xi] >= tree {
            *tallest = true;
            Some(())
        } else {
            Some(())
        }
    };
    let left_visible = (0..x).rev().zip(repeat(y)).scan(false, scan_until_occluded).count();
    let top_visible = repeat(x).zip((0..y).rev()).scan(false, scan_until_occluded).count();
    let right_visible = (x+1..w).zip(repeat(y)).scan(false, scan_until_occluded).count();
    let bottom_visible = repeat(x).zip(y+1..h).scan(false, scan_until_occluded).count();

    left_visible * top_visible * right_visible * bottom_visible
}

fn best_scenic_score(trees: &Trees) -> usize {
    trees.iter().enumerate().flat_map(|(y, row)| {
        row.iter().enumerate().map(move |(x, tree)| scenic_score(x, y, *tree, trees))
    }).max().unwrap()
}

fn main() {
    let trees: Vec<_> = io::stdin().lock().lines()
        .map(|line| line.unwrap().as_bytes().to_vec())
        .collect();
    println!("{}", visible_outside(&trees));
    println!("{}", best_scenic_score(&trees));
}

