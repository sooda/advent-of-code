use std::io::{self, BufRead};
use std::collections::HashMap;
use std::iter::once;

type Move = (char, i32);

fn dump_visits(visits: &HashMap::<Pt, usize>, head: Pt, tail: &[Pt]) {
    let it = visits.keys().chain(once(&head)).chain(tail.iter());
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
            for (ti, &t) in tail.iter().enumerate().rev() {
                if (x, y) == t {
                    ch = (b'1' + ti as u8) as char;
                }
            }
            if (x, y) == head {
                ch = 'H';
            }
            print!("{}", ch);
        }
        println!();
    }
}

type Pt = (i32, i32);

fn delta(h: Pt, t: Pt) -> Pt {
    (h.0 - t.0, h.1 - t.1)
}

fn knotty_move(head: Pt, mut tail: Pt) -> Pt {
    let d = delta(head, tail);
    if d.0 == 0 || d.1 == 0 {
        // directly in line
        if d.0.abs() > 1 || d.1.abs() > 1 {
            if false {
                println!("direct {:?}", d);
            }
            tail = (tail.0 + d.0.signum(), tail.1 + d.1.signum());
        }
    } else {
        // diagonal move
        if d.0.abs() > 1 || d.1.abs() > 1 {
            if false {
                println!("diag {:?}", d);
            }
            tail = (tail.0 + d.0.signum(), tail.1 + d.1.signum());
        }
    }
    tail
}

fn tail_visits_once(moves: &[Move], ntails: usize) -> usize {
    let mut visits = HashMap::<Pt, usize>::new();
    let mut head = (0, 0);
    // tails[0] moves first, tails.last() is the end of the rope
    let mut tails = vec![(0, 0); ntails];
    visits.insert((0, 0), 1);
    for &(dir, length) in moves {
        if false {
            println!("looks like:");
            dump_visits(&visits, head, &tails);
            println!("then {} {}", dir, length);
        }
        // x axis points to the right, y up
        let dhead = match dir {
            'L' => (-1, 0),
            'R' => (1, 0),
            'U' => (0, 1),
            'D' => (0, -1),
            _ => panic!("bad dir"),
        };
        for i in 0..length {
            if false {
                println!();
                println!("move #{} from:", i);
                dump_visits(&visits, head, &tails);
                println!();
            }
            head = (head.0 + dhead.0, head.1 + dhead.1);
            if false {
                println!("for idx 0");
            }
            let t = knotty_move(head, tails[0]);
            if false {
                println!("after that move:");
                dump_visits(&visits, head, &tails);
            }
            tails[0] = t;
            for ti in 1..ntails {
                let (src, dest) = (ti - 1, ti);
                if false {
                    println!("for idx {}", ti);
                }
                let t = knotty_move(tails[src], tails[dest]);
                if false {
                    println!("after that move:");
                    dump_visits(&visits, head, &tails);
                }
                tails[dest] = t;
            }

            visits.entry(*tails.last().unwrap()).and_modify(|n| *n += 1).or_insert(1);
        }
    }
    if false {
        println!("end looks like:");
        dump_visits(&visits, head, &tails);
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
    println!("{}", tail_visits_once(&moves, 1));
    println!("{}", tail_visits_once(&moves, 9));
}
