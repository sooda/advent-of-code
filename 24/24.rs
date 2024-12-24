#![feature(let_chains)]

use std::io::{self, Read};
use std::collections::HashMap;

#[derive(Debug)]
enum GateOp {
    And,
    Or,
    Xor
}

#[derive(Debug)]
struct Gate {
    op: GateOp,
    in_a: String,
    in_b: String,
    out: String,
}

type States = HashMap<String, bool>;

struct Device {
    states: States,
    gates: Vec<Gate>,
}

fn z_number(states: &States) -> u64 {
    let mut z = 0;
    for (k, &v) in states {
        if v && let Some((_, n)) = k.split_once('z') {
            let n = n.parse::<u64>().unwrap();
            z |= 1 << n;
        }
    }
    z
}

// all your bit are belong to us
fn get_signal(signal: &str, gates: &[Gate], states: &mut States) -> bool {
    if let Some(&val) = states.get(signal) {
        val
    } else {
        let gate = gates.iter().find(|g| g.out == signal).unwrap();
        let a = get_signal(&gate.in_a, gates, states);
        let b = get_signal(&gate.in_b, gates, states);
        let val = match gate.op {
            GateOp::And => a && b,
            GateOp::Or => a || b,
            GateOp::Xor => (a || b) && a != b,
        };
        states.insert(signal.to_string(), val);
        val
    }
}

fn simulate_z(device: &Device) -> u64 {
    let mut states = device.states.clone();
    for g in &device.gates {
        get_signal(&g.out, &device.gates, &mut states);
    }
    z_number(&states)
}

fn parse_gate(line: &str) -> Gate {
    // x00 AND y00 -> z00
    let mut words = line.split(' ');
    let in_a = words.next().unwrap().to_string();
    let op = match words.next().unwrap() {
        "AND" => GateOp::And,
        "OR" => GateOp::Or,
        "XOR" => GateOp::Xor,
        _ => panic!()
    };
    let in_b = words.next().unwrap().to_string();
    words.next().unwrap().to_string();
    let out = words.next().unwrap().to_string();
    Gate { op, in_a, in_b, out }
}

fn parse(file: &str) -> Device {
    let (states, gates) = file.split_once("\n\n").unwrap();
    let states = states.lines()
        .map(|l| l.split_once(": ").unwrap())
        .map(|(a, b)| (a.to_string(), b == "1"))
        .collect();
    let gates = gates.lines()
        .map(parse_gate)
        .collect();
    Device { states, gates }
}

fn main() {
    let mut file = String::new();
    io::stdin().read_to_string(&mut file).unwrap();
    let device = parse(&file);
    println!("{}", simulate_z(&device));
}
