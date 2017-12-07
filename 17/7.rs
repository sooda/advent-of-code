use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Node<'a> {
    name: &'a str,
    weight: u32,
    children: Vec<usize>
}

struct NodeSpec {
    name: String,
    weight: u32,
    children: Vec<String>
}

fn parse_line(line: &str) -> NodeSpec {
    // tknk (41) -> ugml, padx, fwft
    // jptl (61)
    let mut mapping = line.split(" -> ");
    let name_children = mapping.next().unwrap();
    let mut name_weight = name_children.split(" ");
    let name = name_weight.next().unwrap();
    // should have used regexes, this is awful
    let weight = (&name_weight.next().unwrap()[1..]).split(")").next().unwrap().parse().unwrap();
    let mut children_names = vec![];
    if let Some(children) = mapping.next() {
        for name in children.split(", ") {
            children_names.push(name.to_string());
        }
    }

    NodeSpec { name: name.to_string(), weight: weight, children: children_names }
}

// parent is the one that contains this program as a child
fn parent(programs: &[Node], program: usize) -> Option<usize> {
    programs.iter().enumerate()
        .filter_map(|(i, ref n)|
                    if n.children.contains(&program) { Some(i) } else { None })
        .next()
}

// bottom of the stack is the root of the tree: the node that has no parent
fn bottom_program<'a>(programs: &'a [Node]) -> &'a Node<'a> {
    programs.iter().enumerate()
        .find(|&(i, _)| parent(programs, i).is_none())
        .unwrap().1
}

fn bottom_program_name<'a>(programs: &'a [Node]) -> &'a str {
    bottom_program(programs).name
}

fn track<'a>(programs: &'a [Node], n: &Node) -> u32 {
    if n.children.len() == 0 {
        return n.weight;
    }

    let mut weights = n.children.iter()
        .map(|&ci| &programs[ci])
        .map(|c| (track(programs, c), c.name, c.weight))
        .collect::<Vec<_>>();

    assert!(weights.len() > 1); // this seems to be the case
    weights.sort();

    if weights[0].0 < weights[1].0 || weights[weights.len() - 1].0 > weights[weights.len() - 2].0 {
        println!("!! {} {:?}", n.name, weights);
        println!("!! sides {} {}",
                 weights[0].2 + weights[1].0 - weights[0].0,
                 weights[weights.len() - 1].2 - (weights[weights.len() - 1].0 - weights[weights.len() - 2].0),
                 );
    }
    //println!("{} {}", n.name, n.weight + weights.iter().sum::<u32>());
    n.weight + weights.iter().map(|&(w, _, _)| w).sum::<u32>()
}

fn b<'a>(programs: &'a [Node]) -> u32 {
    track(programs, bottom_program(programs))
}

fn main() {
    let nodespecs = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap())
        .lines().map(|x| parse_line(&x.unwrap())).collect::<Vec<_>>();
    // get node names without children first, because the children are indices
    let mut nodes = nodespecs.iter()
        .map(|ns| Node { name: &ns.name, weight: ns.weight, children: vec![] })
        .collect::<Vec<_>>();
    // name -> index mapping
    let indices: HashMap<&str, usize> = nodes.iter().enumerate()
        .map(|(i, n)| { (n.name, i) }).collect();
    println!("{:?}", nodes);
    println!("{:?}", indices);
    // now that indices exist, the children can be built from the names
    for (ns, n) in nodespecs.iter().zip(nodes.iter_mut()) {
        n.children = ns.children.iter()
            .map(|name| *indices.get::<str>(name).unwrap()).collect();
    }
    println!("{:?}", nodes);
    // tknk for sample
    println!("{}", bottom_program_name(&nodes));
    // sample: ugml has to be 60
    println!("total weight is {} but it's unrelated", b(&nodes));
}
