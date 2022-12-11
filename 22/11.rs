use std::io::{self, Read};
use std::collections::VecDeque;

extern crate regex;
use regex::Regex;

#[derive(Clone, Debug)]
enum OpArg {
    Old,
    Constant(u64),
}

#[derive(Clone, Debug)]
enum Op {
    Sum,
    Mul
}

#[derive(Clone, Debug)]
struct Monkey {
    items: VecDeque<u64>,
    op: Op,
    op_arg: OpArg,
    div_test: u64,
    true_throw: usize,
    false_throw: usize,
}

fn throw_item(monkey: &mut Monkey, worry_ease: u64) -> Option<(usize, u64)> {
    if let Some(item) = monkey.items.pop_front() {
        let arg = match monkey.op_arg {
            OpArg::Old => item,
            OpArg::Constant(x) => x,
        };
        let post_op = match monkey.op {
            Op::Sum => item + arg,
            Op::Mul => item * arg,
        } / worry_ease % (2 * 3 * 5 * 7 * 11 * 13 * 17 * 19 * 23);
        let destination = if post_op % monkey.div_test == 0 {
            monkey.true_throw
        } else {
            monkey.false_throw
        };
        Some((destination, post_op))
    } else {
        None
    }
}

fn business_round(monkeys: &mut [Monkey], inspections: &mut [u64], worry_ease: u64) {
    for mi in 0..monkeys.len() {
        while let Some((destination, item)) = throw_item(&mut monkeys[mi], worry_ease) {
            inspections[mi] += 1;
            monkeys[destination].items.push_back(item);
        }
    }
}

fn monkey_business(mut monkeys: Vec<Monkey>, rounds: usize, worry_ease: u64) -> u64 {
    let mut inspections = vec![0; monkeys.len()];

    for round in 0..rounds {
        if false {
            println!("before {}: {:?} {:#?}", round, inspections, monkeys);
        }
        business_round(&mut monkeys, &mut inspections, worry_ease);
        if false {
            if round == 0 || round == 19 || (round + 1) % 1000 == 0 {
                println!("{} {:?}", round, inspections);
            }
        }
    }

    inspections.sort_unstable();
    inspections[inspections.len() - 1] * inspections[inspections.len() - 2]
}

fn parse_monkey(input: &str) -> Monkey {
    let re = Regex::new(r"Monkey \d+:
  Starting items: ([\d, ]+)
  Operation: new = old ([\*\+]) (\d+|old)
  Test: divisible by (\d+)
    If true: throw to monkey (\d+)
    If false: throw to monkey (\d+)").unwrap();
    let cap = re.captures(input).unwrap();
    let items = cap.get(1).unwrap()
        .as_str()
        .split(", ")
        .map(|item| item.parse().unwrap())
        .collect();
    let op = match cap.get(2).unwrap().as_str() {
        "+" => Op::Sum,
        "*" => Op::Mul,
        _ => panic!(),
    };
    let op_arg = match cap.get(3).unwrap().as_str() {
        "old" => OpArg::Old,
        x => OpArg::Constant(x.parse().unwrap()),
    };
    let div_test = cap.get(4).unwrap().as_str().parse().unwrap();
    let true_throw = cap.get(5).unwrap().as_str().parse().unwrap();
    let false_throw = cap.get(6).unwrap().as_str().parse().unwrap();
    Monkey { items, op, op_arg, div_test, true_throw, false_throw }
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    input.split("\n\n").map(parse_monkey).collect()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let monkeys = parse_monkeys(&input);
    println!("{}", monkey_business(monkeys.clone(), 20, 3));
    println!("{}", monkey_business(monkeys, 10000, 1));
}
