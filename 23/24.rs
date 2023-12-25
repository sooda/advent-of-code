use std::io::{self, BufRead};

extern crate regex;
use regex::Regex;

type Coord = (f64, f64, f64);
type Hail = (Coord, Coord);

fn sum(a: Coord, b: Coord) -> Coord {
    (a.0 + b.0, a.1 + b.1, a.2 + b.2)
}

fn diff(a: Coord, b: Coord) -> Coord {
    (a.0 - b.0, a.1 - b.1, a.2 - b.2)
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
    // scary business to compare this many floats precisely without any epsilons
    if den == 0.0 {
        if num_t == 0.0 && num_u == 0.0 {
            Some((0.0, 0.0))
        } else {
            None
        }
    } else {
        Some((num_t / den, num_u / den))
    }
}

fn pair_intersecting(h1: Hail, h2: Hail, min_bound: f64, max_bound: f64) -> bool {
    if let Some((t, u)) = intersect_xy(h1, h2) {
        if t >= 0.0 && u >= 0.0 {
            let p1 = sum(h1.0, mulf(t, h1.1));

            let p2 = sum(h2.0, mulf(u, h2.1));
            if false {
                println!("t u {:?}  p1 {:?}  p2 {:?}", (t, u), p1, p2);
            }

            p1.0 >= min_bound && p1.0 <= max_bound && p1.1 >= min_bound && p1.1 <= max_bound
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

// only in xy plane
fn find_perfect(storm: &[Hail]) -> Coord {
    let pairs_count = storm.len() * (storm.len() - 1) / 2;
    for dy in 1..300 {
        for dx in 1..300 {
            let rock = (dx as f64, dy as f64, 0.0);
            let storm_rock_view = storm.iter()
                .map(|h| (h.0, diff(h.1, rock)))
                .collect::<Vec<Hail>>();
            let hits = intersecting(&storm_rock_view, 0.0, 1e50);
            if hits == pairs_count {
                return rock;
            }
        }
    }
    panic!("not found!");
}

fn perfect_rock(storm: &[Hail]) -> f64 {
    // r_p + t*r_v == h_p + t*h_v for every h for some t > 0
    // also, all the hails are intersecting sometimes from the rock's frame of reference r_v
    let rock_vel = find_perfect(storm);
    let storm_rock_view = storm.iter()
        .map(|h| (h.0, diff(h.1, rock_vel)))
        .collect::<Vec<Hail>>();
    let (t, u) = intersect_xy(storm_rock_view[0], storm_rock_view[1]).unwrap();

    // r_p + t*r_v = a_p + t*a_v | r_p = a_p + t*a_v - t*r_v
    // r_p + u*r_v = b_p + u*b_v | r_p = b_p + u*b_v - u*r_v
    // a_p + t*a_v - t*r_v = b_p + u*b_v - u*r_v
    // t*r_v - u*r_v = a_p + t*a_v - b_p - u*b_v
    // (t-u)*r_v = a_p + t*a_v - b_p - u*b_v
    let a_p = sum(storm[0].0, mulf(t, storm[0].1));
    let b_p = sum(storm[1].0, mulf(u, storm[1].1));
    let r_v = mulf(1.0 / (t - u), diff(a_p, b_p));
    assert!((r_v.0 - rock_vel.0).abs() < 1.0);
    assert!((r_v.1 - rock_vel.1).abs() < 1.0);
    // r_p + t*r_v = a_p + t*a_v | r_p = a_p + t*a_v - t*r_v
    let rockpos = diff(a_p, mulf(t, r_v));
    rockpos.0 + rockpos.1 + rockpos.2
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
    println!("{}", perfect_rock(&storm));
}
