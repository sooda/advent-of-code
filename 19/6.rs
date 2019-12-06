use std::io::{self, BufRead};
use std::collections::HashMap;

fn search(orbits: &HashMap<String, String>, countmap: &mut HashMap<String, usize>, object: &str) -> usize {
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

fn common<'a>(orbits: &'a HashMap<String, String>, countmap: &'a HashMap<String, usize>,
    mut a: &'a str, mut b: &'a str) -> &'a str {
    // walk both up until they're level
    while countmap[a] > countmap[b] {
        a = &orbits[a];
    }
    while countmap[b] > countmap[a] {
        b = &orbits[b];
    }
    // the chains can be equally long but diverging at some point, so find that point
    while a != b {
        a = &orbits[a];
        b = &orbits[b];
    }

    a
}

fn path_to_santa(orbits: &HashMap<String, String>) -> Option<usize> {
    let mut countmap = HashMap::new();
    // mark the end of the recursion explicitly from the definition
    countmap.insert("COM".to_string(), 0);

    search(orbits, &mut countmap, "YOU");
    search(orbits, &mut countmap, "SAN");
    let knot = common(orbits, &countmap, "YOU", "SAN");

    // YOU and SAN not counted because they're not orbited by us, they're orbiting
    Some((countmap["YOU"] - 1 - countmap[knot]) + (countmap["SAN"] - 1 - countmap[knot]))
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
