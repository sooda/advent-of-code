use std::io::{self, BufRead};

extern crate regex;
use regex::Regex;

type Coord = (f64, f64, f64);
type Hail = (Coord, Coord);

fn sum(a: Coord, b: Coord) -> Coord {
    (a.0 + b.0, a.1 + b.1, a.2 + b.2)
}

fn mulf(a: f64, b: Coord) -> (f64, f64, f64) {
    (a * b.0, a * b.1, a * b.2)
}

fn intersect_xy(a: Hail, b: Hail) -> Option<(f64, f64)> {
    let ((x1, y1, _), (x2, y2, _)) = a;
    let (x2, y2) = (x1 + x2, y1 + y2);
    let ((x3, y3, _), (x4, y4, _)) = b;
    let (x4, y4) = (x3 + x4, y3 + y4);
    let num_t = (x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4);
    let num_u = (x1 - x3) * (y1 - y2) - (y1 - y3) * (x1 - x2);
    let den   = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
    if den == 0.0 {
        None
    } else {
        Some((num_t / den, num_u / den))
    }
}

fn pair_intersecting(h1: Hail, h2: Hail, min_bound: f64, max_bound: f64) -> bool {
    if let Some((t, u)) = intersect_xy(h1, h2) {
        if t >= 0.0 && u >= 0.0 {
            let p = sum(h1.0, mulf(t, h1.1));
            p.0 >= min_bound && p.0 <= max_bound && p.1 >= min_bound && p.1 <= max_bound
        } else {
            false
        }
    } else {
        false
    }
}

fn intersecting(storm: &[Hail], min_bound: f64, max_bound: f64) -> usize {
    storm.iter()
        .enumerate()
        .map(|(i, &h1)| {
            storm.iter()
                .skip(i + 1)
                .filter(move |&&h2| pair_intersecting(h1, h2, min_bound, max_bound))
                .count()
        }).sum()
}

fn parse_hail(line: &str) -> Hail {
    // 19, 13, 30 @ -2,  1, -2
    let re = Regex::new(r"(-?\d+), +(-?\d+), +(-?\d+) @ +(-?\d+), +(-?\d+), +(-?\d+)").unwrap();
    let cap = re.captures(line).unwrap();
    let g = |i| cap.get(i).unwrap().as_str().parse().unwrap();
    ((g(1), g(2), g(3)), (g(4), g(5), g(6)))
}

fn main() {
    let storm = io::stdin().lock().lines()
        .map(|row| parse_hail(&row.unwrap()))
        .collect::<Vec<_>>();

    println!("{}", intersecting(&storm, 7.0, 27.0));
    println!("{}", intersecting(&storm, 200_000_000_000_000.0, 400_000_000_000_000.0));
}
