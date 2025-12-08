use std::io::{self, BufRead};
use std::collections::{HashSet, HashMap};

type Point = (i64, i64, i64);

fn distance(a: Point, b: Point) -> i64 {
    // no sqrt -- just ordering required
    let d = (a.0 - b.0, a.1 - b.1, a.2 - b.2);
    d.0 * d.0 + d.1 * d.1 + d.2 * d.2
}

fn join(adjmap: &HashMap<usize, Vec<usize>>, visited: &mut HashSet<usize>, i: usize) {
    if !visited.insert(i) {
        return;
    } else {
        for &j in adjmap.get(&i).unwrap() {
            join(adjmap, visited, j);
        }
    }
}

fn three_largest_mul(boxes: &[Point], pairings: usize) -> i64 {
    let mut pairs = boxes.iter()
        .enumerate()
        .flat_map(|(i, &bi)| {
            boxes.iter()
                .enumerate()
                .skip(i + 1)
                .map(move |(j, &bj)| (distance(bi, bj), i, j))
        })
        .collect::<Vec<(i64, usize, usize)>>();
    pairs.sort_unstable();
    let pairs = pairs;
    let mut adjmap = HashMap::new();
    for &(_di, i, j) in pairs.iter().take(pairings) {
        adjmap.entry(i)
            .and_modify(|neighs: &mut Vec<_>| neighs.push(j))
            .or_insert(vec![j]);
        adjmap.entry(j)
            .and_modify(|neighs: &mut Vec<_>| neighs.push(i))
            .or_insert(vec![i]);
    }
    let mut groups = HashSet::new();
    for (i, _js) in &adjmap {
        let mut visits = HashSet::new();
        join(&adjmap, &mut visits, *i);
        let mut visits_v = visits.drain().collect::<Vec<_>>();
        visits_v.sort_unstable();
        groups.insert(visits_v);
    }
    let mut lengths = groups.drain()
        .map(|g| -(g.len() as i64))
        .collect::<Vec<_>>();
    lengths.sort_unstable();
    -lengths[0] * lengths[1] * lengths[2]
}

fn parse(line: &str) -> Point {
    let mut sp = line.split(',');
    (sp.next().unwrap().parse().unwrap(),
    sp.next().unwrap().parse().unwrap(),
    sp.next().unwrap().parse().unwrap())
}

fn main() {
    let boxes = io::stdin().lock().lines()
        .map(|line| parse(&line.unwrap()))
        .collect::<Vec<_>>();
    println!("{:?}", three_largest_mul(&boxes, 10));
    println!("{:?}", three_largest_mul(&boxes, 1000));
}
