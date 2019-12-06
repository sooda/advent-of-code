use std::io::{self, BufRead};
use std::collections::HashMap;

fn search(orbits: &HashMap<String, String>, countmap: &mut HashMap<String, usize>, object: &String) -> usize {
    // already visited?
    let count = countmap.get(object);
    if count.is_some() {
        return *count.unwrap();
    }

    let parent_count = search(orbits, countmap, &orbits[object]);
    countmap.insert(object.to_string(), parent_count + 1);
    parent_count + 1
}

fn orbit_count(orbits: &HashMap<String, String>) -> usize {
    // the number of orbits is the distance from COM in the orbit graph (a DAG)
    let mut countmap = HashMap::new();
    // mark the end of the recursion explicitly from the definition
    countmap.insert("COM".to_string(), 0);

    for celestial_object in orbits.keys() {
        search(orbits, &mut countmap, celestial_object);
    }

    countmap.values().sum()
}

fn path_to_com<'a>(orbits: &'a HashMap<String, String>, from: &'a str) -> Vec<&'a str> {
    let mut route = Vec::new();
    let mut hop = from;
    while hop != "COM" {
        hop = &orbits[hop];
        route.push(hop);
    }
    route
}

fn path_to_santa(orbits: &HashMap<String, String>) -> Option<usize> {
    let youpath = path_to_com(orbits, "YOU");
    let sanpath = path_to_com(orbits, "SAN");

    for (i, x) in youpath.iter().enumerate() {
        if sanpath.contains(x) {
            for (j, y) in sanpath.iter().enumerate() {
                if x == y {
                    return Some(i + j);
                }
            }
        }
    }
    None
}

fn parse_orbit(desc: &str) -> (String, String) {
    let mut sp = desc.split(')');
    let (parent, moon) = (sp.next().unwrap().to_string(), sp.next().unwrap().to_string());
    (moon, parent)
}

fn main() {
    // each "moon" should orbit directly just one "planet", so no map collisions
    let orbits: HashMap<String, String> = io::stdin().lock().lines()
        .map(|orbitdesc| parse_orbit(&orbitdesc.unwrap()))
        .collect();
    println!("{:?}", orbit_count(&orbits));
    println!("{:?}", path_to_santa(&orbits));
}
