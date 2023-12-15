use std::io::{self, BufRead};

fn hash_step(step: &str) -> u32 {
    step.as_bytes().iter().fold(0, |curr, &x| ((curr + (x as u32)) * 17) % 256)
}

fn hash_steps(init_seq: &str) -> u32 {
    init_seq.split(',').map(|step| hash_step(step)).sum()
}

fn main() {
    let init_sequence: String = io::stdin().lock().lines().next().unwrap().unwrap();
    println!("{}", hash_steps(&init_sequence));
}
