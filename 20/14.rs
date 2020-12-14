use std::io::{self, BufRead};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
struct MaskData {
    x_bits: u64,
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
            let x_bits = data.chars().fold(0, |acc, bit| {
                (acc << 1) + if bit == 'X' { 1 } else { 0 }
            });
            let value = data.chars().fold(0, |acc, bit| {
                (acc << 1) + if bit == '1' { 1 } else { 0 }
            });
            Mask(MaskData {
                x_bits,
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
    (value & mask.x_bits) | mask.value
}

fn execute_docking(program: &[Instruction]) -> u64 {
    let mut mem: HashMap<u64, u64> = HashMap::new();
    let mut current_mask = MaskData { x_bits: 0, value: 0 };
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

// PDEP
fn scatter_bits(src: u64, dst_bits: u64) -> u64 {
    let mut dst_remaining = dst_bits;
    let mut scattered_bits = 0;
    let mut src_i = 0;

    while dst_remaining != 0 {
        let dst_i = dst_remaining.trailing_zeros();
        let bit_val = (src >> src_i) & 1;
        scattered_bits |= bit_val << dst_i;
        // xor would avoid the "!" but it's almost too clever for this
        dst_remaining &= !(1 << dst_i);
        src_i += 1;
    }

    scattered_bits
}

fn scatter_mem(mem: &mut HashMap<u64, u64>, base_addr: u64, x_bits: u64, value: u64) {
    let affected_positions = 1 << x_bits.count_ones();
    let base_addr = base_addr & !x_bits;
    for pos_id in 0u64..affected_positions {
        let scattered_bits = scatter_bits(pos_id, x_bits);
        mem.insert(base_addr | scattered_bits, value);
    }
}

fn execute_docking_v2(program: &[Instruction]) -> u64 {
    let mut mem: HashMap<u64, u64> = HashMap::new();
    let mut current_mask = MaskData { x_bits: 0, value: 0 };
    for &instruction in program {
        match instruction {
            Mask(data) => current_mask = data,
            MemOp { addr, value } => {
                scatter_mem(&mut mem, addr | current_mask.value, current_mask.x_bits, value);
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
    println!("{}", execute_docking_v2(&program));
}
