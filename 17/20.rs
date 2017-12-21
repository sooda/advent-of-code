use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

extern crate regex;
use regex::Regex;

use std::ops;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
    // println!("{}", steps);
    for _i in 0..steps {
        step(universe);
        /*
        let min_i = universe.iter().map(|p| p.p.0.abs() + p.p.1.abs() + p.p.2.abs()).enumerate()
            .min_by(|&(_, x), &(_, y)| x.cmp(&y)).unwrap();
        println!("{} {:?}", _i, min_i);
        */
    }
    let min_i = universe.iter().map(|p| p.p.0.abs() + p.p.1.abs() + p.p.2.abs()).enumerate()
        .min_by(|&(_, x), &(_, y)| x.cmp(&y)).unwrap();
    min_i.0
}

//#[derive(Debug)]
//struct A(Point);
type A = Point;
//struct A(usize, Point);
/*
struct B(Vec<A>);
use std::iter::FromIterator;
impl FromIterator<(usize, Point)> for B {
    fn from_iter<I: IntoIterator<Item=(usize, Point)>>(iter: I) -> Self {
        let mut v = B::new();
        for i in iter {
            v.add(A(i.0, i.1));
        }
        v
    }
}
*/

/*
use std::cmp::Ordering;

impl Ord for A {
    fn cmp(&self, other: &A) -> Ordering {
        //println!("cmp {:?} {:?}", self, other);
        self.p.cmp(&other.p)
    }
}

impl PartialOrd for A {
    fn partial_cmp(&self, other: &A) -> Option<Ordering> {
        println!("peq {:?} {:?}", self, other);
        Some(self.cmp(other))
    }
}

impl PartialEq for A {
    fn eq(&self, other: &A) -> bool {
        println!("eq {:?} {:?}", self, other);
        (self).p == (other).p
    }
}

impl Eq for A { }
*/

fn del_collisions(universe: &mut Vec<Point>) -> usize {
    let mut next = 0;
    let mut orig_len = universe.len();
    loop {
        if next == universe.len() {
            break;
        }
        //println!("hit? {}", next);
        // remove all duplicates, including the original
        let kill_pos = universe[next].p.clone();
        //println!("pos {:?}?", kill_pos);
        let mut deleted = false;
        loop {
            if let Some(remove_pos) = {
                // skip is just an optimization - no dupes before this one
                universe.iter().skip(next + 1).position(|&other| other.p == kill_pos)
            } {
                println!("yes {} {:?}", next + 1 + remove_pos, universe[next + 1 + remove_pos]);
                deleted = true;
                universe.remove(next + 1 + remove_pos); // could also swap_remove
                // hold for another possible duplicate
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

fn collision_winners(universe: &mut Vec<Point>) -> usize {
    // doing all this with FromIter traits etc is just too nasty
    /*
    let mut indexed_universe = Vec::new();
    for i in universe.iter().cloned().enumerate() {
        indexed_universe.push(A(i.0, i.1));
    }
    */

    // exiting, _or_ staying put
    let mut escaped = 0;

    for _i in 0.. {
        println!("i {}", _i);
        /*
        for ip in &mut indexed_universe {
            println!("{:?}", ip);
            ip.1.p += ip.1.v;
            ip.1.v += ip.1.a;
        }
        */
        //step2(universe);
        //universe.sort();
        let mut exiting_origin = vec![false; universe.len()];
        for (p, exiting) in universe.iter_mut().zip(exiting_origin.iter_mut()) {
            let v0 = p.v;
            p.v += p.a;
            p.p += p.v;
            let dv = p.v - v0;
            // velocity has the same sign as acceleration? won't change direction
            let const_dir = p.v.0 * p.a.0 >= 0 && p.v.1 * p.a.1 >= 0 && p.v.2 * p.a.2 >= 0;
            // velocity has the same sign as position? pos growing away from zero
            let outwards = p.v.0 * p.p.0 >= 0 && p.v.1 * p.p.1 >= 0 && p.v.2 * p.p.2 >= 0;
            //println!("{:?} {:?} {:?}", p, const_dir, outwards);
            *exiting = const_dir && outwards;
        }
        println!("exiting {} of {}",
                 exiting_origin.iter().filter(|&&x| x).count(),
                 universe.len());

        for p in universe.iter() {
            //println!("{:?}", p);
        }
        let len = universe.len();
        let coll = del_collisions(universe);
        if coll > 0 {
            println!("at {}, died {} of {}", _i, coll, len);
        }
        if universe.len() == 0 {
            println!("all gone at {}", _i);
            break;
        }

        let mut furthest = [
            universe.iter().enumerate().max_by_key(|&(_, p)| p.p.0.abs()).unwrap().0,
            universe.iter().enumerate().max_by_key(|&(_, p)| p.p.1.abs()).unwrap().0,
            universe.iter().enumerate().max_by_key(|&(_, p)| p.p.2.abs()).unwrap().0,
        ];
        let mut fastest = [
            universe.iter().enumerate().max_by_key(|&(_, p)| p.v.0.abs()).unwrap().0,
            universe.iter().enumerate().max_by_key(|&(_, p)| p.v.1.abs()).unwrap().0,
            universe.iter().enumerate().max_by_key(|&(_, p)| p.v.2.abs()).unwrap().0,
        ];
        furthest.sort_by(|a, b| b.cmp(a));
        fastest.sort_by(|a, b| b.cmp(a));

        println!("{:?} {:?}", furthest, fastest);
        //for (&fu, &fa) in furthest.iter().zip(fastest.iter()) {
        {
            // based on just x is a slower b oundary condition but it doesn't matter to be just
            // conservative
            let fu = furthest[0];
            let fa = fastest[0];
            // can't touch this
            if fu == fa && exiting_origin[fu] {
                println!("at {}, escaping {:?}", _i, universe[fu]);
                let del = universe.len() - 1;
                universe.remove(del);
                escaped += 1;
                if universe.len() == 0 {
                    break;
                }
            }
        }
            /*
        if exiting_origin.iter().all(|&x| x) {
            println!("done in {}", _i);
            break;
        }
        */
        //println!("{} {}", _i, universe.len());
        /*
        for i in 0..indexed_universe.len() {
            if i == indexed_universe.len() {
                break;
            }
            loop {
                let remove_pos = {
                    let last = &indexed_universe[i];
                    let pos = indexed_universe.iter().skip(i + 1)
                        .position(|ref x| x.1.p == last.1.p);
                    if let Some(pos) = pos {
                        println!("del {} of {}", i + pos, indexed_universe.len());
                        pos
                    } else {
                        break
                    }
                };
                indexed_universe.swap_remove(remove_pos);
            }
        }
        */
        /*
        indexed_universe.sort();
        indexed_universe.dedup();
        */
    }
    /*
    let mut a = vec![
        (0, Point { p: Vec3(0, 0, 0), v: Vec3(10, 0, 0), a: Vec3(0, 0, 0) }),
        (1, Point { p: Vec3(0, 0, 0), v: Vec3(20, 0, 0), a: Vec3(0, 0, 0) }),
    ];

    for i in &mut a {
        i.1.p += i.1.v;
    }
    println!("{:?}", a);
    */

    escaped
}

fn main() {
    let mut universe = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap())
        .lines().map(|x| parse_line(&x.unwrap())).collect::<Vec<_>>();
    println!("{:?}", zeroest_particle(&mut universe.clone()));
    println!("{:?}", collision_winners(&mut universe));
}
