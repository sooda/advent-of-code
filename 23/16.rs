use std::io::{self, BufRead};
use std::collections::HashSet;

type Map = Vec<Vec<char>>;

// x right, y down
type Coord = (i32, i32);

fn cw(pos: Coord) -> Coord {
    (-pos.1, pos.0)
}

fn ccw(pos: Coord) -> Coord {
    (pos.1, -pos.0)
}

fn energize(map: &Map, visited: &mut HashSet<(Coord, Coord)>, mut pos: Coord, mut dir: Coord) {
    let w = map[0].len() as i32;
    let h = map.len() as i32;
    loop {
        pos.0 += dir.0;
        pos.1 += dir.1;
        if pos.0 < 0 || pos.0 >= w || pos.1 < 0 || pos.1 >= h {
            return;
        }
        if visited.contains(&(pos, dir)) {
            return;
        }
        visited.insert((pos, dir));
        match (map[pos.1 as usize][pos.0 as usize], dir) {
            ('.', _) => (),
            ('/', (_, 0)) => dir = ccw(dir),
            ('/', (0, _)) => dir = cw(dir),
            ('\\', (_, 0)) => dir = cw(dir),
            ('\\', (0, _)) => dir = ccw(dir),
            ('|', (_, 0)) => {
                energize(map, visited, pos, (0, 1));
                energize(map, visited, pos, (0, -1));
                return;
            },
            ('|', (0, _)) => (),
            ('-', (_, 0)) => (),
            ('-', (0, _)) => {
                energize(map, visited, pos, (1, 0));
                energize(map, visited, pos, (-1, 0));
                return;
            },
            _ => panic!()
        }
    }
}

fn tiles_energized(map: &Map, startpos: Coord, dir: Coord) -> usize {
    let mut visited = HashSet::new();
    energize(map, &mut visited, (startpos.0 - dir.0, startpos.1 - dir.1), dir);
    visited.iter().map(|&(pos, _dir)| pos).collect::<HashSet::<Coord>>().len()
}

fn max_tiles_energized(map: &Map) -> usize {
    let w = map[0].len() as i32;
    let h = map.len() as i32;
    let left = (0..h).map(|y| tiles_energized(map, (0, y), (1, 0))).max().unwrap();
    let right = (0..h).map(|y| tiles_energized(map, (w - 1, y), (-1, 0))).max().unwrap();
    let top = (0..w).map(|x| tiles_energized(map, (x, 0), (0, 1))).max().unwrap();
    let bot = (0..w).map(|x| tiles_energized(map, (x, h - 1), (0, -1))).max().unwrap();
    [left, right, top, bot].into_iter().max().unwrap()
}

fn main() {
    let map = io::stdin().lock().lines()
        .map(|row| row.unwrap().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    println!("{}", tiles_energized(&map, (0, 0), (1, 0)));
    println!("{}", max_tiles_energized(&map));
}
