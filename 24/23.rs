use std::io::{self, BufRead};
use std::collections::{HashMap, HashSet};

type Nodes<'a> = HashSet<&'a str>;
type Edges<'a> = HashMap<&'a str, Vec<&'a str>>;

fn graphize(connections: &[(String, String)]) -> (Nodes, Edges) {
    let mut nodes = HashSet::<&str>::new();
    let mut edges = HashMap::<&str, Vec<&str>>::new();
    for (a, b) in connections {
        nodes.insert(a);
        nodes.insert(b);
        edges.entry(a).or_insert(Vec::new()).push(b);
        edges.entry(b).or_insert(Vec::new()).push(a);
    }
    (nodes, edges)
}

fn three_sets_t(nodes: &Nodes, edges: &Edges) -> usize {
    let mut sets = HashSet::<[&str; 3]>::new();
    // for each x, find prev-x-next so that also prev-next is a link
    for &x in nodes {
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

fn password(nodes: &Nodes, edges: &Edges) -> String {
    let mut subgraphs = HashMap::new();
    for n in nodes {
        for i in 0..edges[n].len() {
            let mut subgraph = edges[n].clone();
            subgraph.remove(i);
            subgraph.push(n);
            subgraph.sort();
            *subgraphs.entry(subgraph).or_insert(0) += 1;
        }
    }

    subgraphs.into_iter()
        .max_by_key(|(_graph, count)| *count)
        .unwrap().0
        .join(",")
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
    let (nodes, edges) = graphize(&connections);
    println!("{}", three_sets_t(&nodes, &edges));
    println!("{}", password(&nodes, &edges));
}
