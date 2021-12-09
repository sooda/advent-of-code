use std::io::{self, BufRead};

type Map = Vec<Vec<u32>>;

fn low_point_risk_level(map: &Map) -> u32 {
    let mut total_risk = 0;
    for y in 0..map.len() {
        let row = &map[y];
        for x in 0..row.len() {
            let mut low = true;
            let current = row[x];
            if x > 0 {
                low = low && current < row[x - 1];
            }
            if x < row.len() - 1 {
                low = low && current < row[x + 1];
            }
            if y > 0 {
                low = low && current < map[y - 1][x];
            }
            if y < map.len() - 1 {
                low = low && current < map[y + 1][x];
            }
            let risk_level = current + 1;
            if low {
                total_risk += risk_level;
            }
        }
    }
    total_risk
}

fn dfs_basin(map: &Map, visited: &mut Vec<Vec<bool>>, x: usize, y: usize) -> u32 {
    if visited[y][x] {
        return 0;
    }
    if map[y][x] == 9 {
        return 0;
    }
    visited[y][x] = true;
    let mut tot = 0;
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
        let row = &map[y];
        for x in 0..row.len() {
            let mut low = true;
            let current = row[x];
            if x > 0 {
                low = low && current < row[x - 1];
            }
            if x < row.len() - 1 {
                low = low && current < row[x + 1];
            }
            if y > 0 {
                low = low && current < map[y - 1][x];
            }
            if y < map.len() - 1 {
                low = low && current < map[y + 1][x];
            }
            if low {
                let size = basin_size(map, x, y);
                //println!("size at {} {} = {}", x, y, size);
                big_basin.push(size);
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

