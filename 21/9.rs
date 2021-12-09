use std::io::{self, BufRead};
use std::collections::HashSet;

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

fn main() {
    let map: Map = io::stdin().lock().lines()
        .map(|input| input.unwrap().bytes().map(|b| (b - b'0') as u32).collect())
        .collect();
    println!("{}", low_point_risk_level(&map));
}

