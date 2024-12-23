use std::io::{self, BufRead};
use std::collections::{HashMap, HashSet};

fn three_sets_t(connections: &[(String, String)]) -> usize {
    let mut nodes = HashSet::<&str>::new();
    let mut edges = HashMap::<&str, Vec<&str>>::new();
    for (a, b) in connections {
        nodes.insert(a);
        nodes.insert(b);
        edges.entry(a).or_insert(Vec::new()).push(b);
        edges.entry(b).or_insert(Vec::new()).push(a);
    }

    let mut sets = HashSet::<[&str; 3]>::new();
    // for each x, find prev-x-next so that also prev-next is a link
    for &x in &nodes {
        for prev in &edges[x] {
            for next in &edges[x] {
                if edges[prev].contains(&next) {
                    if x.starts_with("t") || prev.starts_with("t") || next.starts_with("t") {
                        let mut them = [x, prev, next];
                        them.sort();
                        sets.insert(them);
                    }
                }
            }
        }
    }
    sets.len()
}

fn parse(line: &str) -> (String, String) {
    let mut sp = line.split('-');
    let a = sp.next().unwrap().to_string();
    let b = sp.next().unwrap().to_string();
    (a, b)
}

fn main() {
    let connections: Vec<_> = io::stdin().lock().lines()
        .map(|line| parse(&line.unwrap()))
        .collect();
    println!("{}", three_sets_t(&connections));
}
