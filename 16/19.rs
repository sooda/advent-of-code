use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn round(elves: &mut [u32]) -> bool {
    let mut modified = 0;
    for i in 0..elves.len() {
        if elves[i] > 0 {
            for j in 1..elves.len() {
                let jj = (i + j) % elves.len();
                if elves[jj] > 0 {
                    //println!("{} takes {}", i+1, jj+1);
                    modified += 1;
                    elves[i] += elves[jj];
                    elves[jj] = 0;
                    break;
                }
            }
        }
    }

    modified == 0
}

fn winner(num_elves: usize) -> usize {
    let mut elves = vec![1u32; num_elves];
    // println!("{:?}", elves);
    while !round(&mut elves) {
        // nothing here
        //println!("{:?}", elves);
    }

    1 + elves.iter().position(|&e| e > 0).unwrap()
}

fn main() {
    winner(10); // debugged these two on paper
    winner(15);
    assert!(winner(5) == 3);
    let input = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap()).lines().next().unwrap().unwrap();
    println!("{}", winner(input.parse().unwrap()));
}
