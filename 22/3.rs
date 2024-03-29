use std::io::{self, BufRead};
use std::collections::HashSet;

fn item_priority(item: u8) -> usize {
    // some bit ops (ascii bit 5) would be more clever but that's not the point here
    (match item {
        b'a'..=b'z' => item - b'a' + 1,
        b'A'..=b'Z' => item - b'A' + 27,
        _ => panic!()
    }) as usize
}

fn sack_priority(sack: &str) -> usize {
    let n2 = sack.len() / 2;
    let compartment_a: HashSet<u8> = sack.as_bytes().iter().cloned().take(n2).collect();
    let compartment_b: HashSet<u8> = sack.as_bytes().iter().cloned().skip(n2).collect();
    item_priority(*compartment_a.intersection(&compartment_b).next().unwrap())
}

fn priority_sums(sacks: &[String]) -> usize {
    sacks.iter().map(|s| sack_priority(s)).sum()
}

fn common_priority(sacks: &[String]) -> usize {
    let mut common: HashSet<u8> = sacks[0].as_bytes().iter().cloned().collect();
    for s in &sacks[1..] {
        let s_set: HashSet<u8> = s.as_bytes().iter().cloned().collect();
        common = common.intersection(&s_set).cloned().collect();
    }
    item_priority(*common.iter().next().unwrap())
}

fn group_priority_sums(sacks: &[String]) -> usize {
    sacks.chunks(3).map(|group| {
        common_priority(group)
    }).sum()
}

fn main() {
    let rucksacks: Vec<_> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect();
    println!("{}", priority_sums(&rucksacks));
    println!("{}", group_priority_sums(&rucksacks));
}
