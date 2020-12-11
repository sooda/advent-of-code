use std::io::{self, BufRead};

const EMPTY: u8 = b'L';
const OCCUPIED: u8 = b'#';

type Map = Vec<Vec<u8>>;

fn w(map: &Map) -> usize {
    map[0].len()
}

fn h(map: &Map) -> usize {
    map.len()
}

fn rule1(map: &Map, x: i32, y: i32) -> u8 {
    let mut occupied_adjacent = 0;
    for &(nx, ny) in &[
        (x - 1, y - 1),
        (x    , y - 1),
        (x + 1, y - 1),
        (x - 1, y    ),
        (x + 1, y    ),
        (x - 1, y + 1),
        (x    , y + 1),
        (x + 1, y + 1),
    ] {
        if let Some(&thing) = map.get(ny as usize).and_then(|row| row.get(nx as usize)) {
            if thing == OCCUPIED {
                occupied_adjacent += 1;
            }
        }
    }

    match map[y as usize][x as usize] {
        EMPTY if occupied_adjacent == 0 => OCCUPIED,
        OCCUPIED if occupied_adjacent >= 4 => EMPTY,
        x => x
    }
}

fn rule2(map: &Map, x: i32, y: i32) -> u8 {
    let mut occupied_adjacent = 0;
    for &(dx, dy) in &[
        ( - 1,   - 1),
        (   0,   - 1),
        (   1,   - 1),
        ( - 1,     0),
        (   1,     0),
        ( - 1,     1),
        (   0,     1),
        (   1,     1),
    ] {
        let mut nx = x;
        let mut ny = y;
        loop {
            nx += dx;
            ny += dy;
            if let Some(&thing) = map.get(ny as usize).and_then(|row| row.get(nx as usize)) {
                if thing == OCCUPIED {
                    occupied_adjacent += 1;
                    break;
                } else if thing == EMPTY {
                    // can't see further
                    break;
                }
            } else {
                break;
            }
        }
    }

    match map[y as usize][x as usize] {
        EMPTY if occupied_adjacent == 0 => OCCUPIED,
        OCCUPIED if occupied_adjacent >= 5 => EMPTY,
        x => x
    }
}

fn simulate(map: &Map, rule: fn(&Map, i32, i32) -> u8) -> Map {
    let mut new_map = vec![vec![b'?'; w(map)]; h(map)];
    for y in 0..h(map) {
        for x in 0..w(map) {
            new_map[y][x] = rule(map, x as i32, y as i32);
        }
    }
    new_map
}

fn num_seated(map: &Map) -> usize {
    map.iter().map(|row| row.iter().filter(|&&x| x == OCCUPIED).count()).sum()
}

fn dump(map: &Map) {
    for row in map {
        for &ch in row {
            print!("{}", ch as char);
        }
        println!();
    }
    println!();
}

fn stable_state_seated(mut map: Map, rule: fn(&Map, i32, i32) -> u8) -> usize {
    let mut next;
    loop {
        if false {
            dump(&map);
        }
        next = simulate(&map, rule);
        if next == map {
            return num_seated(&map);
        }
        map = next;
    }
}

fn main() {
    let map: Map = io::stdin().lock().lines()
        .map(|line| line.unwrap().into_bytes())
        .collect();
    println!("{}", stable_state_seated(map.clone(), rule1));
    println!("{}", stable_state_seated(map, rule2));
}
