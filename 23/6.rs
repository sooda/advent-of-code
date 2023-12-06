use std::io::{self, Read};

fn beats(time: i64, best_distance: i64) -> i64 {
    (1..time).filter(|button_press| {
        let travel = (time - button_press) * button_press;
        travel > best_distance
    }).count() as i64
}

fn ways_to_beat(records: &[(i64, i64)]) -> i64 {
    records.iter().map(|r| beats(r.0, r.1)).fold(1, |acc, x| acc * x)
}

fn join(total: i64, next: i64) -> i64 {
    total * 10i64.pow(next.ilog10() + 1) as i64 + next
}

fn fix_kerning(records: &[(i64, i64)]) -> (i64, i64) {
    records.iter().fold((0, 0), |acc, x| (join(acc.0, x.0), join(acc.1, x.1)))
}

fn parse(file: &str) -> Vec<(i64, i64)> {
    let mut l = file.lines();
    let times = l.next().unwrap();
    let distances = l.next().unwrap();
    times.split(' ').flat_map(|s| s.parse::<i64>().ok())
        .zip(
            distances.split(' ').flat_map(|s| s.parse::<i64>().ok())
        )
        .collect()
}

fn main() {
    let mut file = String::new();
    io::stdin().read_to_string(&mut file).unwrap();
    let paper = parse(&file);
    println!("{:}", ways_to_beat(&paper));
    let fixed = fix_kerning(&paper);
    println!("{:}", beats(fixed.0, fixed.1));
}
