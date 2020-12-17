use std::io::{self, BufRead};
use std::collections::HashSet;

type Map = HashSet<(i32, i32, i32, i32)>;

fn neighbourhood(map: &Map, x: i32, y: i32, z: i32, w: i32) -> usize {
    let mut total = 0;
    for dx in -1..=1 {
        for dy in -1..=1 {
            for dz in -1..=1 {
                for dw in -1..=1 {
                    if dx != 0 || dy != 0 || dz != 0 || dw != 0 {
                        total += map.contains(&(x + dx, y + dy, z + dz, w + dw)) as usize;
                    }
                }
            }
        }
    }
    total
}

fn update_cell(map: &Map, x: i32, y: i32, z: i32, w: i32) -> bool {
    let n = neighbourhood(map, x, y, z, w);
    if map.contains(&(x, y, z, w)) {
        n == 2 || n == 3
    } else {
        n == 3
    }
}

fn dimensions(map: &Map) -> ((i32, i32), (i32, i32), (i32, i32), (i32, i32)) {
    // the world starts at level 0 so all of these are ok to use, in the worst case this would just
    // be too conservative but wouldn't break anything
    let (mut x0, mut x1, mut y0, mut y1, mut z0, mut z1, mut w0, mut w1) = (0, 0, 0, 0, 0, 0, 0, 0);
    for k in map.iter() {
        x0 = x0.min(k.0);
        x1 = x1.max(k.0);
        y0 = y0.min(k.1);
        y1 = y1.max(k.1);
        z0 = z0.min(k.2);
        z1 = z1.max(k.2);
        w0 = w0.min(k.3);
        w1 = w1.max(k.3);
    }
    ((x0, x1), (y0, y1), (z0, z1), (w0, w1))
}

fn dump(map: &Map) {
    let dim = dimensions(map);
    for z in (dim.2.0)..=(dim.2.1) {
        println!("z={}", z);
        for y in (dim.1.0)..=(dim.1.1) {
            for x in (dim.0.0)..=(dim.0.1) {
                print!("{}", if map.contains(&(x, y, z, 0)) { '#' } else { '.' });
            }
            println!();
        }
        println!();
    }
    println!();
}

// could also BSP the space if it gets big and sparse, or consider just the set of active cells and
// choose their neighbors, but this is simpler for now
fn step(map: &Map, next: &mut Map) {
    let dim = dimensions(map);
    for x in (dim.0.0 - 1)..=(dim.0.1 + 1) {
        for y in (dim.1.0 - 1)..=(dim.1.1 + 1) {
            for z in (dim.2.0 - 1)..=(dim.2.1 + 1) {
                for w in (dim.3.0 - 1)..=(dim.3.1 + 1) {
                    let state = update_cell(map, x, y, z, w);
                    if state {
                        next.insert((x, y, z, w));
                    } else {
                        next.remove(&(x, y, z, w));
                    }
                }
            }
        }
    }
}

fn score(map: &Map) -> usize {
    map.len()
}

fn animate(boot_state: &Vec<Vec<char>>, n: usize, fourd: bool) -> usize {
    let mut map: Map = boot_state.iter()
        .enumerate().flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, &ch)| {
                match ch {
                    '#' => Some((x as i32, y as i32, 0, 0)),
                    '.' => None,
                    _ => panic!()
                }
            })
        }).collect();

    if false {
        println!("Before any cycles:");
        println!();
        dump(&map);
    }

    for i in 1..=n {
        let mut next: Map = Map::new();
        step(&map, &mut next);
        map = next;
        if !fourd {
            map.retain(|&(_x, _y, _z, _w)| _w == 0);
        }
        if false {
            println!("After {} cycles:", i);
            println!();
            dump(&map);
        }
    }

    score(&map)
}

fn main() {
    let boot_state: Vec<Vec<char>> = io::stdin().lock().lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();
    println!("{}", animate(&boot_state, 6, false));
    println!("{}", animate(&boot_state, 6, true));
}
