use std::io::{self, BufRead};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Reverse;

type Pos = u32;

// pose to cost
type Distances = HashMap<Pos, usize>;
// backwards to start
type Edges = HashMap<Pos, HashSet<Pos>>;

fn dijkstra(buttons: &[u32], start: Pos) -> (Distances, Edges) {
    let mut heap: BinaryHeap::<(Reverse<usize>, Pos)> = BinaryHeap::new(); // dist, pose
    let mut distances = Distances::new();
    let mut edges = Edges::new();
    heap.push((Reverse(0), start));
    distances.insert(start, 0);

    while let Some(current) = heap.pop() {
        let (Reverse(dist_i), pi) = current;

        let mut run = |pj: Pos, dist: usize| {
            if dist < *distances.get(&pj).unwrap_or(&std::usize::MAX) {
                heap.push((Reverse(dist), pj));
                distances.insert(pj, dist);
                edges.entry(pj).or_insert(HashSet::new()).insert(pi);
            }
        };

        for &button in buttons {
            run(pi ^ button, dist_i + 1);
        }
    }
    (distances, edges)
}

fn fewest_button_presses(lights: u32, buttons: &[u32]) -> usize {
    let (distances, _edges) = dijkstra(buttons, 0);
    *distances.get(&lights).expect("button sequence not found")
}

fn button_presses_summed(light_manual: &[(u32, Vec<u32>, Vec<u32>)]) -> usize {
    light_manual.iter().map(|l| fewest_button_presses(l.0, &l.1)).sum()
}

// [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
fn parse(line: &str) -> (u32, Vec<u32>, Vec<u32>) {
    let (_, lights) = line.split_once('[').unwrap();
    let (lights, rest) = lights.split_once("] ").unwrap();
    let (buttons, joltages) = rest.rsplit_once(' ').unwrap();
    let lights = lights.chars().enumerate().fold(0, |acc, (i, x)| match x {
        '.' => acc,
        '#' => acc | (1 << i),
        _ => panic!("bad input"),
    });
    let buttons = buttons.split(' ')
        .map(|button| {
            button.strip_prefix('(').unwrap()
                .strip_suffix(')').unwrap()
                .split(',')
                .map(|num| num.parse::<u32>().unwrap())
                .fold(0, |acc, num| acc | (1 << num))
        }).collect();
    let joltages = joltages.strip_prefix('{').unwrap()
                .strip_suffix('}').unwrap()
                .split(',')
                .map(|j| j.parse().unwrap())
                .collect();
    (lights, buttons, joltages)
}

fn main() {
    let light_manual = io::stdin().lock().lines()
        .map(|line| parse(&line.unwrap()))
        .collect::<Vec<_>>();
    println!("{}", button_presses_summed(&light_manual));
}
