use std::io::{self, BufRead};

type Map = Vec<(i64, i64)>;

fn expand(map: &mut Map, n: i64) {
    let maxx = map.iter().map(|&(x, _)| x).max().unwrap();
    let maxy = map.iter().map(|&(_, y)| y).max().unwrap();
    for x in (0..=maxx).rev() {
        if !map.iter().any(|&(xi, _)| xi == x) {
            map.iter_mut().filter(|(xi, _)| *xi > x).for_each(|p| p.0 += n);
        }
    }
    for y in (0..=maxy).rev() {
        if !map.iter().any(|&(_, yi)| yi == y) {
            map.iter_mut().filter(|(_, yi)| *yi > y).for_each(|p| p.1 += n);
        }
    }
}

fn short_paths(map: &Map) -> i64 {
    let mut sum = 0;
    for (i, a) in map.iter().enumerate() {
        for b in map.iter().skip(i + 1) {
            sum += (a.0 - b.0).abs() + (a.1 - b.1).abs();
        }
    }
    sum
}

fn main() {
    let mut map: Map = io::stdin().lock().lines()
        .enumerate()
        .fold(Map::new(), |mut map, (y, line)| {
            line.unwrap().chars().enumerate().for_each(|(x, ch)| {
                if ch == '#' {
                    map.push((x as i64, y as i64));
                }
            });
            map
        });
    let mut map2 = map.clone();
    let mut map3 = map.clone();
    expand(&mut map, 1);
    println!("{}", short_paths(&map));
    expand(&mut map2, 9);
    println!("10x: {}", short_paths(&map2));
    expand(&mut map3, 1_000_000 - 1);
    println!("{}", short_paths(&map3));
}
