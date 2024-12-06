use std::io::{self, BufRead};
use std::collections::HashSet;

type Map = Vec<Vec<char>>;

fn start_pos(map: &Map) -> (i32, i32) {
    let (x, y, _) = map.iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, &ch)| (x, y, ch)))
        .find(|&(_, _, ch)| ch == '^')
        .unwrap();
    (x as i32, y as i32)
}

fn route_positions(map: &Map) -> Option<usize> {
    let (mut x, mut y) = start_pos(map);
    let (mut dx, mut dy) = (0, -1); // up
    let w = map[0].len() as i32;
    let h = map.len() as i32;
    let mut positions = vec![vec![false; map[0].len()]; map.len()];
    let mut visits = HashSet::new();
    loop {
        positions[y as usize][x as usize] = true;
        if !visits.insert(((x, y), (dx, dy))) {
            // cycle, can't get to the end
            return None;
        }
        if (x == 0 && dx == -1) ||
                (x == w-1 && dx == 1) ||
                (y == 0 && dy == -1) ||
                (y == h-1 && dy == 1) {
            break;
        }
        while map[(y + dy) as usize][(x + dx) as usize] == '#' {
            (dx, dy) = (-dy, dx);
        }
        x += dx;
        y += dy;
    }
    Some(positions.into_iter()
        .map(|row| row.into_iter().filter(|&x| x).count())
        .sum())
}

fn possible_obstruction(map: &Map, x: usize, y: usize) -> bool {
    if map[y][x] != '.' {
        false
    } else {
        let mut map = map.clone();
        map[y][x] = '#';
        route_positions(&map).is_none()
    }
}

fn possible_obstructions(map: &Map) -> usize {
    (0..map.len())
        .flat_map(|y| (0..map[0].len()).map(move |x| (x, y)))
        .filter(|&(x, y)| possible_obstruction(&map, x, y))
        .count()
}

fn main() {
    let map: Map = io::stdin().lock().lines()
        .map(|line| line.unwrap()
             .chars().collect()
            ).collect();
    println!("{}", route_positions(&map).unwrap());
    println!("{}", possible_obstructions(&map));
}
