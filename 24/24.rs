#![feature(let_chains)]

use std::io::{self, Read};
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Clone)]
enum GateOp {
    And,
    Or,
    Xor
}

#[derive(Debug, Clone)]
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

fn xy_into_z(x: u64, y: u64) -> States {
    let mut states = States::new();
    for i in 0..64 {
        states.insert(format!("x{i:0>2}"), (x & (1 << i)) != 0);
        states.insert(format!("y{i:0>2}"), (y & (1 << i)) != 0);
    }
    states
}

fn swapped_pairs(device: &Device) -> String {
    if false {
        for ai in 0..45 {
            for bi in 0..45 {
                let a = 1 << ai;
                let b = 1 << bi;
                let z = a + b;
                let states = xy_into_z(a, b);
                if simulate_z(&Device { states, gates: device.gates.clone() }) != z {
                    panic!("boo");
                }
            }
        }
    }

    let mut confusing = HashSet::<String>::new();
    for gate in &device.gates {
        if gate.out.starts_with("z") {
            let x = gate.out.replace("z", "x");
            // this isn't the final carry bit?
            if device.gates.iter().any(|g| g.in_a == x || g.in_b == x) {
                if gate.op != GateOp::Xor {
                    println!("z {} gets input from {:?}", gate.out, gate.op);
                    confusing.insert(gate.out.clone());
                }
            }
        }
        match gate.op {
            // each and goes only to or
            // except the x00&y00 goes to and, Xor
            // that would be the or outputs
            // because the or is optimized away
            // because there is no carry bit yet
            // (skt in my graph)
            GateOp::And => {
                if !(gate.in_a == "x00" || gate.in_a == "y00") {
                    for g in &device.gates {
                        if gate.out == g.in_a || gate.out == g.in_b {
                            if g.op != GateOp::Or {
                                println!("wire {} looks confusing, goes from and to {:?}", gate.out, g.op);
                                confusing.insert(gate.out.clone());
                            }
                        }
                    }
                }
            },
            // each or receives only and
            GateOp::Or => {
                for g in &device.gates {
                    if gate.in_a == g.out || gate.in_b == g.out {
                        if g.op != GateOp::And {
                            println!("wire {} looks confusing, goes to or from {:?}", g.out, g.op);
                            confusing.insert(g.out.clone());
                        }
                    }
                }
            },
            GateOp::Xor => {
                // for a wire connected to a xor:
                // - wire is x or y and input
                // - or wire is z and output
                // - or downstream is xor to z
                // - or upstream is xor from x/y
                // anyway let's try starting from x and y
                // that did not work, in between can be ok but if z isnt z then error
                let xor_in_matches = |name: &str, fun: &mut dyn FnMut(&Gate) -> bool| {
                    for g in &device.gates {
                        if (g.in_a == name || g.in_b == name) && g.op == GateOp::Xor && fun(g) {
                            return true;
                        }
                    }
                    return false;
                };
                if gate.in_a.starts_with("x") || gate.in_a.starts_with("y") {
                    if !xor_in_matches(&gate.out, &mut |xorgate| {
                        if xorgate.out.starts_with("z") {
                            true
                        } else {
                            println!("wire {} looks confusing, is not z", xorgate.out);
                            confusing.insert(xorgate.out.clone());
                            false
                        }
                    }) {
                        // hmm, not too reliable?
                        // println!("wire {} looks confusing, downstream bad", gate.out);
                    }
                }

                let upstream_xor_matches = |g: &Gate, fun: &mut dyn FnMut(&Gate) -> bool| {
                    for parent in &device.gates {
                        if (g.in_a == parent.out || g.in_b == parent.out) && parent.op == GateOp::Xor && fun(parent) {
                            return true;
                        }
                    }
                    return false;
                };

                if gate.out.starts_with("z") {
                    if !upstream_xor_matches(&gate, &mut |xorgate: &Gate| {
                        if xorgate.in_a.starts_with("x") || xorgate.in_a.starts_with("y") {
                            true
                        } else {
                            println!("wire {} looks confusing, is not between input xor and output xor", xorgate.out);
                            confusing.insert(xorgate.out.clone());
                            false
                        }
                    }) {
                        // hmm, not too reliable?
                        //println!("NO x or y found for {:?}", gate);
                    }
                }
            },
        }
    }

    let mut confusing = confusing.into_iter().collect::<Vec<_>>();
    confusing.sort();
    confusing.join(",")
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
    if false {
        println!("digraph G {{");
        for g in &device.gates {
            println!("{} [label={} shape=circle]", g.in_a, g.in_a);
            println!("{} [label={} shape=circle]", g.in_b, g.in_b);
            println!("{} [label={} shape=circle]", g.out, g.out);
            println!("{}{:?}{} [label={:?} shape=rect]", g.in_a, g.op, g.in_b, g.op);
            println!("{} -> {}{:?}{}", g.in_a, g.in_a, g.op, g.in_b);
            println!("{} -> {}{:?}{}", g.in_b, g.in_a, g.op, g.in_b);
            println!("{}{:?}{} -> {}", g.in_a, g.op, g.in_b, g.out);
        }
        println!("}}");
    } else {
        println!("{}", simulate_z(&device));
        println!("{}", swapped_pairs(&device));
    }
}
