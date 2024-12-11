use std::io::{self, BufRead};

fn blink(mut stones: Vec<u128>, blinks: usize) -> usize {
    for _ in 0..blinks {
        let n = stones.len();
        let mut i = 0;
        for _ in 0..n {
            let stone = stones[i];
            if stone == 0 {
                stones[i] = 1;
            } else {
                let digits = stone.ilog10() + 1;
                if digits % 2 == 0 {
                    let base = 10u128.pow(digits / 2);
                    let (a, b) = (stone / base, stone % base);
                    stones[i] = a;
                    stones.insert(i + 1, b);
                    i += 1;
                } else {
                    stones[i] = stone * 2024;
                }
            }
            i += 1;
        }
    }
    stones.len()
}

fn main() {
    let stones: Vec<_> = io::stdin().lock().lines()
        .next().unwrap().unwrap()
        .split(' ').map(|s| s.parse().unwrap())
        .collect();
    println!("{}", blink(stones.clone(), 6));
    println!("{}", blink(stones, 25));
}
