use std::io::{self, BufRead};
use std::iter::repeat;

type Tree = u8;
type Trees = [Vec<Tree>];

// is it better to have F copy or to pass around a ref to it?
fn visibility<F: Fn(&mut bool, Tree) -> Option<()> + Copy>(x: usize, y: usize, trees: &Trees, f: F)
-> (usize, usize, usize, usize) {
    let h = trees.len();
    let w = trees[0].len();

    // this just shortens copypasta, hopefully the compiler doesn't dyn all the things
    let compute = |it: &mut dyn Iterator<Item = (usize, usize)>| {
        it.map(|(xi, yi)| trees[yi][xi]).scan(false, f).count()
    };

    let left = compute(&mut (0..x).rev().zip(repeat(y)));
    let top = compute(&mut repeat(x).zip((0..y).rev()));
    let right = compute(&mut (x+1..w).zip(repeat(y)));
    let bottom = compute(&mut repeat(x).zip(y+1..h));

    (left, top, right, bottom)
}

fn visible(x: usize, y: usize, trees: &Trees) -> bool {
    let h = trees.len();
    let w = trees[0].len();
    let tree_at_origin = trees[y][x];
    // like take_while
    let f = |_: &mut bool, candidate: Tree| {
        if candidate >= tree_at_origin {
            None
        } else {
            Some(())
        }
    };
    let vis = visibility(x, y, trees, f);

    vis.0 == x || vis.1 == y || vis.2 == w - 1 - x || vis.3 == h - 1 - y
}

fn visible_outside(trees: &Trees) -> usize {
    trees.iter().enumerate().flat_map(|(y, row)| {
        row.iter().enumerate().filter(move |(x, _)| visible(*x, y, trees))
    }).count()
}

fn scenic_score(x: usize, y: usize, trees: &Trees) -> usize {
    let tree_at_origin = trees[y][x];
    // take_while would have off-by-1 issues
    let f = |tallest: &mut bool, candidate: Tree| {
        if *tallest {
            None
        } else {
            *tallest = candidate >= tree_at_origin;
            Some(())
        }
    };
    let vis = visibility(x, y, trees, f);

    vis.0 * vis.1 * vis.2 * vis.3
}

fn best_scenic_score(trees: &Trees) -> usize {
    trees.iter().enumerate().flat_map(|(y, row)| {
        row.iter().enumerate().map(move |(x, _)| scenic_score(x, y, trees))
    }).max().unwrap()
}

fn main() {
    let trees: Vec<_> = io::stdin().lock().lines()
        .map(|line| line.unwrap().as_bytes().to_vec())
        .collect();
    println!("{}", visible_outside(&trees));
    println!("{}", best_scenic_score(&trees));
}
