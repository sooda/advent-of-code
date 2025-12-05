use std::io::{self, Read};

type Pair = (i64, i64);

fn is_fresh(id: i64, ranges: &[Pair]) -> bool {
    ranges.iter().any(|r| id >= r.0 && id <= r.1)
}

fn fresh_count(ranges: &[Pair], available: &[i64]) -> usize {
    available.iter().filter(|&&id| is_fresh(id, ranges)).count()
}

fn overlap(a: Pair, b: Pair) -> bool {
    a.0.max(b.0) <= a.1.min(b.1)
}

fn union(a: Pair, b: Pair) -> Pair {
    (a.0.min(b.0), a.1.max(b.1))
}

fn reduce(totals: &mut Vec<Pair>, r: Pair) {
    if let Some(ti) = totals.iter().position(|&t| overlap(r, t)) {
        let t = totals.swap_remove(ti);
        reduce(totals, union(r, t));
    } else {
        totals.push(r);
    }
}

fn fresh_range(ranges: &[Pair]) -> i64 {
    let mut totals = Vec::new();
    for &r in ranges {
        reduce(&mut totals, r);
    }
    totals.iter().map(|&(lo, hi)| hi - lo + 1).sum()
}

fn parse(file: &str) -> (Vec<Pair>, Vec<i64>) {
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
    println!("{:?}", fresh_range(&ranges));
}
