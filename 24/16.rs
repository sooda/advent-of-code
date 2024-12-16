use std::io::{self, BufRead};
use std::ops::{Index, IndexMut};
use std::collections::{HashMap, BinaryHeap};
use std::cmp::Reverse;

type Data = char;
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
    fn iter(&self) -> impl Iterator<Item = (Pos, Data)> + '_ {
        self.0.iter().enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(move |(x, &h)| ((x as i32, y as i32), h))
            })
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

fn right(p: Pos) -> Pos {
    (-p.1, p.0)
}

fn left(p: Pos) -> Pos {
    right(right(right(p)))
}

fn dijkstra(map: &Map, start: (Pos, Pos), end: &[(Pos, Pos)]) -> usize {
    let mut heap: BinaryHeap::<(Reverse<usize>, (Pos, Pos))> = BinaryHeap::new(); // dist, pose
    let mut distances = HashMap::<(Pos, Pos), usize>::new(); // pose to cost
    heap.push((Reverse(0), start));

    while let Some(current) = heap.pop() {
        let (Reverse(dist_i), (pi, d)) = current;

        let mut run = |p: Pos, d: Pos, dist: usize| {
            if dist < *distances.get(&(p, d)).unwrap_or(&std::usize::MAX) {
                heap.push((Reverse(dist), (p, d)));
                distances.insert((p, d), dist);
            }
        };

        if map[add(pi, d)] != '#' {
            run(add(pi, d), d, dist_i + 1);
        }
        run(pi, left(d), dist_i + 1000);
        run(pi, right(d), dist_i + 1000);
    }

    end.into_iter().map(|e| *distances.get(&e).unwrap()).min().unwrap()
}

fn lowest_score(map: &Map) -> usize {
    let start = map.iter().find(|&(_, ch)| ch == 'S').unwrap().0;
    let end = map.iter().find(|&(_, ch)| ch == 'E').unwrap().0;
    // starts east, end doesn't have a favorable heading
    dijkstra(&map,
             (start, (1, 0)),
             &[(end, (-1, 0)), (end, (1, 0)), (end, (0, -1)), (end, (0, 1))])
}

fn main() {
    let map = Map(io::stdin().lock().lines()
        .map(|line| line.unwrap().chars().collect())
        .collect());
    println!("{:?}", lowest_score(&map));
}
