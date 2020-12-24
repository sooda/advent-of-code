use std::io::{self, BufRead};
use std::collections::HashMap;

// (x, y) is (east, northeast)
type Coord = (i32, i32);

fn walk_direction(dir: &[Coord]) -> Coord {
    dir.iter().fold((0, 0), |coord, step| {
        (coord.0 + step.0, coord.1 + step.1)
    })
}

fn parse_direction(line: &str) -> Vec<Coord> {
    let mut chars = line.chars();
    let mut out = Vec::new();
    while let Some(ch) = chars.next() {
        let step = match ch {
            'e' => (1, 0),
            's' => match chars.next().unwrap() {
                'e' => (1, -1),
                'w' => (0, -1),
                _ => panic!(),
            },
            'w' => (-1, 0),
            'n' => match chars.next().unwrap() {
                'w' => (-1, 1),
                'e' => (0, 1),
                _ => panic!(),
            },
            _ => panic!(),
        };
        out.push(step);
    }
    out
}

fn tiles_flipped(directions: &[Vec<Coord>]) -> usize {
    // nonexistent: white side up
    // false: white side up, has been flipped
    // true: black side up, has been flipped
    let mut floor: HashMap<Coord, bool> = HashMap::new();
    for dir in directions {
        let tile = floor.entry(walk_direction(dir)).or_insert(false);
        *tile = !*tile;
    }
    floor.values().filter(|&&v| v).count()
}

fn main() {
    assert_eq!(walk_direction(&parse_direction("esew")), (1, -1));
    assert_eq!(walk_direction(&parse_direction("nwwswee")), (0, 0));
    let directions: Vec<Vec<Coord>> = io::stdin().lock().lines()
        .map(|line| parse_direction(&line.unwrap()))
        .collect();
    println!("{}", tiles_flipped(&directions));
}
