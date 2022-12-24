use std::io::{self, BufRead};
use std::collections::{HashMap, HashSet, BinaryHeap};
use std::cmp::Reverse;

// as the inner area in the picture: (0,0) is the top left ground in the valley
type Coords = (i32, i32);

fn sum(a: Coords, b: Coords) -> Coords {
    (a.0 + b.0, a.1 + b.1)
}

#[derive(Clone, Copy, Debug)]
enum Dir {
    Right,
    Left,
    Down,
    Up
}
use Dir::*;

// blizzards is impossible to type
type Blizs = HashMap<Coords, Dir>;

struct Map {
    blizs: Blizs,
    // not counting the wall tiles, so size of the inner area
    ground_size: Coords,
}


impl Map {
    fn bliz_at(&self, pos: Coords, dir: Dir, time: i32) -> Coords {
        let next = sum(match dir {
            Right => (pos.0 + time % self.ground_size.0, pos.1),
            Left => (pos.0 - time % self.ground_size.0, pos.1),
            Down => (pos.0, pos.1 + time % self.ground_size.1),
            Up => (pos.0, pos.1 - time % self.ground_size.1)
        }, self.ground_size);

        (next.0 % self.ground_size.0, next.1 % self.ground_size.1)
    }

    fn out_of_bounds(&self, pos: Coords) -> bool {
        pos.0 < 0 || pos.0 >= self.ground_size.0 || pos.1 < 0 || pos.1 >= self.ground_size.1
    }

    fn empty_cell(&self, pos: Coords, time: i32) -> bool {
        // FIXME: store per row and per col separately; needs more space but is faster to index
        let row_safe = self.blizs.iter()
            .filter(|(bpos, _)| bpos.1 == pos.1)
            .all(|(&bpos, &bdir)| self.bliz_at(bpos, bdir, time) != pos);
        let col_safe = self.blizs.iter()
            .filter(|(bpos, _)| bpos.0 == pos.0)
            .all(|(&bpos, &bdir)| self.bliz_at(bpos, bdir, time) != pos);
        row_safe && col_safe
    }

    fn print(&self, time: i32, expedition: Coords) {
        println!("minutes {}:", time);
        for y in 0..self.ground_size.1 {
            for x in 0..self.ground_size.0 {
                let pos = (x, y);
                let mut blizs_here = self.blizs.iter()
                    .filter(|&(&bpos, &bdir)| self.bliz_at(bpos, bdir, time) == pos);
                let num_blizs_here = blizs_here.clone().count();
                let ch = if pos == expedition {
                    'E'
                } else if num_blizs_here == 0 {
                    '.'
                } else if num_blizs_here == 1 {
                    let (_, dir) = blizs_here.next().unwrap();
                    match dir {
                        Right => '>',
                        Left => '<',
                        Down => 'v',
                        Up => '^',
                    }
                } else if num_blizs_here < 9 {
                    (b'0' + num_blizs_here as u8) as char
                } else {
                    '!'
                };
                print!("{}", ch);
            }
            println!();
        }
    }
}

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
struct Score {
    // heuristic distance, not actual; just to make the heap prioritize spots close to goal
    dist_to_goal: Reverse<i32>,
}

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
struct Node {
    minutes: Reverse<i32>, // consumed time; smaller is better
    pos: Coords, // expedition is too hard to type this morning
}

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
struct State {
    score: Score,
    node: Node,
}

// in manhattan
fn distance(a: Coords, b: Coords) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn dijkstra(map: &Map) -> i32 {
    let entry_pos = (0, -1);
    let exit_pos = (map.ground_size.0 - 1, map.ground_size.1);
    let debug = false;
    let startscore = Score { dist_to_goal: Reverse(distance(exit_pos, entry_pos)) };
    let startnode = Node { minutes: Reverse(0), pos: entry_pos };
    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    let mut visited = HashSet::new();
    heap.push(State { score: startscore, node: startnode });
    visited.insert(startnode);

    let mut push = |h: &mut BinaryHeap<_>, state: State, best| {
        let score = state.score;
        let node = state.node;

        // heuristic: can't possibly make it better?
        if node.minutes.0 + score.dist_to_goal.0 >= best {
            return;
        }

        if visited.insert(node) {
            h.push(state);
        }
    };

    // goal not yet discovered
    // some big number but still allows adding more
    let mut best_minutes = std::i32::MAX / 2;

    let next_delta = &[(0, 0), (1, 0), (-1, 0), (0, 1), (0, -1)];
    while let Some(state) = heap.pop() {
        let (score, node) = (state.score, state.node);
        if debug {
            println!("visit {:?} minutes {} goaldist {}",
                     node.pos, node.minutes.0, score.dist_to_goal.0);
            map.print(node.minutes.0, node.pos);
        }

        if score.dist_to_goal.0 == 0 {
            best_minutes = best_minutes.min(node.minutes.0);
            if debug {
                println!("found a goal of {}", best_minutes);
            }
            // no more traversal
            continue;
        }

        for nextpos in next_delta.iter().map(|&d| sum(node.pos, d)) {
            if map.out_of_bounds(nextpos) && !(
                    nextpos == exit_pos
                    || (node.minutes.0 == 0 && node.pos == entry_pos)) {
                continue;
            }

            if map.empty_cell(nextpos, node.minutes.0 + 1) {
                push(&mut heap, State {
                    score: Score {
                        dist_to_goal: Reverse(distance(exit_pos, nextpos)),
                    },
                    node: Node {
                        minutes: Reverse(node.minutes.0 + 1),
                        pos: nextpos,
                    }
                }, best_minutes);
            }
        }
    }

    best_minutes
}

fn fewest_minutes(map: &Map) -> i32 {
    dijkstra(map)
}

fn parse_map(lines: &[String]) -> Map {
    let (w, h) = (lines[0].len() as i32 - 2, lines.len() as i32 - 2);
    let blizs = lines.iter()
        .skip(1)
        .take(h as usize)
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .skip(1)
                .take(w as usize)
                .enumerate()
                .map(move |(x, ch)| ((x as i32, y as i32), ch))
        })
        .filter_map(|(pos, ch)| {
            match ch {
                '.' => None,
                _ => Some((pos, match ch {
                    '>' => Right,
                    '<' => Left,
                    'v' => Down,
                    '^' => Up,
                    _ => panic!("bad input")
                }))
            }
        })
        .collect();
    Map { blizs, ground_size: (w, h) }
}

fn main() {
    let lines: Vec<_> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect();
    let map = parse_map(&lines);
    if false {
        map.print(0, (0, -1));
    }
    println!("{}", fewest_minutes(&map));
}
