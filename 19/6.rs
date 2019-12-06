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
}
