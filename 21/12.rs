use std::io::{self, BufRead};
use std::collections::HashSet;
use std::collections::HashMap;

type Node = String;

struct Graph {
    nodes: HashSet<Node>,
    edges: HashMap<Node, HashSet<Node>>,
}

fn is_small_cave(cave: &str) -> bool {
    cave.chars().all(|c| c.is_lowercase())
}

impl Graph {
    fn from_spec(spec: &[String]) -> Graph {
        let mut g = Graph { nodes: HashSet::new(), edges: HashMap::new() };
        for edge in spec {
            let mut sp = edge.split('-');
            let na = sp.next().unwrap().to_owned();
            let nb = sp.next().unwrap().to_owned();
            g.nodes.insert(na.clone());
            g.nodes.insert(nb.clone());
            g.edges.entry(na.clone()).or_insert(HashSet::new()).insert(nb.clone());
            g.edges.entry(nb).or_insert(HashSet::new()).insert(na);
        }
        g
    }
}

fn traverse(sys: &Graph, from: &Node, mut visited: HashSet<Node>, paths: &mut usize, level: usize) {
    //println!("{:-<width$}in {} paths {}", '-', from, paths, width=4*level);
    if from == "end" {
        *paths += 1;
        return;
    }
    if is_small_cave(from) && visited.contains(from) {
        // visiting small caves more than once is not allowed
        return;
    }
    visited.insert(from.to_owned());
    for to in sys.edges.get(from).unwrap() {
        traverse(sys, to, visited.clone(), paths, level + 1);
    }
}

fn traverse_flexibly(sys: &Graph, from: &Node, mut visited: HashSet<Node>, mut twice_visited: Option<Node>, paths: &mut usize, level: usize) {
    //println!("{:-<width$}in {} paths {}", '-', from, paths, width=4*level);
    if from == "end" {
        *paths += 1;
        return;
    }
    if is_small_cave(from) && visited.contains(from) {
        if twice_visited.is_some() || from == "start" {
            // visiting just one small cave except start twice is allowed
            return;
        } else {
            twice_visited = Some(from.clone());
        }
    }
    visited.insert(from.to_owned());
    for to in sys.edges.get(from).unwrap() {
        traverse_flexibly(sys, to, visited.clone(), twice_visited.clone(), paths, level + 1);
    }
}

fn suitable_paths(sys: &Graph) -> usize {
    let mut paths = 0;
    traverse(sys, &"start".to_string(), HashSet::new(), &mut paths, 0);
    paths
}

fn suitable_paths_one_double(sys: &Graph) -> usize {
    let mut paths = 0;
    traverse_flexibly(sys, &"start".to_string(), HashSet::new(), None, &mut paths, 0);
    paths
}

fn main() {
    let spec: Vec<_> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect();
    let sys = Graph::from_spec(&spec);
    println!("{:?}", suitable_paths(&sys));
    println!("{:?}", suitable_paths_one_double(&sys));
}
