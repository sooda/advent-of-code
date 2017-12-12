use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

#[derive(Clone, Debug)]
struct Node {
    children: Vec<usize>
}

fn parse_line(line: &str) -> Node {
    // 1 <-> 1
    // 2 <-> 0, 3, 4
    let mut mapping = line.split(" <-> ");
    let _name = mapping.next(); // identical as the index of this node
    let children = mapping.next().unwrap()
        .split(", ").map(|i| i.to_string().parse().unwrap()).collect();

    Node { children: children }
}

fn group_count(nodes: &[Node], from: usize, visit_map: &mut [bool]) -> usize {
    if visit_map[from] {
        0
    } else {
        visit_map[from] = true;
        1 + nodes[from].children.iter()
            .map(|&c| group_count(nodes, c, visit_map)).sum::<usize>()
    }
}

fn zero_group_count(nodes: &[Node]) -> usize {
    let mut visited = vec![false; nodes.len()];
    group_count(nodes, 0, &mut visited)
}

fn main() {
    let nodes = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap())
        .lines().map(|x| parse_line(&x.unwrap())).collect::<Vec<_>>();
    println!("{:?}", nodes);
    println!("{}", zero_group_count(&nodes));
}
