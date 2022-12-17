use std::io::{self, BufRead};
use std::collections::HashSet;

fn map_height(map: &HashSet<(i64, i64)>) -> i64 {
    // y axis points up
    // 0 is the ground level, 1 is where the first rock stops
    // (x 0 is in the air though)
    map.iter().map(|&(_, y)| y).max().unwrap_or(0)
}

const WIDE: i64 = 7;
const LAUNCH_X: i64 = 2;
const LAUNCH_Y: i64 = 4;

fn render(map: &HashSet<(i64, i64)>) {
    let miny = map.iter().map(|&(_, y)| y).min().unwrap();
    let maxy = map.iter().map(|&(_, y)| y).max().unwrap();
    assert_eq!(miny, 1);
    for y in (1..=maxy).rev() {
        print!("|");
        for x in 0..WIDE {
            print!("{}", if map.contains(&(x, y)) { '#' } else { '.' });
        }
        println!("|");
    }
    println!("+-------+");
}

fn drop<D: Iterator<Item=i64>>(map: &mut HashSet<(i64, i64)>, shape: &[(i64, i64)], dirs: &mut D) {
    let shapewid = shape.iter().map(|&(x, _)| x).max().unwrap() - 0 + 1; // min always 0
    let (mut x, mut y) = (LAUNCH_X, map_height(map) + LAUNCH_Y);
    let rocks_ok = |nx, ny| shape.iter().all(|&(sx, sy)| !map.contains(&(nx + sx, ny + sy)));
    loop {
        // move left or right one step
        let dx = dirs.next().unwrap();
        let dy = 0;
        let edges_ok = x + dx >= 0 && x + dx + shapewid <= WIDE;
        if edges_ok && rocks_ok(x + dx, y + dy) {
            x += dx;
            y += dy;
        }

        // If I take one more step, I'll be the farthest away from home I've ever been.
        let dx = 0;
        let dy = -1;
        let edges_ok = y + dy > 0;
        if edges_ok && rocks_ok(x + dx, y + dy) {
            x += dx;
            y += dy;
        } else {
            break;
        }
    }

    for &(sx, sy) in shape.iter() {
        map.insert((x + sx, y + sy));
    }
}

fn end_height(directions: &[i64], rocks_limit: usize) -> i64 {
    // left and bottom is 0; x goes right, y goes up
    let shapes: [&[(i64, i64)]; 5] = [
        &[
            (0, 0), (1, 0), (2, 0), (3, 0)
        ],
        &[
                    (1, 2),
            (0, 1), (1, 1), (2, 1),
                    (1, 0)
        ],
        &[
                            (2, 2),
                            (2, 1),
            (0, 0), (1, 0), (2, 0),
        ],
        &[
            (0, 3),
            (0, 2),
            (0, 1),
            (0, 0),
        ],
        &[
            (0, 1), (1, 1),
            (0, 0), (1, 0),
        ],
    ];

    let mut map: HashSet<(i64, i64)> = HashSet::new();
    let mut dirs = directions.iter().copied().cycle();

    for (i, shape) in (0..rocks_limit).zip(shapes.iter().cycle()) {
        drop(&mut map, shape, &mut dirs);
        if false {
            println!("at {} hei {}", i, map_height(&map));
            render(&map);
            println!();
        }
    }

    map_height(&map)
}

fn main() {
    let directions: Vec<i64> = io::stdin().lock().lines().next().unwrap().unwrap()
        .as_bytes().iter().map(|b| {
            match b {
                b'<' => -1,
                b'>' => 1,
                _ => panic!()
            }
        })
        .collect();
    println!("{}", end_height(&directions, 2022));
}
