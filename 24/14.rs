use std::io::{self, BufRead};
use std::collections::HashSet;

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

fn dump(robots: &[Robot], size: (i32, i32)) {
    for y in 0..size.1 {
        for x in 0..size.0 {
            if robots.iter().any(|&(p, _)| p == (x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
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

fn fill(robots: &mut HashSet<Pos>, p: Pos) -> usize {
    if !robots.remove(&p) {
        0
    } else {
        1 + [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .map(|d| fill(robots, add(p, d)))
            .sum::<usize>()
    }
}

fn artistic_heuristic(robots: &mut [Robot]) -> usize {
    let mut rob = robots.iter().map(|&(p, _)| p).collect::<HashSet<Pos>>();
    robots.iter().map(|&(p, _)| fill(&mut rob, p)).max().unwrap()
}

fn christmas_tree(robots: &mut [Robot], w: i32, h: i32) -> usize {
    let size = (w, h);
    let (a, b) = (14, 78); // perhaps just for my input
    for i in 1..99999 {
        for r in &mut *robots {
            r.0 = add(r.0, r.1);
            r.0 = mod_(add(r.0, size), size);
        }
        let h = artistic_heuristic(robots);
        println!("after {} seconds h {}", i, h);
        if (i - a) % 101 == 0 || (i - b) % 103 == 0 {
            dump(robots, size);
        }
        // arbitrary "good enough" limit
        if h > 40 {
            return i;
        }
    }
    panic!()
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
    println!("{}", safety_factor(&mut robots.clone(), 101, 103, 100));
    println!("{}", christmas_tree(&mut robots, 101, 103));
}
