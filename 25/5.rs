use std::io::{self, Read};

fn is_fresh(id: i64, ranges: &[(i64, i64)]) -> bool {
    ranges.iter().any(|r| id >= r.0 && id <= r.1)
}

fn fresh_count(ranges: &[(i64, i64)], available: &[i64]) -> usize {
    available.iter().filter(|&&id| is_fresh(id, ranges)).count()
}

fn parse(file: &str) -> (Vec<(i64, i64)>, Vec<i64>) {
    let mut sp = file.split("\n\n");
    let ranges = sp.next().unwrap()
        .lines()
        .map(|l| {
            let mut rsp = l.split('-');
            (rsp.next().unwrap().parse().unwrap(),
            rsp.next().unwrap().parse().unwrap())
        }).collect();
    let available = sp.next().unwrap()
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();
    (ranges, available)
}

fn main() {
    let mut file = String::new();
    io::stdin().read_to_string(&mut file).unwrap();
    let (ranges, available) = parse(&file);
    println!("{:?}", fresh_count(&ranges, &available));
}
