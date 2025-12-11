use std::io::{self, BufRead};
use std::collections::HashMap;

type Net = HashMap<String, Vec<String>>;

fn find_paths<'a>(net: &'a Net, node: &'a str, end: &str, paths: &mut HashMap<&'a str, usize>) -> usize {
    if node == end {
        1
    } else if let Some(&n) = paths.get(node) {
        n
    } else if let Some(edges) = net.get(node) {
        let n = edges.iter().map(|e| find_paths(net, e, end, paths)).sum();
        paths.insert(node, n);
        n
    } else {
        0
    }
}

fn total_paths(net: &Net) -> usize {
    let mut paths = HashMap::new();
    find_paths(net, "you", "out", &mut paths);
    *paths.get("you").expect("no paths to out?")
}

fn parse(line: &str) -> (String, Vec<String>) {
    let (node, edgestr) = line.split_once(": ").unwrap();
    let edges = edgestr.split(' ').map(|e| e.to_string()).collect();
    (node.to_string(), edges)
}

fn main() {
    let net: Net = io::stdin().lock().lines()
        .map(|line| parse(&line.unwrap())
            ).collect();
    println!("{}", total_paths(&net));
}
