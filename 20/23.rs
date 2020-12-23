use std::io::{self, BufRead};

// a simple type to minimize accidental messing up with indices vs values
#[derive(PartialEq, Clone, Copy)]
struct Label(usize);

// note!! 1-based indexing
fn from_labeling(labeling: &str) -> Vec<Label> {
    labeling.as_bytes().iter().map(|&b| Label((b - b'1') as usize)).collect()
}

fn to_labeling(cups: &[Label]) -> String {
    cups.iter().map(|&c| (c.0 as u8 + b'1') as char).collect()
}

fn simulate(cups: Vec<Label>, n: usize) -> Vec<Label> {
    // current cup always in the front
    let ncups = cups.len();
    // linked list from label to label
    let mut successors: Vec<Label> = vec![Label(0); cups.len()];
    for (&a, &b) in cups.iter().zip(cups.iter().cycle().skip(1)) {
        successors[a.0] = b;
    }

    let find_destination = |mut value: Label, removed_cups: (Label, Label, Label)| {
        loop {
            // current minus one, wrapping
            value = Label((value.0 + ncups - 1) % ncups);
            if value != removed_cups.0 && value != removed_cups.1 && value != removed_cups.2 {
                return value;
            }
        }
    };
    let remove_links = |map: &mut Vec<Label>, left_label: Label, last_label: Label| {
        let right_label = map[last_label.0];
        map[left_label.0] = right_label;
        // (successor of last is left dangling)
    };

    let insert_links = |map: &mut Vec<Label>, left_label: Label, first_label: Label, last_label: Label| {
        let right_label = map[left_label.0];
        map[last_label.0] = right_label;
        map[left_label.0] = first_label;
    };

    let mut current_label = cups[0];
    for _move in 1..=n {
        let first_label = successors[current_label.0];
        let second_label = successors[first_label.0];
        let third_label = successors[second_label.0];

        remove_links(&mut successors, current_label, third_label);

        let dest_label = find_destination(current_label, (first_label, second_label, third_label));
        insert_links(&mut successors, dest_label, first_label, third_label);

        current_label = successors[current_label.0];
    }

    (0..cups.len()).scan(Label(0), |label, _| {
        let value = *label;
        *label = successors[label.0];
        Some(value)
    }).collect::<Vec<Label>>()
}

fn short_game(labeling: &str, n: usize) -> String {
    let final_cups = simulate(from_labeling(labeling), n);
    to_labeling(&final_cups[1..])
}

fn long_game(labeling: &str, n: usize, total_cups: usize) -> u64 {
    let mut cups = from_labeling(labeling);
    let init_size = cups.len();
    cups.extend((init_size..total_cups).map(|x| Label(x)));
    let final_cups = simulate(cups, n);
    // 1-based values but they're 0-based internally
    (final_cups[1].0 + 1) as u64 * (final_cups[2].0 + 1) as u64
}

fn main() {
    let input: String = io::stdin().lock().lines()
        .map(|line| line.unwrap().parse().unwrap())
        .next().unwrap();
    println!("{}", short_game(&input, 100));
    println!("{}", long_game(&input, 10_000_000, 1_000_000));
}
