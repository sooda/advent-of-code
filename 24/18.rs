#![feature(let_chains)]

use std::io::{self, BufRead};
use std::ops::{Index, IndexMut};
use std::collections::{HashMap, BinaryHeap};
use std::cmp::Reverse;

type Data = bool;
#[derive(Clone)]
struct Map(Vec<Vec<Data>>);
type Pos = (i32, i32);

impl Map {
    fn w(&self) -> i32 {
        self.0[0].len() as i32
    }
    fn h(&self) -> i32 {
        self.0.len() as i32
    }
    fn at(&self, p: Pos) -> Option<&Data> {
        if p.0 >= 0 && p.0 < self.w() && p.1 >= 0 && p.1 < self.h() {
            Some(&self.0[p.1 as usize][p.0 as usize])
        } else {
            None
        }
    }
    fn at_mut(&mut self, p: Pos) -> Option<&mut Data> {
        if p.0 >= 0 && p.0 < self.w() && p.1 >= 0 && p.1 < self.h() {
            Some(&mut self.0[p.1 as usize][p.0 as usize])
        } else {
            None
        }
    }
}

impl Index<Pos> for Map {
    type Output = Data;
    fn index(&self, p: Pos) -> &Self::Output {
        self.at(p).unwrap()
    }
}

impl IndexMut<Pos> for Map {
    fn index_mut(&mut self, p: Pos) -> &mut Data {
        self.at_mut(p).unwrap()
    }
}

fn add(a: Pos, b: Pos) -> Pos {
    (a.0 + b.0, a.1 + b.1)
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
    let mut map = Map(vec![vec![false; size as usize]; size as usize]);
    for &p in positions.iter().take(simulation) {
        if p.0 >= size || p.1 >= size {
            // size 7 with real input
            return None;
        }
        map[p] = true;
    }
    let end = (size - 1, size - 1);
    let distances = dijkstra(&map, (0, 0), end);
    distances.get(&end).map(|n| *n)
}

fn first_blocking_fall(positions: &[Pos], size: i32) -> Option<Pos> {
    let mut map = Map(vec![vec![false; size as usize]; size as usize]);
    let end = (size - 1, size - 1);
    for &p in positions {
        map[p] = true;
        let distances = dijkstra(&map, (0, 0), end);
        if !distances.contains_key(&end) {
            return Some(p);
        }
    }
    // sample input
    None
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
    println!("{:?}", steps_after_fall(&positions, 7, 12));
    println!("{:?}", steps_after_fall(&positions, 71, 1024));
    println!("{:?}", first_blocking_fall(&positions, 71));
}
