use std::io::{self, BufRead};
use std::collections::HashMap;

extern crate regex;
use regex::Regex;

type Coord = (u32, u32, u32);
// begin, end coords; inclusive range by the input spec
type Brick = (Coord, Coord);

fn overlap(a: (u32, u32), b: (u32, u32)) -> Option<(u32, u32)> {
    let left = a.0.max(b.0);
    let right = a.1.min(b.1);
    if left <= right {
        Some((left, right))
    } else {
        None
    }
}

// some overlap in x,y plane and just touching in z
fn supports(below: Brick, above: Brick) -> bool {
    below.0.2.max(below.1.2) + 1 == above.0.2.min(above.1.2) &&
        overlap((below.0.0, below.1.0), (above.0.0, above.1.0)).is_some() &&
        overlap((below.0.1, below.1.1), (above.0.1, above.1.1)).is_some()
}

fn disintegratable(bricks: &[Brick]) -> usize {
    // (one below, many above it)
    let mut supp_map = HashMap::<Brick, Vec<Brick>>::new();
    // number of times supported
    let mut supp_by = HashMap::<Brick, usize>::new();
    for below in bricks {
        for above in bricks {
            if supports(*below, *above) {
                supp_map.entry(*below).or_insert(Vec::new()).push(*above);
                *supp_by.entry(*above).or_insert(0) += 1;
            }
        }
    }
    let mut ok = 0;
    for (_below, aboves) in &supp_map {
        if aboves.iter().all(|a| *supp_by.get(a).unwrap() > 1) {
            // not the only one supporting, so this can go
            ok += 1;
        }
    }
    for b in bricks {
        if supp_map.get(b) == None {
            // supports nothing, "leaf node"
            ok += 1;
        }
    }
    ok
}

// quick tetris mode
fn drop_brick(b: Brick, z: u32) -> Brick {
    ((b.0.0, b.0.1, z), (b.1.0, b.1.1, z + (b.1.2 - b.0.2)))
}

fn settle(mut bricks: Vec<Brick>) -> Vec<Brick> {
    let mut settled = Vec::<Brick>::new();
    bricks.sort_unstable_by_key(|a| a.0.2.min(a.1.2));
    'outer: for b in bricks {
        let z_min = b.0.2.min(b.1.2);
        let b_xrange = (b.0.0, b.1.0);
        let b_yrange = (b.0.1, b.1.1);
        for below_z in (1..z_min).rev() {
            if settled.iter().any(|s| {
                s.0.2.max(s.1.2) == below_z &&
                    overlap(b_xrange, (s.0.0, s.1.0)).is_some() &&
                    overlap(b_yrange, (s.0.1, s.1.1)).is_some()
            }) {
                settled.push(drop_brick(b, below_z + 1));
                continue 'outer;
            }
        }
        // no bricks found so it hit the floor
        // (and dug itself underground, no problem)
        settled.push(drop_brick(b, 1));
    }
    settled
}

fn parse_brick(line: &str) -> Brick {
    let re = Regex::new(r"(\d+),(\d+),(\d+)~(\d+),(\d+),(\d+)").unwrap();
    let cap = re.captures(line).unwrap();
    let g = |i| cap.get(i).unwrap().as_str().parse().unwrap();
    ((g(1), g(2), g(3)), (g(4), g(5), g(6)))
}

fn main() {
    let bricks = io::stdin().lock().lines()
        .map(|row| parse_brick(&row.unwrap()))
        .collect::<Vec<_>>();

    let bricks = settle(bricks);
    println!("{}", disintegratable(&bricks));
}
