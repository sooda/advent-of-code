use std::io::{self, Read};
use std::iter;

fn invalid(a: i64) -> bool {
    let digits = a.ilog10() + 1;
    if digits % 2 == 0 {
        let x = a / 10i64.pow(digits / 2);
        let y = a % 10i64.pow(digits / 2);
        x == y
    } else {
        false
    }
}

fn sum_of_invalid(ranges: &[(i64, i64)]) -> i64 {
    ranges.iter()
        .flat_map(|&(a, b)| (a..=b))
        .filter(|&x| invalid(x)).sum()
}

fn silly(a: i64) -> bool {
    let digits = a.ilog10() + 1;
    for div in 2..=digits {
        if digits % div == 0 {
            let size = 10i64.pow(digits / div);
            let repeat = a % size;
            let all_repeat = iter::successors(Some(a / size), |&ai| Some(ai / size))
                .take((div - 1) as usize)
                .all(|ai| ai % size == repeat);
            if all_repeat {
                return true;
            }
        }
    }
    false
}

fn sum_of_silly(ranges: &[(i64, i64)]) -> i64 {
    ranges.iter()
        .flat_map(|&(a, b)| (a..=b))
        .filter(|&x| silly(x)).sum()
}

fn parse(file: &str) -> Vec<(i64, i64)> {
    file.split(",").map(|ab| {
        let mut sp = ab.split("-");
        (sp.next().unwrap().parse().unwrap(),
        sp.next().unwrap().parse().unwrap())
    }).collect()
}

fn main() {
    let mut file = String::new();
    io::stdin().read_to_string(&mut file).unwrap();
    let ranges = parse(file.trim());
    println!("{:?}", sum_of_invalid(&ranges));
    println!("{:?}", sum_of_silly(&ranges));
}
