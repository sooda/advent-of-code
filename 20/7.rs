use std::io::{self, BufRead};
use std::collections::{HashMap, HashSet};

// (the naming here is a bit confusing because a bag owner is a child in the tree)
fn search_bags<'a>(bag_owners: &HashMap<&'a str, Vec<&'a str>>, which: &'a str, visit_map: &mut HashSet<&'a str>) -> usize {
    if visit_map.contains(which) {
        // this graph isn't acyclic
        0
    } else {
        let inside = match bag_owners.get(which) {
            Some(owners) => {
                owners.iter()
                    .map(|direct_owner| search_bags(bag_owners, direct_owner, visit_map))
                    .sum()
            },
            None => 0, // the chad bag is never inside another, just contains others
        };
        visit_map.insert(which);
        // count this bag and what was found to contain it
        1 + inside
    }
}

fn eventually_contained<'a>(bag_owners: &HashMap<&'a str, Vec<&'a str>>, which: &'a str) -> usize {
    // minus one so that the bag itself wouldn't be counted
    search_bags(bag_owners, which, &mut HashSet::new()) - 1
}

fn parse_bag(line: &str) -> (String, Vec<(u32, String)>) {
    let mut sp = (&line[..line.len() - 1]).split(" bags contain ");
    let owner = sp.next().unwrap().to_owned();
    let organ_spec = sp.next().unwrap();
    let organs = if organ_spec == "no other bags" {
        Vec::new()
    } else {
        organ_spec.split(", ").map(|spec| {
            // "1 bright white bag" | "2 muted yellow bags"
            let mut words = spec.split(' ');
            let n = words.next().unwrap().parse().unwrap();
            let color = words.next().unwrap().to_owned() + " " + words.next().unwrap();
            (n, color)
        }).collect()
    };

    (owner, organs)
}

fn main() {
    let bags: Vec<_> = io::stdin().lock().lines()
        .map(|line| parse_bag(&line.unwrap()))
        .collect();
    let mut bag_owners: HashMap<&str, Vec<&str>> = HashMap::new();
    for (owner, organs) in &bags {
        for (_n, organ_name) in organs {
            bag_owners.entry(organ_name).or_insert(Vec::new()).push(owner);
        }
    }
    println!("{}", eventually_contained(&bag_owners, "shiny gold"));
}
