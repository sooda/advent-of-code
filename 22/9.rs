use std::io::{self, BufRead};
use std::collections::HashMap;

type Move = (char, i32);

fn dump_visits(visits: &HashMap::<(i32, i32), usize>) {
    let minx = visits.keys().map(|&(x, _)| x).min().unwrap();
    let maxx = visits.keys().map(|&(x, _)| x).max().unwrap();
    let miny = visits.keys().map(|&(_, y)| y).min().unwrap();
    let maxy = visits.keys().map(|&(_, y)| y).max().unwrap();
    for y in (miny..=maxy).rev() {
        for x in minx..=maxx {
            if visits.contains_key(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn tail_visits_once(moves: &[Move]) -> usize {
    let mut visits = HashMap::<(i32, i32), usize>::new();
    let mut head = (0, 0);
    let mut tail = (0, 0);
    visits.insert(tail, 1);
    let delta = |h: (_, _), t: (_, _)| (h.0 - t.0, h.1 - t.1);
    for &(dir, length) in moves {
        if false {
            println!("looks like:");
            dump_visits(&visits);
            println!("then {} {}", dir, length);
        }
        // x axis points to the right, y up
        let (dx, dy) = match dir {
            'L' => (-1, 0),
            'R' => (1, 0),
            'U' => (0, 1),
            'D' => (0, -1),
            _ => panic!("bad dir"),
        };
        for i in 0..length {
            head = (head.0 + dx, head.1 + dy);
            let d = delta(head, tail);
            if false {
                println!("{}: tail {:?}, head {:?}, d {:?}", i, tail, head, d);
                dump_visits(&visits);
            }
            if d.0 == 0 || d.1 == 0 {
                // directly in line
                if d.0.abs() > 1 || d.1.abs() > 1 {
                    tail = (tail.0 + dx, tail.1 + dy);
                }
            } else {
                // diagonal move
                if d.0.abs() > 1 || d.1.abs() > 1 {
                    tail = (tail.0 + d.0.signum(), tail.1 + d.1.signum());
                }
            }
            visits.entry(tail).and_modify(|n| *n += 1).or_insert(1);
        }
    }
    visits.values().count()
}

fn parse_move(line: &str) -> Move {
    let mut sp = line.split(' ');
    let dir = sp.next().unwrap().chars().next().unwrap();
    let length = sp.next().unwrap().parse().unwrap();

    (dir, length)
}

fn main() {
    let moves: Vec<_> = io::stdin().lock().lines()
        .map(|line| parse_move(&line.unwrap()))
        .collect();
    println!("{}", tail_visits_once(&moves));
}
