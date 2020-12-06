use std::io::{self, Read};
use std::collections::HashSet;

fn main() {
    let mut group_questions = String::new();
    io::stdin().read_to_string(&mut group_questions).unwrap();
    group_questions.truncate(group_questions.len() - 1); // strip off last newline
    let groups: Vec<Vec<&str>> = group_questions.split("\n\n").map(|group| {
        // (could also parse the answers to bitmaps and "|" or "&" them together for epic perf)
        group.split('\n').collect()
    }).collect();

    let sum_someyes: usize = groups.iter().map(|people_results| {
        let mut anyone_yes = HashSet::new();
        for person_results in people_results {
            anyone_yes.extend(person_results.chars());
        }
        anyone_yes.len()
    }).sum();
    println!("{}", sum_someyes);

    let sum_fullyes: usize = groups.iter().map(|people_results| {
        let mut all_yes: HashSet<_> = people_results[0].chars().collect();
        for person_results in people_results.iter().skip(1) {
            let set: HashSet<_> = person_results.chars().collect();
            all_yes = all_yes.intersection(&set).copied().collect();
        }
        all_yes.len()
    }).sum();
    println!("{}", sum_fullyes);
}
