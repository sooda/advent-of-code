use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use std::collections::HashSet;
use std::collections::HashMap;

fn redist(banks: &mut Vec<u32>) {
    // rev: max_by returns the last one, but the puzzle spec says ties are broken by lowest idx
    let max_idx = banks.iter().enumerate().rev().max_by(|&(_, a), &(_, b)| a.cmp(b)).unwrap().0;
    let mut blocks = banks[max_idx];
    banks[max_idx] = 0;
    for i in (0..banks.len()).cycle().skip(max_idx + 1) {
        banks[i] += 1;
        blocks -= 1;
        if blocks == 0 {
            break;
        }
    }
}

// count of redistributions from the beginning until a loop is found
fn redist_cycle_count(mut banks: Vec<u32>) -> usize {
    let mut seen = HashSet::new();
    for i in 0.. {
        if seen.contains(&banks) {
            return i;
        }
        seen.insert(banks.clone());
        redist(&mut banks);
    }
    unreachable!()
}

// how long is the cycle we've found?
fn redist_cycle_size(mut banks: Vec<u32>) -> usize {
    let mut seen = HashMap::new();
    for i in 0usize.. {
        if let Some(j) = seen.get(&banks) {
            return i - j;
        }
        seen.insert(banks.clone(), i);
        redist(&mut banks);
    }
    unreachable!()
}

fn main() {
    let mut sample = vec![0u32, 2, 7, 0];
    redist(&mut sample);
    assert!(sample == vec![2u32, 4, 1, 2]);

    // test duplicate max
    let mut sample2 = vec![3u32, 1, 2, 3];
    redist(&mut sample2);
    assert!(sample2 == vec![0u32, 2, 3, 4]);

    assert!(redist_cycle_count(vec![0, 2, 7, 0]) == 5);
    assert!(redist_cycle_size(vec![0, 2, 7, 0]) == 4);
    let input = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap())
        .lines().next().unwrap().unwrap().split("\t").map(|n| n.parse::<u32>().unwrap()).collect::<Vec<_>>();
    println!("{}", redist_cycle_count(input.clone()));
    println!("{}", redist_cycle_size(input));
}
