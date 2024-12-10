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
    fn iter(&self) -> impl Iterator<Item = (Pos, i32)> + '_ {
        self.0.iter().enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(move |(x, &h)| ((x as i32, y as i32), h))
            })
    }
}

trait Mem {
    fn insert(&mut self, pos: Pos) -> bool;
}

#[derive(Clone)]
struct MemSet(HashSet<Pos>);
impl Mem for MemSet {
    fn insert(&mut self, pos: Pos) -> bool {
        self.0.insert(pos)
    }
}

#[derive(Clone)]
struct MemNop;
impl Mem for MemNop {
    fn insert(&mut self, _: Pos) -> bool {
        true
    }
}

fn search(map: &Map, pos: Pos, target: i32, visits: &mut dyn Mem) -> i32 {
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

fn trailhead_measure<T: Mem + Clone>(map: &Map, mem: T) -> i32 {
    map.iter()
        .filter(|&(_, h)| h == 0)
        .map(|(pos, _)| search(map, pos, 1, &mut mem.clone()))
        .sum()
}

fn main() {
    let map = Map::new(io::stdin().lock().lines()
        .map(|line| line.unwrap()
             .bytes().map(|c| (c - b'0') as i32).collect::<Vec<_>>()
            ).collect::<Vec<_>>());
    println!("{}", trailhead_measure(&map, MemSet(HashSet::new())));
    println!("{}", trailhead_measure(&map, MemNop));
}
