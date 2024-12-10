use std::io::{self, BufRead};
use std::collections::HashSet;

struct Map(Vec<Vec<i32>>);
type Pos = (i32, i32);

impl Map {
    fn new(v: Vec<Vec<i32>>) -> Self {
        Self(v)
    }
    fn w(&self) -> i32 {
        self.0[0].len() as i32
    }
    fn h(&self) -> i32 {
        self.0.len() as i32
    }
    fn at(&self, p: Pos) -> Option<i32> {
        if p.0 >= 0 && p.0 < self.w() && p.1 >= 0 && p.1 < self.h() {
            Some(self.0[p.1 as usize][p.0 as usize])
        } else {
            None
        }
    }
}

fn search(map: &Map, pos: Pos, target: i32, visits: &mut HashSet<Pos>) -> i32 {
    if !visits.insert(pos) {
        return 0;
    }

    if let Some(h) = map.at(pos) {
        if h == 9 {
            1
        } else {
            let (x, y) = pos;
            [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)].into_iter()
                .map(|neigh| {
                    if map.at(neigh) == Some(target) {
                        search(map, neigh, target + 1, visits)
                    } else {
                        0
                    }
                })
            .sum()
        }
    } else {
        0
    }
}

fn score(map: &Map, p: Pos) -> i32 {
    search(map, p, 1, &mut HashSet::new())
}

fn trailhead_scores(map: &Map) -> i32 {
    let mut total = 0;
    for (y, row) in map.0.iter().enumerate() {
        for (x, &h) in row.iter().enumerate() {
            if h == 0 {
                total += score(map, (x as i32, y as i32));
            }
        }
    }
    total
}

fn search2(map: &Map, pos: Pos, target: i32, visits: &mut HashSet<Pos>) -> i32 {
    if let Some(h) = map.at(pos) {
        if h == 9 {
            1
        } else {
            let (x, y) = pos;
            [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)].into_iter()
                .map(|neigh| {
                    if map.at(neigh) == Some(target) {
                        search2(map, neigh, target + 1, visits)
                    } else {
                        0
                    }
                })
            .sum()
        }
    } else {
        0
    }
}

fn rating(map: &Map, p: Pos) -> i32 {
    search2(map, p, 1, &mut HashSet::new())
}

fn trailhead_ratings(map: &Map) -> i32 {
    let mut total = 0;
    for (y, row) in map.0.iter().enumerate() {
        for (x, &h) in row.iter().enumerate() {
            if h == 0 {
                total += rating(map, (x as i32, y as i32));
            }
        }
    }
    total
}

fn main() {
    let map = Map::new(io::stdin().lock().lines()
        .map(|line| line.unwrap()
             .bytes().map(|c| (c - b'0') as i32).collect::<Vec<_>>()
            ).collect::<Vec<_>>());
    println!("{}", trailhead_scores(&map));
    println!("{}", trailhead_ratings(&map));
}
