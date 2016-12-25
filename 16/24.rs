use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use std::collections::vec_deque::VecDeque;
use std::collections::HashSet;

extern crate permutohedron;
use permutohedron::Heap;

type Node = char;
const WALL: Node = '#';

struct GridGraph {
    map: Vec<Node>,
    w: i32,
    h: i32
}

type NodeId = u32;
type Depth = usize;

impl GridGraph {
    fn new(nodes: &[String]) -> Self {
        let map = nodes.iter().flat_map(|x| x.chars()).collect::<Vec<_>>();
        GridGraph { map: map, w: nodes[0].len() as i32, h: nodes.len() as i32 }
    }

    fn at(&self, x: i32, y: i32) -> NodeId {
        assert!(x < self.w);
        assert!(y < self.h);
        (y * self.w + x) as NodeId
    }

    fn neighbors(&self, nodeid: NodeId) -> Vec<(NodeId, Depth)> {
        let mut neighs = Vec::new();
        let x = nodeid as i32 % self.w;
        let y = nodeid as i32 / self.w;
        assert!(y < self.h);

        let moves = [(-1i32, 0i32), (0, 1), (1, 0), (0, -1)];
        for &(dx, dy) in moves.iter() {
            if dy == -1 && y == 0 { continue; }
            if dx == -1 && x == 0 { continue; }

            let xx = x + dx;
            let yy = y + dy;
            let pos = self.at(xx, yy);
            if self.map[pos as usize] != WALL {
                neighs.push((pos, 1));
            }
        }

        neighs
    }

    fn find(&self, needle: Node) -> NodeId {
        self.map.iter().position(|&n| n == needle).unwrap() as u32
    }
}

fn search(g: &GridGraph, root: NodeId, goals: &[NodeId]) -> Vec<Depth> {
    let mut results = vec![0; goals.len()];
    let mut found = 0;
    if let Some(_) = goals.iter().position(|&g| g == root) {
        // result already 0
        found += 1;
    }

    let mut visited = HashSet::new();
    visited.insert(root);

    let mut queue = VecDeque::new();
    queue.push_back((0, root));

    while let Some((steps, node)) = queue.pop_front() {
        //println!("at {:?} {:?}   {} {}", steps, node, node%11, node/11);
        // ugh. was going to generalize this and dijkstra this but nah, only need to actually
        // search in unweighted so dist is now always 1 from the grid graph
        for &(neigh, dist) in g.neighbors(node).iter() {
            //println!("  {:?}", neigh);
            if !visited.contains(&neigh) {
                if let Some(i) = goals.iter().position(|&g| g == neigh) {
                    results[i] = steps + dist;
                    found += 1;
                    //println!("yass {} {}", steps, found);
                    if found == results.len() {
                        return results;
                    }
                }
                visited.insert(neigh);
                queue.push_back((steps + dist, neigh));
            }
        }
    }
    unreachable!()
}

fn main() {
    // my input has just seven places to visit. number of permutations is 7! = 5040, cheap enough
    // to try them all, with some preprocessing involved first so we get the distances between them
    // into a graph for easy access

    // parse the input for a map graph
    let input = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap()).lines().map(Result::unwrap).collect::<Vec<_>>();

    // find the number of relevant locations where the exposed wires are located
    let row_numbers = |row: &String| row.chars().filter(|&c| c >= '1' && c <= '9').collect::<Vec<_>>();
    let numbers = input.iter().flat_map(row_numbers).collect::<Vec<_>>();
    let last_location = *numbers.iter().max().unwrap() as u32 - '0' as u32;
    println!("{:?}",last_location);

    // make an actual graph of the map
    let g = GridGraph::new(&input);
    let root = g.find('0');
    println!("{} {} {}", g.w, g.h, root);

    // find shortest paths between each location pair in single passes of bfs. this reduces the
    // grid map to a topological graph
    let locations = 0..last_location+1;
    let positions = locations.map(|l| g.find(('0' as u8 + l as u8) as char)).collect::<Vec<_>>();
    let distances = positions.iter().map(|&p| search(&g, p, &positions)).collect::<Vec<_>>();
    println!("{:?}", distances);

    let mut route = (1..last_location+1).collect::<Vec<_>>();

    // try each permutation of the places of interest in this smaller topological graph. each
    // position can be visited more than once, if necessary
    for &go_back_home in &[false, true] {
        let heap = Heap::new(&mut route);
        let mut permutations = Vec::new(); // meh. heap can't be .iter()'d
        for data in heap { permutations.push(data.clone()); }
        let min_path = permutations.iter().map(
            |candidate| {
                let mut steps = distances[0][candidate[0] as usize];
                let mut prev = candidate[0];
                for &node in &candidate[1..] {
                    steps += distances[prev as usize][node as usize];
                    prev = node;
                }
                if go_back_home {
                    steps += distances[prev as usize][0];
                }
                steps
            }).min();
        println!("{:?}", min_path);
    }
    // alternatively, could just bfs in a graph that has the visited places in the state too, with
    // a vector of all marked as visited as the goal node
}
