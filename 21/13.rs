#![feature(hash_drain_filter)]
use std::io::{self, BufRead};
use std::collections::HashSet;

fn generic_fold<F, T>(dots: &mut HashSet<(i32, i32)>, flip_predicate: F, transform: T)
where
F: FnMut(&(i32, i32)) -> bool,
T: FnMut(&(i32, i32)) -> (i32, i32)
{
    let moving_part: HashSet<_> = dots.drain_filter(flip_predicate).collect();
    dots.extend(moving_part.iter().map(transform));
}

fn fold_along_x(dots: &mut HashSet<(i32, i32)>, flip_coord: i32) {
    generic_fold(dots,
                 |&(x, _)| x >= flip_coord,
                 |&(x, y)| (flip_coord - (x - flip_coord), y));
}

fn fold_along_y(dots: &mut HashSet<(i32, i32)>, flip_coord: i32) {
    generic_fold(dots,
                 |&(_, y)| y >= flip_coord,
                 |&(x, y)| (x, flip_coord - (y - flip_coord)));
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

fn display(dots: HashSet<(i32, i32)>) {
    let minx = dots.iter().map(|&d| d.0).min().unwrap();
    let maxx = dots.iter().map(|&d| d.0).max().unwrap();
    let miny = dots.iter().map(|&d| d.1).min().unwrap();
    let maxy = dots.iter().map(|&d| d.1).max().unwrap();
    for y in miny..=maxy {
        for x in minx..=maxx {
            let c = if dots.contains(&(x, y)) {
                '#'
            } else {
                ' '
            };
            print!("{}", c);
        }
        println!();
    }
}

fn fold_fully_and_display(mut dots: HashSet<(i32, i32)>, folds: &[(bool, i32)]) {
    for &f in folds {
        fold(&mut dots, f);
    }
    display(dots);
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
    println!("{:?}", dots_after_one_fold(dots.clone(), &folds));
    fold_fully_and_display(dots, &folds);
}
