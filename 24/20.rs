use std::io::{self, BufRead};
use std::ops::{Index, IndexMut};
use std::collections::{HashMap, HashSet, BinaryHeap};
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

// type State = (Pos, [bool; 2]);
#[derive(Eq, PartialEq, Copy, Clone, Hash, Ord, PartialOrd, Debug)]
struct State(Pos);
// pose to cost
type Distances = HashMap<State, usize>;
// backwards to start
type Edges = HashMap<State, HashSet<State>>;

fn dijkstra(map: &Map, start: Pos) -> (Distances, Edges) {
    let mut heap: BinaryHeap::<(Reverse<usize>, State)> = BinaryHeap::new();
    let mut distances = Distances::new();
    let mut edges = Edges::new();

    let start = State(start);
    heap.push((Reverse(0), start));
    distances.insert(start, 0);

    while let Some(current) = heap.pop() {
        let (Reverse(dist_i), state_i) = current;

        let mut run = |state_j: State, dist: usize| {
            if dist <= *distances.get(&state_j).unwrap_or(&std::usize::MAX) {
                heap.push((Reverse(dist), state_j));
                distances.insert(state_j, dist);
                edges.entry(state_j).or_insert(HashSet::new()).insert(state_i);
            }
        };

        for d in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let p = add(state_i.0, d);
            if map[p] != '#' {
                run(State(p), dist_i + 1);
            }
        }
    }
    (distances, edges)
}

fn goal_path(edges: Edges, end: Pos) -> Vec<Pos> {
    let mut ret = Vec::new();
    let mut current = end;
    while let Some(x) = edges.get(&State(current)) {
        ret.push(current);
        // this map is special and the path is trivially straight
        assert_eq!(x.len(), 1);
        current = x.iter().next().unwrap().0;
    }
    ret.push(current);
    ret
}

fn saving_cheats(map: &Map, maxlen: i32) -> usize {
    let start = map.iter().find(|&(_, ch)| ch == 'S').unwrap().0;
    let end = map.iter().find(|&(_, ch)| ch == 'E').unwrap().0;
    let (distances, edges) = dijkstra(&map, start);
    let path = goal_path(edges, end);

    let mut saves = HashMap::new(); // to match with the example
    let mut good_cheat_count = 0;

    for (i, &p1) in path.iter().enumerate() {
        for &p2 in path.iter().skip(i + 1) {
            let pair_distance = (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs();
            if pair_distance <= maxlen {
                let d1 = distances[&State(p1)] as i32;
                let d2 = distances[&State(p2)] as i32;
                let saved = (d1 - d2).abs() - pair_distance;
                if saved > 0 {
                    *saves.entry(saved).or_insert(0) += 1;
                    if saved >= 100 {
                        good_cheat_count += 1;
                        if false {
                            dump(&map, &[p1, p2].into_iter().collect());
                        }
                    }
                }
            }
        }
    }


    if map.w() == 15 { // sample input
        let mut saves = saves.iter().collect::<Vec<_>>();
        saves.sort();
        for s in saves {
            println!("{} cheats that save {} ps", s.1, s.0);
        }
    }

    good_cheat_count
}

fn dump(map: &Map, tiles: &HashSet<Pos>) {
    for (y, row) in map.0.iter().enumerate() {
        for (x, &ch) in row.iter().enumerate() {
            print!("{}", if tiles.contains(&(x as i32, y as i32)) { 'O' } else { ch });
        }
        println!();
    }
    println!();
}

fn main() {
    let map = Map(io::stdin().lock().lines()
        .map(|line| line.unwrap().chars().collect())
        .collect());

    println!("{:?}", saving_cheats(&map, 2));
    println!("{:?}", saving_cheats(&map, 20));
}
