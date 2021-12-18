use std::io::{self, BufRead};
use std::str::Chars;

const DEBUGTRACE: bool = false;

#[derive(PartialEq,Debug,Clone)]
enum Number {
    Regular(i32),
    Pair(Box<Number>, Box<Number>)
}
use Number::*;

fn magnitude(num: &Number) -> i32 {
    match num {
        &Regular(x) => x,
        Pair(l, r) => 3 * magnitude(l) + 2 * magnitude(r)
    }
}

fn add_rightmost(num: &mut Number, value: i32) {
    match num {
        Regular(n) => *n += value,
        Pair(_, r) => add_rightmost(r, value)
    }
}

fn add_leftmost(num: &mut Number, value: i32) {
    match num {
        Regular(n) => *n += value,
        Pair(l, _) => add_leftmost(l, value)
    }
}

fn try_explosion(num: &mut Number, level: usize) -> Option<(Option<i32>, Option<i32>)> {
    if let Pair(l, r) = num {
        if level == 4 {
            let ret = match (&**l, &**r) {
                (&Regular(ln), &Regular(rn)) => {
                    Some((Some(ln), Some(rn)))
                },
                _ => panic!("too complex to explode")
            };
            *num = Regular(0);
            ret
        } else if let Some(exploded) = try_explosion(&mut *l, level + 1) {
            if let Some(value) = exploded.1 {
                add_leftmost(r, value);
            }
            Some((exploded.0, None))
        } else if let Some(exploded) = try_explosion(&mut *r, level + 1) {
            if let Some(value) = exploded.0 {
                add_rightmost(l, value);
            }
            Some((None, exploded.1))
        } else {
            None
        }
    } else {
        None
    }
}

fn explosion(mut num: Number) -> Number {
    try_explosion(&mut num, 0);
    num
}

fn execute_split(value: i32) -> Number {
    let l = value / 2;
    let r = (value + 1) / 2;
    Pair(Box::new(Regular(l)), Box::new(Regular(r)))
}

fn try_split(num: &mut Number) -> bool {
    match num {
        Regular(n) => {
            if *n >= 10 {
                *num = execute_split(*n);
                true
            } else {
                false
            }
        },
        Pair(l, r) => {
            if try_split(l) {
                true
            } else if try_split(r) {
                true
            } else {
                false
            }
        }
    }
}

fn split(mut num: Number) -> Number {
    try_split(&mut num);
    num
}

fn do_stringify(num: &Number, v: &mut Vec::<char>) {
    match num {
        &Regular(mut x) => {
            if x == 0 {
                v.push('0');
            } else {
                while (x % 1000) != 0 {
                    let digit = (x / 100) % 10;
                    x *= 10;
                    if digit != 0 {
                        v.push((b'0' + (digit as u8)) as char);
                    }
                }
            }
        }
        Pair(l, r) => {
            v.push('[');
            do_stringify(l, v);
            v.push(',');
            do_stringify(r, v);
            v.push(']');
        }
    }
}

fn stringify(num: &Number) -> String {
    let mut v = Vec::new();
    do_stringify(num, &mut v);
    v.iter().collect()
}

fn iterate_reduction(num: Number) -> Number {
    let exploded = explosion(num.clone());
    if exploded != num {
        if DEBUGTRACE {
            println!("it exploded");
        }
        exploded
    } else {
        let split = split(num.clone());
        if split != num {
            if DEBUGTRACE {
                println!("it split");
            }
            split
        } else {
            num
        }
    }
}

fn reduce(mut num: Number) -> Number {
    loop {
        if DEBUGTRACE {
            println!("reduce {:}", stringify(&num));
        }
        let next = iterate_reduction(num.clone());
        if DEBUGTRACE {
            println!("into   {:}", stringify(&next));
        }
        if num == next {
            break;
        }
        num = next;
    }
    num
}

fn snailsum(a: Number, b: Number) -> Number {
    reduce(Pair(Box::new(a), Box::new(b)))
}

fn sum_numbers(numbers: &[Number]) -> Number {
    let first = numbers[0].clone();
    numbers.iter().skip(1).fold(first, |work, next| snailsum(work, next.clone()))
}

fn sum_magnitude(numbers: &[Number]) -> i32 {
    magnitude(&sum_numbers(numbers))
}

fn largest_pair_sum_magnitude(numbers: &[Number]) -> i32 {
    numbers.iter().flat_map(|a| {
        numbers.iter().filter(move |&b| b != a).map(move |b| {
            magnitude(&snailsum(a.clone(), b.clone()))
        })
    }).max().unwrap()
}

fn parse_element(chs: &mut Chars) -> Number {
    match chs.next().unwrap() {
        '[' => parse_pair(chs),
        n @ '0' ..= '9' => Regular((n as u8 - b'0') as i32),
        _ => panic!("pair broken"),
    }
}

fn parse_pair(chs: &mut Chars) -> Number {
    let left = parse_element(chs);
    let comma = chs.next().unwrap();
    assert_eq!(comma, ',');
    let right = parse_element(chs);
    let close = chs.next().unwrap();
    assert_eq!(close, ']');
    Pair(Box::new(left), Box::new(right))
}

fn parse_whole_number(line: &str) -> Number {
    let mut chs = line.chars();
    let open = chs.next().unwrap();
    assert_eq!(open, '[');
    parse_pair(&mut chs)
}

fn main() {
    assert_eq!(explosion(parse_whole_number("[[[[[9,8],1],2],3],4]")), parse_whole_number("[[[[0,9],2],3],4]"));
    assert_eq!(explosion(parse_whole_number("[7,[6,[5,[4,[3,2]]]]]")), parse_whole_number("[7,[6,[5,[7,0]]]]"));
    assert_eq!(explosion(parse_whole_number("[[6,[5,[4,[3,2]]]],1]")), parse_whole_number("[[6,[5,[7,0]]],3]"));
    assert_eq!(explosion(parse_whole_number("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]")), parse_whole_number("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"));
    assert_eq!(explosion(parse_whole_number("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]")), parse_whole_number("[[3,[2,[8,0]]],[9,[5,[7,0]]]]"));

    assert_eq!(split(Regular(10)), parse_whole_number("[5,5]"));
    assert_eq!(split(Regular(11)), parse_whole_number("[5,6]"));
    assert_eq!(split(Regular(12)), parse_whole_number("[6,6]"));

    assert_eq!(snailsum(parse_whole_number("[[[[4,3],4],4],[7,[[8,4],9]]]"), parse_whole_number("[1,1]")), parse_whole_number("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"));

    assert_eq!(magnitude(&parse_whole_number("[9,1]")), 29);
    assert_eq!(magnitude(&parse_whole_number("[1,9]")), 21);
    assert_eq!(magnitude(&parse_whole_number("[[9,1],[1,9]]")), 129);
    assert_eq!(magnitude(&parse_whole_number("[[1,2],[[3,4],5]]")), 143);
    assert_eq!(magnitude(&parse_whole_number("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")), 1384);
    assert_eq!(magnitude(&parse_whole_number("[[[[1,1],[2,2]],[3,3]],[4,4]]")), 445);
    assert_eq!(magnitude(&parse_whole_number("[[[[3,0],[5,3]],[4,4]],[5,5]]")), 791);
    assert_eq!(magnitude(&parse_whole_number("[[[[5,0],[7,4]],[5,5]],[6,6]]")), 1137);
    assert_eq!(magnitude(&parse_whole_number("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")), 3488);

    let numbers: Vec<_> = io::stdin().lock().lines()
        .map(|line| parse_whole_number(&line.unwrap()))
        .collect();
    println!("{:?}", sum_magnitude(&numbers));
    println!("{:?}", largest_pair_sum_magnitude(&numbers));
}
