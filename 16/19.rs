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

#[derive(Debug)]
struct Seat {
    name: usize,
    next: usize,
}

fn winner2(num_elves: usize) -> usize {
    let mut seats = Vec::new();
    for i in 0..num_elves {
        seats.push(Seat { name: 1 + i, next: ((i + 1) % num_elves) });
    }
    let mut current = 0;
    let mut before_victim = num_elves / 2 - 1;
    let mut num_even = (num_elves % 2) == 0;

    for _ in 0..num_elves-1 {
        // unlink, i.e., delete. this moves the one pointed to "before" forward by one
        seats[before_victim].next = seats[seats[before_victim].next].next;

        if num_even {
            // victim moves one forward, so before it stays as-is because of the above
        } else {
            // victim moves two forward
            before_victim = seats[before_victim].next;
        }

        num_even = !num_even;
        current = seats[current].next;
    }

    seats[current].name
}

fn main() {
    winner(10); // debugged these two on paper
    winner(15);
    assert!(winner(5) == 3);
    assert!(winner2(5) == 2);
    let input = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap()).lines().next().unwrap().unwrap();
    println!("{}", winner(input.parse().unwrap()));
    println!("{}", winner2(input.parse().unwrap()));
}
