use std::io::{self, Read};
use std::collections::HashMap;

type Map = HashMap<String, (String, String)>;

fn steps(dirs: &[bool], map: &Map, begin: &str, end: &str) -> usize {
    let mut current = begin;
    let mut ways = dirs.iter().cycle();
    let mut n = 0;
    while current != end {
        let pair = map.get(current).unwrap();
        current = if *ways.next().unwrap() { &pair.1 } else { &pair.0 };
        n += 1;
    }
    n
}

fn parse(file: &str) -> (Vec<bool>, Map) {
    let mut l = file.lines();
    let dirs = l.next().unwrap().chars().map(|c| c == 'R').collect();
    l.next().unwrap();
    let mut map = Map::new();
    for step in l {
        // AAA = (BBB, CCC)
        let mut sp = step.split(" = (");
        let key = sp.next().unwrap();
        let mut sp = sp.next().unwrap().split(", ");
        let left = sp.next().unwrap();
        let right = sp.next().unwrap().trim_end_matches(')');
        map.insert(key.to_string(), (left.to_string(), right.to_string()));
    }

    (dirs, map)
}

fn main() {
    let mut file = String::new();
    io::stdin().read_to_string(&mut file).unwrap();
    let (dirs, map) = parse(&file);
    println!("{}", steps(&dirs, &map, "AAA", "ZZZ"));
}
