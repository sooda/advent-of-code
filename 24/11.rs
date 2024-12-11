use std::io::{self, BufRead};
use std::collections::HashMap;

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

fn blink_faster(stones: &[u128], blinks: usize) -> usize {
    let mut expansion = HashMap::<u128, (u128, Option<u128>)>::new();
    expansion.insert(0, (1, None));
    let mut collection = HashMap::<u128, usize>::new();
    let mut next_collection = HashMap::<u128, usize>::new();
    for &s in stones {
        *collection.entry(s).or_insert(0) += 1;
    }
    let inc = |hm: &mut HashMap<_, _>, k, v| *hm.entry(k).or_insert(0) += v;
    for _ in 0..blinks {
        next_collection.clear();
        for (&stone, &count) in &collection {
            if let Some(data) = expansion.get(&stone) {
                inc(&mut next_collection, data.0, count);
                if let Some(s) = data.1 {
                    inc(&mut next_collection, s, count);
                }
            } else {
                let digits = stone.ilog10() + 1;
                if digits % 2 == 0 {
                    let base = 10u128.pow(digits / 2);
                    let (a, b) = (stone / base, stone % base);
                    expansion.insert(stone, (a, Some(b)));
                    inc(&mut next_collection, a, count);
                    inc(&mut next_collection, b, count);
                } else {
                    expansion.insert(stone, (stone * 2024, None));
                    inc(&mut next_collection, stone * 2024, count);
                }
            }
        }
        std::mem::swap(&mut collection, &mut next_collection);
    }
    collection.values().sum()
}

fn main() {
    let stones: Vec<_> = io::stdin().lock().lines()
        .next().unwrap().unwrap()
        .split(' ').map(|s| s.parse().unwrap())
        .collect();
    println!("{}", blink(stones.clone(), 6));
    println!("{} (backup)", blink(stones.clone(), 25));
    println!("{}", blink_faster(&stones, 25));
    println!("{}", blink_faster(&stones, 75));
}
