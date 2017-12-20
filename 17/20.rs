use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

extern crate regex;
use regex::Regex;

use std::ops;

#[derive(Debug,Clone,Copy)]
struct Vec3(i64, i64, i64);

#[derive(Debug)]
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
        p.p += p.v;
        p.v += p.a;
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

fn main() {
    let mut universe = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap())
        .lines().map(|x| parse_line(&x.unwrap())).collect::<Vec<_>>();
    println!("{:?}", zeroest_particle(&mut universe));
}
