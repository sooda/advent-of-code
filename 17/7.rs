use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Node<'a> {
    name: &'a str,
    children: Vec<usize>
}

struct NodeSpec {
    name: String,
    children: Vec<String>
}

fn parse_line(line: &str) -> NodeSpec {
    // tknk (41) -> ugml, padx, fwft
    // jptl (61)
    let mut mapping = line.split(" -> ");
    let name_children = mapping.next().unwrap();
    let name = name_children.split(" ").next().unwrap();
    let mut children_names = vec![];
    if let Some(children) = mapping.next() {
        for name in children.split(", ") {
            children_names.push(name.to_string());
        }
    }

    NodeSpec { name: name.to_string(), children: children_names }
}

// parent is the one that contains this program as a child
fn parent(programs: &[Node], program: usize) -> Option<usize> {
    programs.iter().enumerate()
        .filter_map(|(i, ref n)|
                    if n.children.contains(&program) { Some(i) } else { None })
        .next()
}

// bottom of the stack is the root of the tree: the node that has no parent
fn bottom_program<'a>(programs: &'a [Node]) -> &'a str {
    programs.iter().enumerate()
        .find(|&(i, _)| parent(programs, i).is_none())
        .unwrap().1.name
}

fn main() {
    let nodespecs = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap())
        .lines().map(|x| parse_line(&x.unwrap())).collect::<Vec<_>>();
    // get node names without children first, because the children are indices
    let mut nodes = nodespecs.iter()
        .map(|ns| Node { name: &ns.name, children: vec![] }).collect::<Vec<_>>();
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
    println!("{}", bottom_program(&nodes));
}
