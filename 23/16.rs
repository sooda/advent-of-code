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

fn tiles_energized(map: &Map) -> usize {
    let mut visited = HashSet::new();
    energize(map, &mut visited, (-1, 0), (1, 0));
    visited.iter().map(|&(pos, _dir)| pos).collect::<HashSet::<Coord>>().len()
}

fn main() {
    let map = io::stdin().lock().lines()
        .map(|row| row.unwrap().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    println!("{}", tiles_energized(&map));
}
