use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use std::str::FromStr;

use std::collections::HashMap;

extern crate regex;
use regex::Regex;

#[derive(Copy, Clone)]
enum CompareOp {
    Lt,
    Gt,
    Geq,
    Equal,
    Leq,
    Neq
}
fn compare(cmp: CompareOp, reg: i32, val: i32) -> bool {
    match cmp {
        Lt =>    reg <  val,
        Gt =>    reg >  val,
        Geq =>   reg >= val,
        Equal => reg == val,
        Leq =>   reg <= val,
        Neq =>   reg != val
    }
}

use CompareOp::*;

#[derive(Debug)]
struct CompareOpParseError { }

impl FromStr for CompareOp {
    type Err = CompareOpParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "<" => Ok(Lt),
            ">" => Ok(Gt),
            ">=" => Ok(Geq),
            "==" => Ok(Equal),
            "<=" => Ok(Leq),
            "!=" => Ok(Neq),
            _ => Err(Self::Err { })
        }
    }
}

struct Instruction {
    target_reg: String,
    addition: i32,
    compare_reg: String,
    compare_op: CompareOp,
    compare_value: i32
}

fn parse_line(line: &str) -> Instruction {
    let re = Regex::new(r"([a-z]+) (inc|dec) ([\d-]+) if ([a-z]+) ([<=>!]+) ([\d-]+)").unwrap();
    let cap = re.captures(line).unwrap();
    let modification = cap.get(3).unwrap().as_str().parse().unwrap();
    Instruction {
        target_reg: cap.get(1).unwrap().as_str().to_string(),
        addition: if cap.get(2).unwrap().as_str() == "inc" { modification } else { -modification },
        compare_reg: cap.get(4).unwrap().as_str().to_string(),
        compare_op: cap.get(5).unwrap().as_str().parse().unwrap(),
        compare_value: cap.get(6).unwrap().as_str().parse().unwrap(),
    }
}

fn run(program: &[Instruction], memory: &mut HashMap<String, i32>) {
    for ref p in program.iter() {
        let reg = match memory.get(&p.compare_reg) {
            Some(v) => *v,
            None => 0
        };
        if compare(p.compare_op, reg, p.compare_value) {
            *memory.entry(p.target_reg.clone()).or_insert(0) += p.addition;
        }
    }
}

fn main() {
    let program = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap())
        .lines().map(|x| parse_line(&x.unwrap())).collect::<Vec<_>>();
    let mut memory: HashMap<String, i32> = HashMap::new();
    run(&program, &mut memory);
    println!("{:?}", memory);
    println!("{}", memory.iter().map(|(_, &v)| v).max().unwrap());
}
