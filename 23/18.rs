use std::io::{self, BufRead};
use std::collections::HashSet;
use std::i64;

type DigStep = (Coord, i64, String);
type Map = HashSet<Coord>;

// x right, y down
type Coord = (i64, i64);

fn cw(pos: Coord) -> Coord {
    (-pos.1, pos.0)
}

fn _ccw(pos: Coord) -> Coord {
    (pos.1, -pos.0)
}

fn sum(a: Coord, b: Coord) -> Coord {
    (a.0 + b.0, a.1 + b.1)
}

fn mul(a: i64, b: Coord) -> Coord {
    (a * b.0, a * b.1)
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

fn cross_product(a: Coord, b: Coord) -> i64 {
    a.0 * b.1 - a.1 * b.0
}

fn winding(steps: &[DigStep]) -> i64 {
   steps.iter()
        .zip(steps.iter().cycle().skip(1))
        .zip(steps.iter().cycle().skip(2))
        .map(|((s0, s1), s2)| {
            let d0 = diff(s1.0, s0.0);
            let d1 = diff(s2.0, s1.0);
            // 0 for no change, -1 or +1 for 90 degree turns
            cross_product(d0, d1)
        })
        .sum::<i64>()
}

fn shoelace(dig_plan: &[DigStep]) -> usize {
    // shoelace formula
    // x1y2 + x2y3 + ... + xny1
    // -
    // y1x2 + y2x3 + ... + ynx1
    let coords = dig_plan.iter()
        .scan((0, 0), |state, x| {
            let ret = Some(*state);
            *state = sum(*state, mul(x.1, x.0));
            ret
        }).collect::<Vec<Coord>>();
    let sl = coords.iter()
        .zip(coords.iter().cycle().skip(1))
        .map(|(p0, p1)| {
            (p1.0 + p0.0) * (p1.1 - p0.1)
            //p0.0 * p1.1 - p0.1 * p1.0
        })
    .sum::<i64>().abs() as usize / 2;
    // interpret each coordinate to be in the middle of a cell.
    // each boundary tile thus has half extra,
    // plus the four main corners have 1/4 extra each,
    // plus any additional inside/outside pair cancels out.
    // ######
    // #    #
    // #   ##
    // #####
    // not sure if this would work if the route would intersect itself though
    let boundary = dig_plan.iter()
        .map(|step| step.1)
        .sum::<i64>() as usize / 2 + 1;
    sl + boundary
}

fn lava_amount(dig_plan: &[DigStep]) -> usize {
    assert!(winding(dig_plan) > 0); // clockwise
    let traced_map = dig(dig_plan);
    let filled_map = fill(dig_plan, traced_map);
    assert_eq!(shoelace(&dig_plan), filled_map.len());
    filled_map.len()
}

fn corrected_step(dig_step: &DigStep) -> DigStep {
    let digits = dig_step.2.strip_prefix('#').unwrap();
    let (left, right) = digits.split_at(5);
    let steps = i64::from_str_radix(left, 16).unwrap();
    let dir = match right {
        "0" => ( 1,  0),
        "1" => ( 0,  1),
        "2" => (-1,  0),
        "3" => ( 0, -1),
        _  => panic!()
    };
    (dir, steps, "".to_string())
}

fn corrected_dig_plan(dig_plan: &[DigStep]) -> Vec<DigStep> {
    dig_plan.iter().map(corrected_step).collect()
}

fn corrected_lava_amount(dig_plan: &[DigStep]) -> usize {
    shoelace(&corrected_dig_plan(dig_plan))
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
    let steps = sp.next().unwrap()
        .parse().unwrap();
    let color = sp.next().unwrap()
        .strip_prefix("(").unwrap()
        .strip_suffix(")").unwrap()
        .to_string();
    (dir, steps, color)
}

fn main() {
    let dig_plan = io::stdin().lock().lines()
        .map(|row| parse_dig(&row.unwrap()))
        .collect::<Vec<_>>();
    println!("{}", lava_amount(&dig_plan));
    println!("{}", corrected_lava_amount(&dig_plan));
}
