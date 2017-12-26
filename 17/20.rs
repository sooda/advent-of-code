use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

extern crate regex;
use regex::Regex;

use std::ops;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Vec3(i64, i64, i64);

#[derive(Debug, Clone, Copy)]
struct Point {
    p: Vec3,
    v: Vec3,
    a: Vec3
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

fn parse_line(line: &str) -> Point {
    let re = Regex::new(
        r"p=<([-\d]+),([-\d]+),([-\d]+)>, v=<([-\d]+),([-\d]+),([-\d]+)>, a=<([-\d]+),([-\d]+),([-\d]+)>"
        ).unwrap();
    let cap = re.captures(line).unwrap();
    let get = |i| cap.get(i).unwrap().as_str().parse::<i64>().unwrap();
    Point {
        p: Vec3(get(1), get(2), get(3)),
        v: Vec3(get(4), get(5), get(6)),
        a: Vec3(get(7), get(8), get(9))
    }
}

fn step(universe: &mut [Point]) {
    for p in universe.iter_mut() {
        p.v += p.a;
        p.p += p.v;
    }
}

fn zeroest_particle(universe: &mut [Point]) -> usize {
    // idk, maybe it will converge in this time
    let steps = 2 * universe.iter().map(|p| p.p.0.max(p.p.1.max(p.p.2)))
        .max().unwrap() as usize;
    for _i in 0..steps {
        step(universe);
    }
    let min_i = universe.iter().map(|p| p.p.0.abs() + p.p.1.abs() + p.p.2.abs()).enumerate()
        .min_by(|&(_, x), &(_, y)| x.cmp(&y)).unwrap();
    min_i.0
}

fn del_collisions(universe: &mut Vec<Point>) -> usize {
    let mut next = 0;
    let orig_len = universe.len();
    loop {
        if next == universe.len() {
            break;
        }
        // remove all duplicates, including the original
        let kill_pos = universe[next].p;
        let mut deleted = false;
        loop {
            if let Some(remove_pos) = {
                // skip is just an optimization - no dupes before this one
                universe.iter().skip(next + 1).position(|&other| other.p == kill_pos)
            } {
                deleted = true;
                // could also swap_remove i guess, it doesn't affect the already processed order
                // but makes the debug prints hard to read.
                universe.remove(next + 1 + remove_pos);
                // stay in this loop for another possible duplicate
            } else {
                break;
            }
        }
        if deleted {
            universe.remove(next);
            // a new point shifts to this position
        } else {
            next += 1;
        }
    }

    // how many collided and thus died
    orig_len - universe.len()
}

fn del_escaper(universe: &mut Vec<Point>, exiting_origin: &[bool]) -> bool {
    // In a non-empty universe, there is always one for all these.
    // This is executed after dropping colliding duplicates.

    // Consider these two one-dimensional cases:
    // 1)
    // particle P has velocity  V, acceleration  A, pos X
    // particle Q has velocity -V, acceleration -A, pos Y < X
    // the first has max pos, the second max velocity since max gives the last one in the list.
    // 2)
    // particle P has velocity  V, acceleration  A, pos X
    // particle Q has velocity  V, acceleration  A, pos X-n
    // max pos gives us P, max velocity Q.
    //
    // To fix: prefer the index that max pos finds, by also considering the position after the
    // velocity in comparison key.

    let furthest_idx = [
        universe.iter().enumerate().max_by_key(|&(_, p)| p.p.0.abs()).unwrap().0,
        universe.iter().enumerate().max_by_key(|&(_, p)| p.p.1.abs()).unwrap().0,
        universe.iter().enumerate().max_by_key(|&(_, p)| p.p.2.abs()).unwrap().0,
    ];

    let fastest_idx = [
        universe.iter().enumerate().max_by_key(|&(_, p)| (p.v.0.abs(), p.p.0.abs())).unwrap().0,
        universe.iter().enumerate().max_by_key(|&(_, p)| (p.v.1.abs(), p.p.1.abs())).unwrap().0,
        universe.iter().enumerate().max_by_key(|&(_, p)| (p.v.2.abs(), p.p.2.abs())).unwrap().0,
    ];

    // HOWEVER... looks like my input does not contain some corner cases. What if the furthest and
    // fastest is exiting, but there is another particle just behind it, slightly slower, but
    // accelerating faster so that it would collide soon?  This worked even without the below
    // acceleration maximum.

    let snappiest_idx = [
        universe.iter().enumerate().max_by_key(|&(_, p)| (p.a.0.abs(), p.v.0.abs(), p.p.0.abs())).unwrap().0,
        universe.iter().enumerate().max_by_key(|&(_, p)| (p.a.1.abs(), p.v.1.abs(), p.p.1.abs())).unwrap().0,
        universe.iter().enumerate().max_by_key(|&(_, p)| (p.a.2.abs(), p.v.2.abs(), p.p.2.abs())).unwrap().0,
    ];

    println!("fu {:?} {:?} fa {:?} {:?}",
             furthest_idx,
             furthest_idx.iter().map(|&i| &universe[i]).collect::<Vec<_>>(),
             fastest_idx,
             fastest_idx.iter().map(|&i| &universe[i]).collect::<Vec<_>>(),
             );

    // Bleh, check just one dimension; should be enough. Dupes are annoying to track here.
    let fp = furthest_idx[0];
    let fv = fastest_idx[0];
    let fa = snappiest_idx[0];
    if fp == fv && fv == fa && exiting_origin[fp] {
        // Can't touch this, ever, so ignore from further consideration
        println!("escaping {:?}", universe[fp]);
        universe.remove(fp);
        true
    } else {
        false
    }
}

fn collision_winners(universe: &mut Vec<Point>) -> usize {
    // number of winners diverging from others, cut off during simulation
    let mut escaped = 0;

    for _i in 0.. {
        println!("i {}", _i);
        step(universe);

        let exiting_origin = universe.iter().map(|&p| {
            // velocity has the same sign as acceleration? won't change direction
            let const_dir = p.v.0 * p.a.0 >= 0 && p.v.1 * p.a.1 >= 0 && p.v.2 * p.a.2 >= 0;
            // velocity has the same sign as position? pos growing away from zero
            let outwards = p.v.0 * p.p.0 >= 0 && p.v.1 * p.p.1 >= 0 && p.v.2 * p.p.2 >= 0;
            const_dir && outwards
        }).collect::<Vec<_>>();

        println!("exiting {} of {}",
                 exiting_origin.iter().filter(|&&x| x).count(),
                 universe.len());

        // for p in universe.iter() {
        //     println!("{:?}", p);
        // }

        let orig_len = universe.len();
        let collided = del_collisions(universe);
        if collided > 0 {
            println!("at {}, died {} of {}", _i, collided, orig_len);
        }

        // this applies for when dropped a last escaper in last step as well
        if universe.len() == 0 {
            println!("all gone at {}", _i);
            break;
        }

        if del_escaper(universe, &exiting_origin) {
            escaped += 1;
        }
    }

    escaped
}

fn main() {
    let mut universe = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap())
        .lines().map(|x| parse_line(&x.unwrap())).collect::<Vec<_>>();
    println!("{:?}", zeroest_particle(&mut universe.clone()));
    println!("{:?}", collision_winners(&mut universe));
}
