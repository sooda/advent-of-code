#![feature(hash_drain_filter)]
use std::io::{self, BufRead};
use std::collections::HashSet;

fn fold_along_x(dots: &mut HashSet<(i32, i32)>, flip_coord: i32) {
    let moving_part: HashSet<_> = dots.drain_filter(|d| d.0 >= flip_coord).collect();
    for d in moving_part {
        dots.insert((flip_coord - (d.0 - flip_coord), d.1));
    }
}

fn fold_along_y(dots: &mut HashSet<(i32, i32)>, flip_coord: i32) {
    let moving_part: HashSet<_> = dots.drain_filter(|d| d.1 >= flip_coord).collect();
    for d in moving_part {
        dots.insert((d.0, flip_coord - (d.1 - flip_coord)));
    }
}

fn fold(dots: &mut HashSet<(i32, i32)>, fold: (bool, i32)) {
    if fold.0 {
        fold_along_x(dots, fold.1);
    } else {
        fold_along_y(dots, fold.1);
    }
}

fn dots_after_one_fold(mut dots: HashSet<(i32, i32)>, folds: &[(bool, i32)]) -> usize {
    fold(&mut dots, folds[0]);
    dots.len()
}

fn parse_origami(paper: &[String]) -> (HashSet<(i32, i32)>, Vec<(bool, i32)>) {
    let mut sp = paper.split(|l| l == "");
    let dots = sp.next().unwrap().iter().map(|s| {
        let mut ssp = s.split(',');
        (ssp.next().unwrap().parse().unwrap(), ssp.next().unwrap().parse().unwrap())
    }).collect();
    let folds = sp.next().unwrap().iter().map(|f| {
        let mut fsp = f.split('=');
        let axis = fsp.next().unwrap() == "fold along x";
        let coord = fsp.next().unwrap().parse().unwrap();
        (axis, coord)
    }).collect();
    (dots, folds)
}

fn main() {
    let paper: Vec<_> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect();
    let (dots, folds) = parse_origami(&paper);
    println!("{:?}", dots_after_one_fold(dots, &folds));
}
