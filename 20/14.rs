use std::io::{self, BufRead};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
struct MaskData {
    retained_bits: u64,
    value: u64,
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Mask(MaskData),
    MemOp { addr: u64, value: u64 },
}
use Instruction::*;

#[derive(Debug)]
struct InstructionParseError;

impl FromStr for Instruction {
    type Err = InstructionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if s.starts_with("mask") {
            // "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"
            let data = s.split(" = ").skip(1).next().unwrap();
            let retained_bits = data.chars().fold(0, |acc, bit| {
                (acc << 1) + if bit == 'X' { 1 } else { 0 }
            });
            let value = data.chars().fold(0, |acc, bit| {
                (acc << 1) + if bit == '1' { 1 } else { 0 }
            });
            Mask(MaskData {
                retained_bits,
                value
            })
        } else {
            // "mem[8] = 11"
            let mut sp = s.split("] = ");
            let addr = sp.next().unwrap().split('[').skip(1).next().unwrap().parse().unwrap();
            let value = sp.next().unwrap().parse().unwrap();
            MemOp {
                addr,
                value
            }
        })
    }
}

fn apply_mask(value: u64, mask: MaskData) -> u64 {
    (value & mask.retained_bits) | mask.value
}

fn execute_docking(program: &[Instruction]) -> u64 {
    let mut mem: HashMap<u64, u64> = HashMap::new();
    let mut current_mask = MaskData { retained_bits: 0, value: 0 };
    for &instruction in program {
        match instruction {
            Mask(data) => current_mask = data,
            MemOp { addr, value } => {
                mem.insert(addr, apply_mask(value, current_mask));
            },
        }
    }
    mem.values().sum()
}

fn main() {
    let program: Vec<Instruction> = io::stdin().lock().lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();
    println!("{}", execute_docking(&program));
}
