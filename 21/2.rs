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

struct Sub {
    pos: (i32, i32),
    aim: i32,
}

impl Sub {
    fn new() -> Sub {
        Sub { pos: (0, 0), aim: 0 }
    }

    fn forward(self, force: i32) -> Sub {
        Sub {
            pos: (self.pos.0 + force, self.pos.1 + self.aim * force),
            aim: self.aim
        }
    }

    fn steer(self, force: i32) -> Sub {
        Sub {
            pos: self.pos,
            aim: self.aim + force
        }
    }

    fn steer_aiming(self, step: &str) -> Sub {
        let mut words = step.split(' ');
        let direction = words.next().unwrap();
        let amount = words.next().unwrap().parse::<i32>().unwrap();
        match direction {
            "forward" => self.forward(amount),
            "down" => self.steer(amount),
            "up" => self.steer(-amount),
            _ => panic!()
        }
    }

    fn position_score(&self) -> i32 {
        self.pos.0 * self.pos.1
    }
}

fn final_position_aiming(course_plan: &[String]) -> i32 {
    let final_sub = course_plan.iter().fold(
        Sub::new(), |sub, step| sub.steer_aiming(step)
    );
    final_sub.position_score()
}

fn main() {
    let course_plan: Vec<String> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect();
    println!("{}", final_position(&course_plan));
    println!("{}", final_position_aiming(&course_plan));
}
