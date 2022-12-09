use std::io::{self, BufRead};
use std::collections::HashMap;

type Move = (char, i32);
type Pt = (i32, i32);

fn dump_visits(visits: &HashMap::<Pt, usize>, knots: &[Pt]) {
    let it = visits.keys().chain(knots.iter());
    let minx = it.clone().map(|&(x, _)| x).min().unwrap();
    let maxx = it.clone().map(|&(x, _)| x).max().unwrap();
    let miny = it.clone().map(|&(_, y)| y).min().unwrap();
    let maxy = it.clone().map(|&(_, y)| y).max().unwrap();

    for y in (miny..=maxy).rev() {
        for x in minx..=maxx {
            let mut ch = '.';
            if visits.contains_key(&(x, y)) {
                ch = '#';
            }
            if (x, y) == (0, 0) {
                ch = 's';
            }
            for (ki, &t) in knots.iter().enumerate().skip(1).rev() {
                if (x, y) == t {
                    ch = (b'0' + ki as u8) as char;
                }
            }
            if (x, y) == knots[0] {
                ch = 'H';
            }
            print!("{}", ch);
        }
        println!();
    }
}

fn delta(h: Pt, t: Pt) -> Pt {
    (h.0 - t.0, h.1 - t.1)
}

fn sum(a: Pt, b: Pt) -> Pt {
    (a.0 + b.0, a.1 + b.1)
}

fn knotty_move(head: Pt, tail: Pt) -> Pt {
    let d = delta(head, tail);
    if d.0.abs() > 1 || d.1.abs() > 1 {
        sum(tail, (d.0.signum(), d.1.signum()))
    } else {
        tail
    }
}

fn tail_visits_once(moves: &[Move], nknots: usize) -> usize {
    let mut visits = HashMap::<Pt, usize>::new();
    // knots[0] moves first, knots.last() is the end of the rope
    let mut knots = vec![(0, 0); nknots];
    visits.insert((0, 0), 1);

    for &(dir, length) in moves {
        if false {
            println!("before {} {}:", dir, length);
            dump_visits(&visits, &knots);
        }
        // x axis points to the right, y up
        let dhead = match dir {
            'L' => (-1, 0),
            'R' => (1, 0),
            'U' => (0, 1),
            'D' => (0, -1),
            _ => panic!("bad dir"),
        };
        for _ in 0..length {
            knots[0] = sum(knots[0], dhead);
            for ki in 1..nknots {
                let (src, dest) = (ki - 1, ki);
                knots[dest] = knotty_move(knots[src], knots[dest]);
            }

            visits.entry(knots[nknots - 1]).and_modify(|n| *n += 1).or_insert(1);
        }
    }

    if false {
        println!("end looks like:");
        dump_visits(&visits, &knots);
    }

    visits.len()
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
    println!("{}", tail_visits_once(&moves, 2));
    println!("{}", tail_visits_once(&moves, 10));
}
