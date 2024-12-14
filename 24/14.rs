use std::io::{self, BufRead};

extern crate regex;
use regex::Regex;

type Pos = (i32, i32);

fn add(a: Pos, b: Pos) -> Pos {
    (a.0 + b.0, a.1 + b.1)
}

fn mod_(a: Pos, b: Pos) -> Pos {
    (a.0 % b.0, a.1 % b.1)
}

type Robot = (Pos, Pos);

fn count(robots: &[Robot], min: Pos, max: Pos) -> usize {
    robots.iter()
        .filter(|(p, _)| p.0 >= min.0 && p.0 < max.0 && p.1 >= min.1 && p.1 < max.1)
        .count()
}

fn safety_factor(robots: &mut [Robot], w: i32, h: i32, n: usize) -> usize {
    let size = (w, h);
    for _ in 0..n {
        for r in &mut *robots {
            r.0 = add(r.0, r.1);
            r.0 = mod_(add(r.0, size), size);
        }
    }
    count(robots, (0, 0), (w/2, h/2)) *
        count(robots, (w/2+1, 0), (w, h/2)) *
        count(robots, (0, h/2+1), (w/2, h)) *
        count(robots, (w/2+1, h/2+1), (w, h))
}

fn parse(line: &str) -> Robot {
    // p=10,3 v=-1,2
    let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
    let cap = re.captures(line).unwrap();
    let get = |i| cap.get(i).unwrap().as_str().parse().unwrap();
    ((get(1), get(2)), (get(3), get(4)))
}

fn main() {
    let mut robots: Vec<_> = io::stdin().lock().lines()
        .map(|line| parse(&line.unwrap()))
        .collect();
    println!("{}", safety_factor(&mut robots.clone(), 11, 7, 100));
    println!("{}", safety_factor(&mut robots, 101, 103, 100));
}
