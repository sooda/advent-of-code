use std::io::{self, BufRead};

fn search(map: &[Vec<char>], pos: (i32, i32), dir: (i32, i32), i: usize) -> usize {
    let w = map[0].len() as i32;
    let h = map.len() as i32;
    let expect = ['M', 'A', 'S'][i];
    let pos = (pos.0 + dir.0, pos.1 + dir.1);
    let bad = pos.0 < 0 || pos.0 >= w || pos.1 < 0 || pos.1 >= h;
    let found = !bad && map[pos.1 as usize][pos.0 as usize] == expect;
    if found && i == 2 {
        1
    } else if found {
        search(map, pos, dir, i + 1)
    } else {
        0
    }
}

fn try_xmas(map: &[Vec<char>], pos: (i32, i32)) -> usize {
    let delta = &[
        (-1, -1), (0, -1), (1, -1),
        (-1,  0),          (1,  0),
        (-1,  1), (0,  1), (1,  1),
    ];
    delta.into_iter().map(|&d| search(map, pos, d, 0)).sum()
}

fn xmas_count(map: &[Vec<char>]) -> usize {
    let w = map[0].len();
    let h = map.len();
    let mut sum = 0;
    for x in 0..w {
        for y in 0..h {
            if map[y][x] == 'X' {
                sum += try_xmas(map, (x as i32, y as i32));
            }
        }
    }
    sum
}

// M.S
// .A.
// M.S
fn try_x_mas(map: &[Vec<char>], pos: (usize, usize)) -> usize {
    let (x, y) = pos;
    if map[y - 1][x - 1] == 'M' && map[y - 1][x + 1] == 'S' &&
            map[y][x] == 'A' &&
            map[y + 1][x - 1] == 'M' && map[y + 1][x + 1] == 'S' {
        1
    } else {
        0
    }
}

// counterclockwise
fn flip(map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let w = map[0].len();
    let h = map.len();
    let mut next = vec![vec!['.'; h]; w];
    // (x, 0) becomes (0, max-x),
    // (max, y) becomes (y, 0), ...
    for y in 0..h {
        for x in 0..w {
            next[w - 1 - x][y] = map[y][x];
        }
    }
    next
}

fn x_mas_count(mut map: Vec<Vec<char>>) -> usize {
    let w = map[0].len();
    let h = map.len();
    let mut sum = 0;
    for _ in 0..4 {
        for y in 1..(h-1) {
            for x in 1..(w-1) {
                sum += try_x_mas(&map, (x, y));
            }
        }
        map = flip(map);
    }
    sum
}

fn main() {
    let map: Vec<Vec<char>> = io::stdin().lock().lines()
        .map(
            |line| line.unwrap().chars().collect()
            ).collect();
    println!("{}", xmas_count(&map));
    println!("{}", x_mas_count(map));
}
