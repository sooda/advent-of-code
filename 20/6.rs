use std::io::{self, Read};
use std::collections::HashSet;

fn main() {
    let mut group_questions = String::new();
    io::stdin().read_to_string(&mut group_questions).unwrap();
    group_questions.truncate(group_questions.len() - 1); // strip off last newline
    let groups: Vec<Vec<&str>> = group_questions.split("\n\n").map(|group| {
        // (could also parse the answers to bitmaps and "|" them together for epic perf)
        group.split('\n').collect()
    }).collect();

    let sum_yesgroups: usize = groups.iter().map(|people_results| {
        let mut anyone_yes = HashSet::new();
        for person_results in people_results {
            anyone_yes.extend(person_results.chars());
        }
        anyone_yes.len()
    }).sum();
    println!("{}", sum_yesgroups);
}
