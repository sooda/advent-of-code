use std::io::{self, BufRead};

fn focusing_power(init_seq: &str) -> u32 {
    let mut boxes: Vec<Vec<(&str, u32)>> = vec![Vec::new(); 256];
    // FIXME: parse and then execute would make this nicer
    for step in init_seq.split(',') {
        if step.contains('-') {
            let mut sp = step.split('-');
            let label = sp.next().unwrap();
            let box_ = &mut boxes[hash_step(label) as usize];
            if let Some(idx) = box_.iter().position(|x| x.0 == label) {
                box_.remove(idx);
            }
        } else if step.contains('=') {
            let mut sp = step.split('=');
            let label = sp.next().unwrap();
            let box_ = &mut boxes[hash_step(label) as usize];
            let focal_len = sp.next().unwrap().parse().unwrap();
            if let Some(idx) = box_.iter().position(|x| x.0 == label) {
                box_[idx] = (label, focal_len);
            } else {
                box_.push((label, focal_len));
            }
        } else {
            panic!()
        }
    }
    boxes.iter().enumerate().map(|(boxnum, b)| {
        b.iter().enumerate().map(move |(slotnum, (_label, foclen))| {
            (boxnum as u32 + 1) * (slotnum as u32 + 1) * foclen
        }).sum::<u32>()
    }).sum()
}

fn hash_step(step: &str) -> u32 {
    step.as_bytes().iter().fold(0, |curr, &x| ((curr + (x as u32)) * 17) % 256)
}

fn hash_steps(init_seq: &str) -> u32 {
    init_seq.split(',').map(|step| hash_step(step)).sum()
}

fn main() {
    let init_sequence: String = io::stdin().lock().lines().next().unwrap().unwrap();
    println!("{}", hash_steps(&init_sequence));
    println!("{}", focusing_power(&init_sequence));
}
