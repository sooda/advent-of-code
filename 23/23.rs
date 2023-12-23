use std::io::{self, BufRead};
use std::collections::{HashMap, HashSet};

type TileMap = Vec<Vec<char>>;

type Coord = (i32, i32);

type Graph = HashMap::<Coord, Vec<(Coord, usize)>>;

fn sum(a: Coord, b: Coord) -> Coord {
    (a.0 + b.0, a.1 + b.1)
}

fn dfs(map: &TileMap, prev_node: Coord, since_prev: usize, pos: Coord, end_node: Coord, graph: &mut Graph, visited: &mut HashSet<(Coord, Coord)>) {
    if !visited.insert((pos, prev_node)) {
        return;
    }
    // -> (ok this direction, some pathway)
    let ok = |p: Coord, d: Coord| {
        if p.0 < 0 || p.0 >= map[0].len() as i32 || p.1 < 0 || p.1 >= map.len() as i32 {
            (false, false)
        } else {
            match map[p.1 as usize][p.0 as usize] {
                '#' => (false, false),
                '.' => (true, true),
                '<' => (d == (-1,  0), true),
                '>' => (d == ( 1,  0), true),
                '^' => (d == ( 0, -1), true),
                'v' => (d == ( 0,  1), true),
                _ => panic!(),
            }
        }
    };
    let ds = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let num_neighs = ds.into_iter().filter(|&d| ok(sum(pos, d), d).1).count();
    let intersection_node = num_neighs > 2;
    let (prev_node, since_prev) = if intersection_node || pos == end_node {
        graph.entry(prev_node).or_insert(Vec::new()).push((pos, since_prev));
        (pos, 1)
    } else {
        // just follow the road
        (prev_node, since_prev + 1)
    };
    for (newpos, _) in ds.into_iter().map(|d| (sum(pos, d), d)).filter(|&(p, d)| ok(p, d).0) {
        if newpos != prev_node {
            dfs(map, prev_node, since_prev, newpos, end_node, graph, visited);
        }
    }
}

// lots of redoing but the graph is so small that couldn't be bothered to cache
fn max_distance(graph: &Graph, current: Coord, end: Coord) -> usize {
    if current == end {
        0
    } else {
        graph.get(&current).unwrap()
            .iter()
            .map(|&(neighnode, dist)| {
                dist + max_distance(graph, neighnode, end)
            })
            .max()
            .unwrap()
    }
}

fn longest_hike(map: &TileMap) -> usize {
    let start = (1, 0);
    let end = (map[0].len() as i32 - 2, map.len() as i32 - 1);
    let mut graph = Graph::new();
    let mut visited = HashSet::new();
    dfs(map, start, 0, start, end, &mut graph, &mut visited);
    max_distance(&graph, start, end)
}

fn main() {
    let tiles = io::stdin().lock().lines()
        .map(|row| row.unwrap().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    println!("{}", longest_hike(&tiles));
}
