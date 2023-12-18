use std::io::{self, BufRead};
use std::collections::HashSet;

type DigStep = (Coord, i32, String);
type Map = HashSet<Coord>;

// x right, y down
type Coord = (i32, i32);

fn cw(pos: Coord) -> Coord {
    (-pos.1, pos.0)
}

fn _ccw(pos: Coord) -> Coord {
    (pos.1, -pos.0)
}

fn sum(a: Coord, b: Coord) -> Coord {
    (a.0 + b.0, a.1 + b.1)
}

fn diff(a: Coord, b: Coord) -> Coord {
    (a.0 - b.0, a.1 - b.1)
}

fn dig(dig_plan: &[DigStep]) -> Map {
    let mut map = Map::new();
    let mut pos = (0, 0);
    for dig in dig_plan {
        for _ in 0..dig.1 {
            map.insert(pos);
            pos = sum(pos, dig.0);
        }
    }
    map
}

fn do_fill(map: &mut Map, pos: Coord) {
    if map.contains(&pos) {
        return;
    }
    map.insert(pos);
    do_fill(map, (pos.0 - 1, pos.1));
    do_fill(map, (pos.0 + 1, pos.1));
    do_fill(map, (pos.0, pos.1 - 1));
    do_fill(map, (pos.0, pos.1 + 1));
}

fn fill(dig_plan: &[DigStep], mut map: Map) -> Map {
    let mut pos = (0, 0);
    for dig in dig_plan {
        let right_hand = cw(dig.0);
        for _ in 0..dig.1 {
            do_fill(&mut map, sum(pos, right_hand));
            pos = sum(pos, dig.0);
        }
    }
    map
}

fn cross_product(a: Coord, b: Coord) -> i32 {
    a.0 * b.1 - a.1 * b.0
}

fn winding(steps: &[DigStep]) -> i32 {
   steps.iter()
        .zip(steps.iter().cycle().skip(1))
        .zip(steps.iter().cycle().skip(2))
        .map(|((s0, s1), s2)| {
            let d0 = diff(s1.0, s0.0);
            let d1 = diff(s2.0, s1.0);
            // 0 for no change, -1 or +1 for 90 degree turns
            cross_product(d0, d1)
        })
        .sum::<i32>()
}

fn lava_amount(dig_plan: &[DigStep]) -> usize {
    assert!(winding(dig_plan) > 0); // clockwise
    let traced_map = dig(dig_plan);
    let filled_map = fill(dig_plan, traced_map);
    filled_map.len()
}

fn parse_dig(line: &str) -> DigStep {
    // R 6 (#70c710)
    let mut sp = line.split(' ');
    let dir = match sp.next().unwrap() {
        "U" => ( 0, -1),
        "D" => ( 0,  1),
        "R" => ( 1,  0),
        "L" => (-1,  0),
        _ => panic!(),
    };
    let steps = sp.next().unwrap().parse().unwrap();
    let color = sp.next().unwrap().strip_prefix("(").unwrap().strip_suffix(")").unwrap().to_string();
    (dir, steps, color)
}

fn main() {
    let dig_plan = io::stdin().lock().lines()
        .map(|row| parse_dig(&row.unwrap()))
        .collect::<Vec<_>>();
    println!("{}", lava_amount(&dig_plan));
}
