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

fn right(p: Pos) -> Pos {
    (-p.1, p.0)
}

fn left(p: Pos) -> Pos {
    right(right(right(p)))
}

// pose to cost
type Distances = HashMap<(Pos, Pos), usize>;
// backwards to start
type Edges = HashMap<(Pos, Pos), HashSet<(Pos, Pos)>>;

fn dijkstra(map: &Map, start: (Pos, Pos)) -> (Distances, Edges) {
    let mut heap: BinaryHeap::<(Reverse<usize>, (Pos, Pos))> = BinaryHeap::new(); // dist, pose
    let mut distances = Distances::new();
    let mut edges = Edges::new();
    heap.push((Reverse(0), start));
    distances.insert(start, 0);

    while let Some(current) = heap.pop() {
        let (Reverse(dist_i), (pi, di)) = current;

        let mut run = |pj: Pos, dj: Pos, dist: usize| {
            if dist <= *distances.get(&(pj, dj)).unwrap_or(&std::usize::MAX) {
                heap.push((Reverse(dist), (pj, dj)));
                distances.insert((pj, dj), dist);
                edges.entry((pj, dj)).or_insert(HashSet::new()).insert((pi, di));
            }
        };

        if map[add(pi, di)] != '#' {
            run(add(pi, di), di, dist_i + 1);
        }
        run(pi, left(di), dist_i + 1000);
        run(pi, right(di), dist_i + 1000);
    }
    (distances, edges)
}

fn find(map: &Map) -> (Distances, Edges) {
    let start = map.iter().find(|&(_, ch)| ch == 'S').unwrap().0;
    // starts east, end doesn't have a favorable heading
    dijkstra(&map, (start, (1, 0)))
}

fn lowest_score(map: &Map, distances: &Distances) -> usize {
    let end = map.iter().find(|&(_, ch)| ch == 'E').unwrap().0;
    let ends = [(end, (-1, 0)), (end, (1, 0)), (end, (0, -1)), (end, (0, 1))];
    ends.into_iter().map(|e| *distances.get(&e).unwrap()).min().unwrap()
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

// this can't get in a cycle because that dijkstra gives us directions and dists thusly
fn tiles(p: (Pos, Pos), best: usize, edges: &Edges, visited: &mut HashSet<Pos>, distances: &Distances) {
    visited.insert(p.0);
    // must have distances because this path leads to end by definition
    let dist = *distances.get(&p).unwrap();
    if let Some(next) = edges.get(&p) {
        for &q in next {
            let q_dist = *distances.get(&q).unwrap();
            let edge_cost = if p.0 == q.0 { 1000 } else { 1 };
            // valid move on the shortest path? don't take detours and then join back again
            if dist == q_dist + edge_cost {
                tiles(q, best, edges, visited, distances);
            }
        }
    }
}

fn best_paths_tiles(map: &Map, distances: &Distances, edges: &Edges) -> usize {
    let end = map.iter().find(|&(_, ch)| ch == 'E').unwrap().0;
    let ends = [(end, (-1, 0)), (end, (1, 0)), (end, (0, -1)), (end, (0, 1))];
    let (best, endpose) = ends.into_iter().map(|e| (*distances.get(&e).unwrap(), e)).min().unwrap();
    let mut ts = HashSet::new();
    // this search stops at the start node that doesn't have edges anymore
    tiles(endpose, best, &edges, &mut ts, &distances);
    if false { dump(map, &ts); }
    ts.len()
}

fn main() {
    let map = Map(io::stdin().lock().lines()
        .map(|line| line.unwrap().chars().collect())
        .collect());
    let (distances, edges) = find(&map);
    println!("{:?}", lowest_score(&map, &distances));
    println!("{:?}", best_paths_tiles(&map, &distances, &edges));
}
