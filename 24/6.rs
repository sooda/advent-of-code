#![feature(let_chains)]

use std::io::{self, BufRead};
use std::collections::HashMap;

type Map = Vec<Vec<char>>;
type Pos = (i32, i32);
type Visits = HashMap<(Pos, Pos), usize>;
type Path = Vec<(Pos, Pos)>;

fn start_pos(map: &Map) -> (i32, i32) {
    let (x, y, _) = map.iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, &ch)| (x, y, ch)))
        .find(|&(_, _, ch)| ch == '^')
        .unwrap();
    (x as i32, y as i32)
}

fn right(p: Pos) -> Pos {
    (-p.1, p.0)
}

fn route_positions(map: &Map, start: (Pos, Pos), mut time: usize, mut visits: Visits) -> Option<(usize, Visits, Path)> {
    let ((mut x, mut y), (mut dx, mut dy)) = start;
    let w = map[0].len() as i32;
    let h = map.len() as i32;
    let mut positions = vec![vec![false; map[0].len()]; map.len()];
    let mut path = Path::new();

    loop {
        positions[y as usize][x as usize] = true;
        path.push(((x, y), (dx, dy)));
        if let Some(existing_time) = visits.insert(((x, y), (dx, dy)), time) {
            if existing_time < time {
                // cycle, can't get to the end
                return None;
            }
        }
        if (x == 0 && dx == -1) ||
                (x == w-1 && dx == 1) ||
                (y == 0 && dy == -1) ||
                (y == h-1 && dy == 1) {
            break;
        }
        while map[(y + dy) as usize][(x + dx) as usize] == '#' {
            (dx, dy) = right((dx, dy));
        }
        x += dx;
        y += dy;
        time += 1;
    }

    let count = positions.into_iter()
        .map(|row| row.into_iter().filter(|&x| x).count())
        .sum();
    Some((count, visits, path))
}

fn possible_obstructions(mut map: Map, mut visits: Visits, path: &Path) -> usize {
    let mut count = 0;
    for ((time, &player), &obstru) in path.iter().enumerate().zip(path.iter().skip(1)).rev() {
        // can't get here like this because the path would get obstructed earlier?
        let this_time = visits.remove(&obstru).unwrap();
        if let Some(&time) = visits.get(
                &(obstru.0, right(obstru.1))) && time < this_time {
            continue;
        }
        if let Some(&time) = visits.get(
                &(obstru.0, right(right(obstru.1)))) && time < this_time {
            continue;
        }
        if let Some(&time) = visits.get(
                &(obstru.0, right(right(right(obstru.1))))) && time < this_time {
            continue;
        }

        let orig_cell = std::mem::replace(&mut map[obstru.0.1 as usize][obstru.0.0 as usize], '#');
        let cycled = route_positions(&map, player, time, visits.clone()).is_none();
        if false { // flip this to double check
            let gold = route_positions(&map, (start_pos(&map), (0, -1)), 0, Visits::new()).is_none();
            if cycled != gold {
                panic!("mismatch at {:?} cycled {} gold {}", (player, obstru), cycled, gold);
            }
        }
        map[obstru.0.1 as usize][obstru.0.0 as usize] = orig_cell;
        if cycled {
            count += 1;
        }
    }
    count
}

fn main() {
    let map: Map = io::stdin().lock().lines()
        .map(|line| line.unwrap()
             .chars().collect()
            ).collect();
    let (counts, visits, path) = route_positions(
        &map, (start_pos(&map), (0, -1)), 0, Visits::new()).unwrap();
    println!("{}", counts);
    println!("{}", possible_obstructions(map, visits, &path));
}
