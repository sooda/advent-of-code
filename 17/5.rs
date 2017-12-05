use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn steps_to_exit(mut jumps: Vec<i32>) -> i32 {
    let mut pc = 0i32;
    let mut steps = 0;
    while pc >= 0 && pc < jumps.len() as i32 {
        let offset = jumps[pc as usize];
        jumps[pc as usize] += 1;
        pc += offset;
        steps += 1;
    }

    steps
}

fn steps_to_exit_strange(mut jumps: Vec<i32>) -> i32 {
    let mut pc = 0i32;
    let mut steps = 0;
    while pc >= 0 && pc < jumps.len() as i32 {
        let offset = jumps[pc as usize];
        jumps[pc as usize] += if offset >= 3 { -1 } else { 1 };
        pc += offset;
        steps += 1;
    }

    steps
}

fn main() {
    assert!(steps_to_exit(vec![0, 3, 0, 1, -3]) == 5);
    assert!(steps_to_exit_strange(vec![0, 3, 0, 1, -3]) == 10);
    let input_lines = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap())
        .lines().map(|x| x.unwrap().parse::<i32>().unwrap()).collect::<Vec<_>>();
    println!("{}", steps_to_exit(input_lines.clone()));
    println!("{}", steps_to_exit_strange(input_lines));
}
