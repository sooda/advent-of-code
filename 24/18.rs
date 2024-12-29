#![feature(let_chains)]

use std::io::{self, BufRead};
use std::collections::{HashSet, HashMap, BinaryHeap};
use std::cmp::Reverse;

type Data = bool;
type Pos = (i32, i32);

fn add(a: Pos, b: Pos) -> Pos {
    (a.0 + b.0, a.1 + b.1)
}

struct Map(HashSet<Pos>, i32);

impl Map {
    fn new(positions: &[Pos], size: i32) -> Self {
        Self(positions.iter().copied().collect(), size)
    }
    fn at(&self, p: Pos) -> Option<Data> {
        if p.0 < 0 || p.0 >= self.1 || p.1 < 0 || p.1 >= self.1 {
            None
        } else {
            Some(self.0.contains(&p))
        }
    }
}

type Distances = HashMap<Pos, usize>;

fn dijkstra(map: &Map, start: Pos, end: Pos) -> Distances {
    let mut heap: BinaryHeap::<(Reverse<usize>, Pos)> = BinaryHeap::new();
    let mut distances = Distances::new();
    heap.push((Reverse(0), start));
    distances.insert(start, 0);

    while let Some(current) = heap.pop() {
        let (Reverse(dist_i), pi) = current;
        if pi == end {
            break;
        }

        let mut run = |pj: Pos, dist: usize| {
            if dist < *distances.get(&pj).unwrap_or(&std::usize::MAX) {
                heap.push((Reverse(dist), pj));
                distances.insert(pj, dist);
            }
        };

        for d in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            if let Some(cell) = map.at(add(pi, d)) && !cell {
                run(add(pi, d), dist_i + 1);
            }
        }
    }

    distances
}

fn steps_after_fall(positions: &[Pos], size: i32, simulation: usize) -> Option<usize> {
    let map = Map::new(&positions[0..simulation.min(positions.len())], size);
    let end = (size - 1, size - 1);
    let distances = dijkstra(&map, (0, 0), end);
    distances.get(&end).map(|n| *n)
}

fn first_blocking_fall(positions: &[Pos], size: i32) -> Option<Pos> {
    let end = (size - 1, size - 1);
    if dijkstra(&Map::new(&positions, size), (0, 0), end).contains_key(&end) {
        // sample input on big map
        return None;
    }
    let mut lo = 0;
    let mut hi = positions.len() - 1;
    // binary search a space where under X a path is found and at or over X a path is not found
    // YYYNNN
    //   LH   at the edge, and lo = mid + 1 becomes the first N
    while lo <= hi {
        let mid = (lo + hi) / 2;
        let map = Map::new(&positions[0..=mid], size);
        let path_found = dijkstra(&map, (0, 0), end).contains_key(&end);
        if path_found {
            lo = mid + 1;
        } else {
            hi = mid - 1;
        }
    }
    Some(positions[lo])
}

fn parse(line: &str) -> Pos {
    let mut sp = line.split(',');
    (sp.next().unwrap().parse().unwrap(),
     sp.next().unwrap().parse().unwrap())
}

fn main() {
    let positions = io::stdin().lock().lines()
        .map(|line| parse(&line.unwrap()))
        .collect::<Vec<_>>();
    println!("sample map {:?}", steps_after_fall(&positions, 7, 12));
    println!("sample map {:?}", first_blocking_fall(&positions, 7));
    println!("{:?}", steps_after_fall(&positions, 71, 1024));
    println!("{:?}", first_blocking_fall(&positions, 71));
}
