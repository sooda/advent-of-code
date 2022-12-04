use std::io::{self, BufRead};

type Sects = (i32, i32);
type Assignment = (Sects, Sects);

// a contains b
fn fully_contains(a: Sects, b: Sects) -> bool {
    return b.0 >= a.0 && b.1 <= a.1
}

fn num_fully_contained(sectss: &[Assignment]) -> usize {
    sectss.iter().filter(|sects| {
        fully_contains(sects.0, sects.1) || fully_contains(sects.1, sects.0)
    }).count()
}

fn partially_contains(a: Sects, b: Sects) -> bool {
    return b.0 <= a.1 && b.1 >= a.0
}

fn num_partially_contained(sectss: &[Assignment]) -> usize {
    sectss.iter().filter(|sects| {
        partially_contains(sects.0, sects.1) || partially_contains(sects.1, sects.0)
    }).count()
}

fn parse_assignments(line: &str) -> Assignment {
    let mut sp = line.split(&['-', ',']).map(|x| x.parse().unwrap());
    ((sp.next().unwrap(), sp.next().unwrap()),
     (sp.next().unwrap(), sp.next().unwrap()))
}

fn main() {
    let section_assignments: Vec<_> = io::stdin().lock().lines()
        .map(|line| parse_assignments(&line.unwrap()))
        .collect();
    println!("{}", num_fully_contained(&section_assignments));
    println!("{}", num_partially_contained(&section_assignments));
}
