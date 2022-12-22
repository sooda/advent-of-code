use std::io::{self, BufRead};
use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
enum Tile {
    Open,
    Wall,
}
use Tile::*;

type Coords = (i64, i64);

type Map = HashMap<Coords, Tile>;

#[derive(Copy, Clone, Debug)]
enum Rotation {
    Left,
    Right,
}
use Rotation::*;

// The input begins and ends with walking, so this is injected with extra rotation in the front.
// Heading is initially up, this begins with Right to bring the heading to the right as specified.
type Route = Vec<(Rotation, i64)>;
type Notes = (Map, Route);

fn left(dir: Coords) -> Coords {
    (dir.1, -dir.0)
}

fn right(dir: Coords) -> Coords {
    (-dir.1, dir.0)
}

fn add(a: Coords, b: Coords) -> Coords {
    (a.0 + b.0, a.1 + b.1)
}

fn wrap(map: &Map, pos: Coords, dir: Coords) -> Coords {
    *match dir {
        (1,  0) => map.keys().filter(|&&(_, y)| y == pos.1).min_by_key(|&(x, _)| x).unwrap(),
        (-1, 0) => map.keys().filter(|&&(_, y)| y == pos.1).max_by_key(|&(x, _)| x).unwrap(),
        (0,  1) => map.keys().filter(|&&(x, _)| x == pos.0).min_by_key(|&(_, y)| y).unwrap(),
        (0, -1) => map.keys().filter(|&&(x, _)| x == pos.0).max_by_key(|&(_, y)| y).unwrap(),
        _ => panic!()
    }
}

fn final_pos(notes: &Notes) -> (Coords, Coords) {
    let mut pos: Coords = *notes.0.keys().filter(|&&(_, y)| y == 0).min_by_key(|&(x, _)| x).unwrap();
    let mut dir = (0, -1); // up
    for &(rot, steps) in &notes.1 {
        dir = match rot {
            Left => left(dir),
            Right => right(dir),
        };
        for _ in 0..steps {
            let next = add(pos, dir);
            let (next, tile) = match notes.0.get(&next) {
                Some(tile) => (next, tile),
                None => {
                    let next = wrap(&notes.0, pos, dir);
                    (next, notes.0.get(&next).unwrap())
                }
            };
            let newpos = match tile {
                Open => next,
                Wall => pos,
            };
            pos = newpos;
        }
    }
    (pos, dir)
}

fn final_password(notes: &Notes) -> i64 {
    let (pos, dir) = final_pos(notes);
    let facing = match dir {
        (1,  0) => 0,
        (0,  1) => 1,
        (-1, 0) => 2,
        (0, -1) => 3,
        _ => panic!()
    };
    1000 * (pos.1 + 1) + 4 * (pos.0 + 1) + facing
}

fn parse_map(note: &[String]) -> Map {
    note.iter().enumerate().flat_map(|(y, row)| {
        row.chars().enumerate().filter_map(move |(x, ch)| {
            match ch {
                ' ' => None,
                '.' => Some(((x as i64, y as i64), Open)),
                '#' => Some(((x as i64, y as i64), Wall)),
                _ => panic!("bad input"),
            }
        })
    }).collect::<Map>()
}

// 10R5L5R10L4R5L5
fn parse_route(mut note: &str) -> Route {
    // begins with a number
    let mut route = Vec::new();
    let mut prev_rot = Right;
    while let Some(pos) = note.as_bytes().iter().position(|&a| a == b'L' || a == b'R') {
        let (steps, rest) = note.split_at(pos);
        route.push((prev_rot, steps.parse().unwrap()));
        prev_rot = match rest.chars().next().unwrap() {
            'L' => Left,
            'R' => Right,
            _ => panic!()
        };
        note = rest.split_at(1).1;
    }
    route.push((prev_rot, note.parse().unwrap()));
    route
}

fn parse_notes(data: &[String]) -> Notes {
    let mut sp = data.split(|row| row == "");
    let map = parse_map(sp.next().unwrap());
    let route = parse_route(&sp.next().unwrap()[0]);
    (map, route)
}

fn main() {
    let data: Vec<_> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect();
    let notes = parse_notes(&data);
    println!("{}", final_password(&notes));
}
