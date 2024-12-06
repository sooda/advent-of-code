use std::io::{self, BufRead};

type Map = Vec<Vec<char>>;

fn start_pos(map: &Map) -> (i32, i32) {
    let (x, y, _) = map.iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, &ch)| (x, y, ch)))
        .find(|&(_, _, ch)| ch == '^')
        .unwrap();
    (x as i32, y as i32)
}

fn route_positions(map: &Map) -> usize {
    let (mut x, mut y) = start_pos(map);
    let (mut dx, mut dy) = (0, -1); // up
    let w = map[0].len() as i32;
    let h = map.len() as i32;
    let mut positions = vec![vec![false; map[0].len()]; map.len()];
    loop {
        positions[y as usize][x as usize] = true;
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
    positions.into_iter()
        .map(|row| row.into_iter().filter(|&x| x).count())
        .sum()
}

fn main() {
    let map: Map = io::stdin().lock().lines()
        .map(|line| line.unwrap()
             .chars().collect()
            ).collect();
    println!("{}", route_positions(&map));
}
