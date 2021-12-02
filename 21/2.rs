use std::io::{self, BufRead};

fn steer(pos: (i32, i32), step: &str) -> (i32, i32) {
    let mut words = step.split(' ');
    let direction = words.next().unwrap();
    let amount = words.next().unwrap().parse::<i32>().unwrap();
    match direction {
        "forward" => (pos.0 + amount, pos.1),
        "down" => (pos.0, pos.1 + amount),
        "up" => (pos.0, pos.1 - amount),
        _ => panic!()
    }
}

fn final_position(course_plan: &[String]) -> i32 {
    let final_pos = course_plan.iter().fold((0i32, 0i32), |acc, step| steer(acc, step));
    final_pos.0 * final_pos.1
}

fn main() {
    let course_plan: Vec<String> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect();
    println!("{}", final_position(&course_plan));
}
