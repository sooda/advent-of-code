use std::io::{self, Read};
use std::collections::HashMap;

type Map = HashMap<String, (String, String)>;

fn steps<F>(dirs: &[bool], map: &Map, begin: &str, end: F) -> usize
where F: Fn(&str) -> bool
{
    let mut current = begin;
    let mut ways = dirs.iter().cycle();
    let mut n = 0;
    while !end(current) {
        let pair = map.get(current).unwrap();
        current = if *ways.next().unwrap() { &pair.1 } else { &pair.0 };
        n += 1;
    }
    n
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while a != 0 {
        let c = b % a;
        b = a;
        a = c;
    }
    b
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn steps_ghost(dirs: &[bool], map: &Map) -> usize {
    let startnodes = map.keys().filter(|k| k.ends_with('A')).map(|s| s as &str).collect::<Vec<_>>();
    let counts = startnodes.iter().map(|begin| steps(dirs, map, begin, |s| s.ends_with('Z'))).collect::<Vec<_>>();
    counts.iter().fold(1, |acc, &x| lcm(acc, x))
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
    println!("{}", steps(&dirs, &map, "AAA", |x| x == "ZZZ"));
    println!("{}", steps_ghost(&dirs, &map));
}
