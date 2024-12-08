use std::io::{self, BufRead};
use std::collections::HashMap;

type Map = Vec<Vec<char>>;

fn find_antennas(map: &Map) -> HashMap<char, Vec<(i32, i32)>> {
    let mut antennas = HashMap::new();
    for (y, row) in map.iter().enumerate() {
        for (x, &ch) in row.iter().enumerate() {
            if ch != '.' {
                antennas.entry(ch).or_insert(Vec::new()).push((x as i32, y as i32));
            }
        }
    }
    antennas
}

fn put(antinodes: &mut Vec<Vec<bool>>, x: i32, y: i32) -> bool {
    if x >= 0 && x < antinodes[0].len() as i32 && y >= 0 && y < antinodes.len() as i32 {
        antinodes[y as usize][x as usize] = true;
        true
    } else {
        false
    }
}

fn unique_antinodes(map: &Map) -> usize {
    let mut antinodes = vec![vec![false; map[0].len()]; map.len()];

    for (_kind, locations) in find_antennas(map) {
        for (i, a) in locations.iter().enumerate() {
            for b in locations.iter().skip(i + 1) {
                let (dx, dy) = (b.0 - a.0, b.1 - a.1);
                put(&mut antinodes, a.0 - dx, a.1 - dy);
                put(&mut antinodes, b.0 + dx, b.1 + dy);
            }
        }
    }

    antinodes.into_iter()
        .map(|row| row.into_iter().filter(|&x| x).count())
        .sum()
}

fn unique_resonant_antinodes(map: &Map) -> usize {
    let mut antinodes = vec![vec![false; map[0].len()]; map.len()];

    for (_kind, locations) in find_antennas(map) {
        for (i, a) in locations.iter().enumerate() {
            for b in locations.iter().skip(i + 1) {
                let (dx, dy) = (b.0 - a.0, b.1 - a.1);
                for mult in 1.. {
                    let (ddx, ddy) = (mult * dx, mult * dy);
                    let a_ok = put(&mut antinodes, a.0 + ddx, a.1 + ddy);
                    let b_ok = put(&mut antinodes, b.0 - ddx, b.1 - ddy);
                    if !a_ok && !b_ok {
                        break;
                    }
                }
            }
        }
    }

    antinodes.into_iter()
        .map(|row| row.into_iter().filter(|&x| x).count())
        .sum()
}

fn main() {
    let map: Map = io::stdin().lock().lines()
        .map(|line| line.unwrap()
             .chars().collect()
            ).collect();
    println!("{}", unique_antinodes(&map));
    println!("{}", unique_resonant_antinodes(&map));
}
