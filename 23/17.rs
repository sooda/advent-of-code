use std::io::{self, BufRead};
use std::collections::{HashMap, BinaryHeap};
use std::cmp::Reverse;

type Coords = (i32, i32);

type Map = HashMap<Coords, u8>;

fn cw(pos: Coords) -> Coords {
    (-pos.1, pos.0)
}

fn ccw(pos: Coords) -> Coords {
    (pos.1, -pos.0)
}

fn sum(a: Coords, b: Coords) -> Coords {
    (a.0 + b.0, a.1 + b.1)
}

fn _diff(a: Coords, b: Coords) -> Coords {
    (a.0 - b.0, a.1 - b.1)
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
struct Score {
    // incurred loss, smaller better
    heat_loss: Reverse<i32>,
    // heuristic distance, not actual; just to make the heap prioritize spots close to goal
    dist_to_goal: Reverse<i32>,
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
struct Node {
    pos: Coords,
    heading: Coords,
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
struct State {
    score: Score,
    node: Node,
}

// in manhattan
fn distance(a: Coords, b: Coords) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn walk(map: &Map, mut pos: Coords, delta: Coords, i: i32) -> i32 {
    let mut result = 0;
    for _ in 0..i {
        pos = sum(pos, delta);
        result += *map.get(&pos).unwrap() as i32;
    }
    result
}

fn dijkstra(map: &Map, entry_node: Node, exit_pos: Coords, minstraight: i32, maxstraight: i32) -> i32 {
    let entry_pos = entry_node.pos;
    let startscore = Score { heat_loss: Reverse(0), dist_to_goal: Reverse(distance(exit_pos, entry_pos)) };
    let startnode = entry_node;
    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    let mut parent = HashMap::new();
    let mut dists = HashMap::new();
    heap.push(State { score: startscore, node: startnode });
    dists.insert(startnode, 0);

    let mut push = |h: &mut BinaryHeap<_>, dists: &mut HashMap<_, _>, state: State, prev: Node| {
        let score = state.score;
        let node = state.node;

        dists.insert(node, score.heat_loss.0);
        parent.insert(node, prev);
        h.push(state);
    };

    while let Some(state) = heap.pop() {
        let (score, node) = (state.score, state.node);

        if score.dist_to_goal.0 == 0 {
            // optimal search, the first found goal is the best one
            if false {
                let mut pos = node;
                while let Some(&next) = parent.get(&pos) {
                    println!("pos {:?}", pos);
                    pos = next;
                }
            }
            return score.heat_loss.0;
        }

        let next_dir = [cw(node.heading), ccw(node.heading)];
        for dir in next_dir {
            for i in minstraight..=maxstraight {
                let next_pos = sum(node.pos, (i * dir.0, i * dir.1));
                let next_node = Node { pos: next_pos, heading: dir };
                if let Some(&_edge_cost) = map.get(&next_pos) {
                    let next_loss = score.heat_loss.0 + walk(map, node.pos, dir, i);
                    if *dists.get(&next_node).unwrap_or(&std::i32::MAX) <= next_loss {
                        continue;
                    }
                    push(&mut heap, &mut dists, State {
                        score: Score {
                            heat_loss: Reverse(next_loss),
                            dist_to_goal: Reverse(distance(exit_pos, next_pos)),
                        },
                        node: next_node,
                    }, node);
                }
            }
        }
    }

    panic!("no route");
}

fn least_heat_loss(map: &Map) -> i32 {
    let maxx = map.keys().map(|p| p.0).max().unwrap();
    let maxy = map.keys().map(|p| p.1).max().unwrap();
    // FIXME: add both to starting set. this is fast enough now
    let horiz = dijkstra(map, Node { pos: (0, 0), heading: (1, 0) }, (maxx, maxy), 1, 3);
    let verti = dijkstra(map, Node { pos: (0, 0), heading: (0, 1) }, (maxx, maxy), 1, 3);
    horiz.min(verti)
}

fn least_heat_loss_ultra(map: &Map) -> i32 {
    let maxx = map.keys().map(|p| p.0).max().unwrap();
    let maxy = map.keys().map(|p| p.1).max().unwrap();
    let horiz = dijkstra(map, Node { pos: (0, 0), heading: (1, 0) }, (maxx, maxy), 4, 10);
    let verti = dijkstra(map, Node { pos: (0, 0), heading: (0, 1) }, (maxx, maxy), 4, 10);
    horiz.min(verti)
}

fn parse_map(rows: &[String]) -> Map {
    rows.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.as_bytes().iter()
                .enumerate()
                .map(move |(x, &b)| ((x as i32, y as i32), b - b'0'))
        })
    .collect::<Map>()
}

fn main() {
    let rows = io::stdin().lock().lines()
        .map(|r| r.unwrap())
        .collect::<Vec<_>>();
    let map = parse_map(&rows);
    println!("{}", least_heat_loss(&map));
    println!("{}", least_heat_loss_ultra(&map));
}
