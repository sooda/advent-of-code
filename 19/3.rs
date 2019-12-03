use std::io::{self, BufRead};
use std::collections::HashSet;
use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
struct Step {
    direction: char,
    length: i32
}

impl FromStr for Step {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction = s.chars().next().unwrap();
        let length = s.split_at(1).1.parse()?;
        Ok(Step { direction, length })
    }
}

fn walk(path: &[Step]) -> HashSet<(i32, i32)> {
    let mut map = HashSet::new();
    let mut x = 0;
    let mut y = 0;

    // note: the origin is not in the map
    for s in path {
        for _ in 0..s.length {
            let (nx, ny) = match s.direction {
                'U' => (x, y - 1),
                'D' => (x, y + 1),
                'L' => (x - 1, y),
                'R' => (x + 1, y),
                _ => panic!("bad input")
            };
            x = nx;
            y = ny;
            map.insert((x, y));
        }
    }

    map
}

fn crossing_distance(a: &[Step], b: &[Step]) -> i32 {
    let map_a = walk(a);
    let map_b = walk(b);
    let wire_crossings = map_a.intersection(&map_b);
    let closest = wire_crossings.min_by_key(|(a, b)| a.abs() + b.abs()).unwrap();

    closest.0.abs() + closest.1.abs()
}

fn parseline(stepline: &str) -> Vec<Step> {
    stepline.split(',')
        .map(|stepdesc| stepdesc.parse().unwrap())
        .collect()
}

fn main() {
    assert_eq!(crossing_distance(
            &parseline("R8,U5,L5,D3"),
            &parseline("U7,R6,D4,L4")),
            6);
    assert_eq!(crossing_distance(
            &parseline("R75,D30,R83,U83,L12,D49,R71,U7,L72"),
            &parseline("U62,R66,U55,R34,D71,R55,D58,R83")),
            159);
    assert_eq!(crossing_distance(
            &parseline("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"),
            &parseline("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")),
            135);

    let wire_paths: Vec<Vec<Step>> = io::stdin().lock().lines()
        .map(|stepline| parseline(&stepline.unwrap()))
        .collect();
    println!("{}", crossing_distance(&wire_paths[0], &wire_paths[1]));
}
