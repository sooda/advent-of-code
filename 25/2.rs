use std::io::{self, Read};

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
    ranges.into_iter()
        .map(|&(a, b)| (a..=b).filter(|&x| invalid(x)).sum::<i64>())
        .sum()
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
}
