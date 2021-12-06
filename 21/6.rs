use std::io::{self, BufRead};

fn fish_population(school: &[u64], sim_length: usize) -> u64 {
    let mut fish = [0u64; 9]; // count per age
    let mut _fish2 = [0u64; 9];
    for &age in school {
        fish[age as usize] += 1;
    }

    _fish2 = fish;
    for start_pos in 0..sim_length {
        fish.rotate_left(1);
        fish[6] += fish[8];
        _fish2[(7 + start_pos) % 9] += _fish2[start_pos % 9];
    }
    assert!(fish.iter().sum::<u64>() == _fish2.iter().sum::<u64>());
    fish.iter().sum()
}

fn main() {
    let input: Vec<u64> = io::stdin().lock().lines()
        .next().unwrap().unwrap().split(',')
        .map(|n| n.parse().unwrap())
        .collect();
    println!("{}", fish_population(&input, 80));
    println!("{}", fish_population(&input, 256));
}
