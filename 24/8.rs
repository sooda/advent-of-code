use std::io::{self, BufRead};
use std::collections::HashMap;

type Map = Vec<Vec<char>>;

fn unique_antinodes(map: &Map) -> usize {
    let mut antinodes = vec![vec![false; map[0].len()]; map.len()];
    let mut antennas = HashMap::<char, Vec<(i32, i32)>>::new();
    let mut put = |x, y| {
        if x >= 0 && x < antinodes[0].len() as i32 && y >= 0 && y < antinodes.len() as i32 {
            antinodes[y as usize][x as usize] = true;
        }
    };

    for (y, row) in map.iter().enumerate() {
        for (x, &ch) in row.iter().enumerate() {
            if ch != '.' {
                antennas.entry(ch).or_insert(Vec::new()).push((x as i32, y as i32));
            }
        }
    }

    for (_kind, locations) in &antennas {
        for (i, a) in locations.iter().enumerate() {
            for b in locations.iter().skip(i + 1) {
                let (dx, dy) = (b.0 - a.0, b.1 - a.1);
                put(a.0 - dx, a.1 - dy);
                put(b.0 + dx, b.1 + dy);
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
}
