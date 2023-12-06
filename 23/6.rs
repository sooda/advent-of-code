use std::io::{self, Read};

fn beats(time: i32, best_distance: i32) -> i32 {
    (1..time).filter(|button_press| {
        let travel = (time - button_press) * button_press;
        travel > best_distance
    }).count() as i32
}

fn ways_to_beat(records: &[(i32, i32)]) -> i32 {
    records.iter().map(|r| beats(r.0, r.1)).fold(1, |acc, x| acc * x)
}

fn parse(file: &str) -> Vec<(i32, i32)> {
    let mut l = file.lines();
    let times = l.next().unwrap();
    let distances = l.next().unwrap();
    times.split(' ').flat_map(|s| s.parse::<i32>().ok())
        .zip(
            distances.split(' ').flat_map(|s| s.parse::<i32>().ok())
        )
        .collect()
}

fn main() {
    let mut file = String::new();
    io::stdin().read_to_string(&mut file).unwrap();
    println!("{:?}", ways_to_beat(&parse(&file)));
}
