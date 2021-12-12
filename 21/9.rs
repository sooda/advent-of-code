use std::io::{self, BufRead};

type Map = Vec<Vec<u32>>;

fn low_point(map: &Map, x: usize, y: usize) -> bool {
    let current = map[y][x];
    if x > 0 && current >= map[y][x - 1] {
        return false;
    }
    if x < map[y].len() - 1 && current >= map[y][x + 1] {
        return false;
    }
    if y > 0 && current >= map[y - 1][x] {
        return false;
    }
    if y < map.len() - 1 && current >= map[y + 1][x] {
        return false;
    }
    true
}

fn low_point_risk_level(map: &Map) -> u32 {
    let mut total_risk = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if low_point(map, x, y) {
                total_risk += map[y][x] + 1;
            }
        }
    }
    total_risk
}

fn dfs_basin(map: &Map, visited: &mut Vec<Vec<bool>>, x: usize, y: usize) -> u32 {
    if visited[y][x] || map[y][x] == 9 {
        return 0;
    }
    visited[y][x] = true;
    let mut tot = 0;
    // isize would again be nicer because x-1 etc. could be in an array. ugh.
    if x > 0 {
        tot += dfs_basin(map, visited, x - 1, y);
    }
    if x < map[y].len() - 1 {
        tot += dfs_basin(map, visited, x + 1, y);
    }
    if y > 0 {
        tot += dfs_basin(map, visited, x, y - 1);
    }
    if y < map.len() - 1 {
        tot += dfs_basin(map, visited, x, y + 1);
    }
    1 + tot
}

fn basin_size(map: &Map, x: usize, y: usize) -> u32 {
    let mut visited = vec![vec![false; map[0].len()]; map.len()];
    dfs_basin(map, &mut visited, x, y)
}

fn basin_product(map: &Map) -> u32 {
    let mut big_basin = Vec::new();
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if low_point(map, x, y) {
                big_basin.push(basin_size(map, x, y));
            }
        }
    }
    big_basin.sort_unstable();
    big_basin.iter().rev().take(3).fold(1, |product, size| product * size)
}

fn main() {
    let map: Map = io::stdin().lock().lines()
        .map(|input| input.unwrap().bytes().map(|b| (b - b'0') as u32).collect())
        .collect();
    println!("{}", low_point_risk_level(&map));
    println!("{}", basin_product(&map));
}

