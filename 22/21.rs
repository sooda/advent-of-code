use std::io::{self, BufRead};
use std::collections::HashMap;

#[derive(Copy, Clone)]
enum Op {
    Plus,
    Minus,
    Mul,
    Div,
}

enum Yell {
    MathResult(String, String, Op),
    Number(i64),
}

struct Monkey {
    name: String,
    yell: Yell,
}

fn obtain(monkeys: &HashMap<&str, &Monkey>, name: &str) -> i64 {
    match &monkeys.get(name).unwrap().yell {
        Yell::Number(n) => *n,
        Yell::MathResult(left, right, op) => {
            let left_num = obtain(monkeys, &left);
            let right_num = obtain(monkeys, &right);
            match op {
                Op::Plus => left_num + right_num,
                Op::Minus => left_num - right_num,
                Op::Mul => left_num * right_num,
                Op::Div => left_num / right_num,
            }
        }
    }
}

fn root_number(monkeys: &HashMap<&str, &Monkey>) -> i64 {
    obtain(monkeys, "root")
}

fn parse_monkey(input: &str) -> Monkey {
    let mut sp = input.split(": ");
    let name = sp.next().unwrap().to_string();
    let yell = sp.next().unwrap();
    let yell = if let Ok(num) = yell.parse::<i64>() {
        Yell::Number(num)
    } else {
        let mut sp = yell.split(' ');
        let left = sp.next().unwrap().to_string();
        let op = sp.next().unwrap();
        let right = sp.next().unwrap().to_string();
        let op = match op {
            "+" => Op::Plus,
            "-" => Op::Minus,
            "*" => Op::Mul,
            "/" => Op::Div,
            _ => panic!()
        };
        Yell::MathResult(left, right, op)
    };
    Monkey { name, yell }
}

fn main() {
    let monkeys: Vec<_> = io::stdin().lock().lines()
        .map(|line| parse_monkey(&line.unwrap()))
        .collect();
    let monkeys_map: HashMap<_, _> = monkeys.iter().map(|m| (&m.name as &str, m)).collect();
    println!("{}", root_number(&monkeys_map));
}
