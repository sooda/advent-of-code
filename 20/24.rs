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

type Floor = HashMap<Coord, bool>;

fn flip_tiles(directions: &[Vec<Coord>]) -> Floor {
    // nonexistent: white side up
    // false: white side up, has been flipped
    // true: black side up, has been flipped
    let mut floor = Floor::new();
    for dir in directions {
        let tile = floor.entry(walk_direction(dir)).or_insert(false);
        *tile = !*tile;
    }
    floor
}

fn black_tiles_up(floor: &Floor) -> usize {
    floor.values().filter(|&&v| v).count()
}

fn tiles_flipped(directions: &[Vec<Coord>]) -> usize {
    let floor = flip_tiles(directions);
    black_tiles_up(&floor)
}

// could also expand the world by one in each step, can't grow more than that
fn dimensions(floor: &Floor) -> ((i32, i32), (i32, i32)) {
    // the world starts at level 0 so all of these are ok to use, in the worst case this would just
    // be too conservative but wouldn't break anything
    let (mut x0, mut x1, mut y0, mut y1) = (0, 0, 0, 0);
    for (&pos, &color) in floor.iter() {
        if color {
            x0 = x0.min(pos.0);
            x1 = x1.max(pos.0);
            y0 = y0.min(pos.1);
            y1 = y1.max(pos.1);
        }
    }
    ((x0, x1), (y0, y1))
}

fn neighbourhood(floor: &Floor, pos: Coord) -> usize {
    let friends = &[
        (1, 0),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (0, -1),
        (1, -1),
    ];
    friends.iter().filter(|f| {
        floor.get(&(pos.0 + f.0, pos.1 + f.1)) == Some(&true)
    }).count()
}

fn update_cell(floor: &Floor, pos: Coord) -> bool {
    let n = neighbourhood(floor, pos);
    if floor.get(&pos) == Some(&true) {
        // was black
        !(n == 0 || n > 2)
    } else {
        // was white
        n == 2
    }
}

fn animate(floor: &Floor) -> Floor {
    let dim = dimensions(floor);
    // this doesn't seem to be very exactly needed; a simple floor.len() would also result in a
    // nice speedup from no initial capacity.
    let (w, h) = (dim.0.1 + - dim.0.0 + 2, dim.1.1 + - dim.1.0 + 2);
    let mut next = Floor::with_capacity((w * h) as usize);
    for x in (dim.0.0 - 1)..=(dim.0.1 + 1) {
        for y in (dim.1.0 - 1)..=(dim.1.1 + 1) {
            let state = update_cell(floor, (x, y));
            next.insert((x, y), state);
        }
    }
    return next;
}

fn animated_tiles(directions: &[Vec<Coord>], days: usize) -> usize {
    let mut floor = flip_tiles(directions);
    for _day in 0..days {
        floor = animate(&floor);
    }
    black_tiles_up(&floor)
}

fn main() {
    assert_eq!(walk_direction(&parse_direction("esew")), (1, -1));
    assert_eq!(walk_direction(&parse_direction("nwwswee")), (0, 0));
    let directions: Vec<Vec<Coord>> = io::stdin().lock().lines()
        .map(|line| parse_direction(&line.unwrap()))
        .collect();
    println!("{}", tiles_flipped(&directions));
    println!("{}", animated_tiles(&directions, 100));
}
