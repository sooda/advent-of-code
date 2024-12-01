use std::io::{self, BufRead};

fn total_distance(lists: &[(i32, i32)]) -> i32 {
    let mut a = lists.iter().map(|x| x.0).collect::<Vec<_>>();
    let mut b = lists.iter().map(|x| x.1).collect::<Vec<_>>();
    a.sort_unstable();
    b.sort_unstable();
    a.iter().zip(b.iter()).map(|(q, w)| (q - w).abs()).sum()
}

fn similarity_score(lists: &[(i32, i32)]) -> i32 {
    lists.iter().map(|(q, _)| {
        lists.iter().filter(|&(_, w)| w == q).count() as i32 * q
    }).sum()
}

fn parse(inp: &str) -> (i32, i32) {
    let mut sp = inp.split("   ");
    let a = sp.next().unwrap().parse().unwrap();
    let b = sp.next().unwrap().parse().unwrap();
    (a, b)
}

fn main() {
    let lists: Vec<(i32, i32)> = io::stdin().lock().lines()
        .map(|line| parse(&line.unwrap()))
        .collect();
    println!("{}", total_distance(&lists));
    println!("{}", similarity_score(&lists));
}
