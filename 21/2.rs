use std::io::{self, BufRead};

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

    fn simple_position_score(&self) -> i32 {
        self.pos.0 * self.aim
    }
}

fn final_scores_aiming(course_plan: &[String]) -> (i32, i32) {
    let final_sub = course_plan.iter().fold(
        Sub::new(), |sub, step| sub.steer_aiming(step)
    );
    (final_sub.simple_position_score(), final_sub.position_score())
}

fn main() {
    let course_plan: Vec<String> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect();
    println!("{:?}", final_scores_aiming(&course_plan));
}
