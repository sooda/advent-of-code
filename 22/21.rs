use std::io::{self, BufRead};
use std::collections::HashMap;

#[derive(Copy, Clone)]
enum Op {
    Plus,
    Minus,
    Mul,
    Div,
}

#[derive(Clone)]
enum Yell {
    MathResult(String, String, Op),
    Number(i64),
}
use Yell::*;

#[derive(Clone)]
struct Monkey {
    name: String,
    yell: Yell,
}

type Monkeys = HashMap<String, Monkey>;

fn obtain(monkeys: &Monkeys, name: &str) -> i64 {
    match &monkeys.get(name).unwrap().yell {
        Number(n) => *n,
        MathResult(left, right, op) => {
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

fn root_number(monkeys: &Monkeys) -> i64 {
    obtain(monkeys, "root")
}

fn find_leaf_monkey(monkeys: &Monkeys, node: &str, find: &str) -> bool {
    match &monkeys.get(node).unwrap().yell {
        Number(_) => node == find,
        MathResult(left, right, _) => {
            find_leaf_monkey(monkeys, left, find) || find_leaf_monkey(monkeys, right, find)
        }
    }
}

fn order_x<'a, 'b>(monkeys: &'a Monkeys, left: &'b str, right: &'b str) -> (&'b str, &'b str) {
    let found = (
        find_leaf_monkey(monkeys, &left, "humn"),
        find_leaf_monkey(monkeys, &right, "humn")
    );
    match found {
        (true, false) => (left, right),
        (false, true) => (right, left),
        (false, false) => panic!("no x"),
        (true, true) => panic!("both x??"),
    }
}

/*

((4+(2*(x-3)))/4) = ((32-2)*5)
 (4+(2*(x-3)))    = (((32-2)*5)*4)
    (2*(x-3))     = ((((32-2)*5)*4)-4)
       (x-3)      = (((((32-2)*5)*4)-4)/2)
        x         = (((((32-2)*5)*4)-4)/2)+3
*/
fn find_x(monkeys: &mut Monkeys, xside: &str, mathside: &str) -> i64 {
    match &monkeys.get(xside).unwrap().yell {
        Number(_) => obtain(monkeys, mathside),
        MathResult(left, right, op) => {
            // FIXME keep the path in topmost search to avoid searching again?
            let (x, other) = order_x(monkeys, left, right);
            let (x, other, mathside) = (x.to_string(), other.to_string(), mathside.to_string());

            // wrap the current math side and the part split from x in a new monkey
            let new_name = mathside.clone() + "_for_x";

            let yell = match (x == *left, op) {
                // (x + other, x - other, x / other, x * other) = mathside
                (true, Op::Plus)   => MathResult(mathside, other, Op::Minus),
                (true, Op::Minus)  => MathResult(mathside, other, Op::Plus),
                (true, Op::Mul)    => MathResult(mathside, other, Op::Div),
                (true, Op::Div)    => MathResult(mathside, other, Op::Mul),
                // (other + x, other - x, other / x, other * x) = mathside
                (false, Op::Plus)  => MathResult(mathside, other, Op::Minus),
                (false, Op::Minus) => MathResult(other, mathside, Op::Minus),
                (false, Op::Mul)   => MathResult(mathside, other, Op::Div),
                (false, Op::Div)   => MathResult(other, mathside, Op::Div),
            };

            let new_m = Monkey { name: new_name.clone(), yell };
            monkeys.insert(new_name.clone(), new_m);

            find_x(monkeys, &x, &new_name)
        }
    }
}

fn root_equality_test(mut monkeys: Monkeys) -> i64 {
    let (left, right) = match &monkeys.get("root").unwrap().yell {
        MathResult(left, right, _) => (left.clone(), right.clone()),
        _ => panic!()
    };
    let (x, math) = order_x(&monkeys, &left, &right);

    find_x(&mut monkeys, x, math)
}

fn parse_monkey(input: &str) -> Monkey {
    let mut sp = input.split(": ");
    let name = sp.next().unwrap().to_string();
    let yell = sp.next().unwrap();
    let yell = if let Ok(num) = yell.parse::<i64>() {
        Number(num)
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
        MathResult(left, right, op)
    };
    Monkey { name, yell }
}

fn main() {
    let monkeys: Vec<_> = io::stdin().lock().lines()
        .map(|line| parse_monkey(&line.unwrap()))
        .collect();
    let monkeys_map: Monkeys = monkeys.iter().map(|m| (m.name.clone(), m.clone())).collect();
    println!("{}", root_number(&monkeys_map));
    println!("{}", root_equality_test(monkeys_map));
}
