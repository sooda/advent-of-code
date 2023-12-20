use std::io::{self, BufRead};
use std::collections::{HashMap, VecDeque};

enum ModuleKind {
    Flipflop,
    Conjunction,
    Broadcaster,
}
use ModuleKind::*;

struct Module {
    name: String,
    kind: ModuleKind,
    outputs: Vec<String>,
}

enum Memory<'a> {
    Flipflop(bool),
    Conjunction(HashMap<&'a str, bool>),
    Broadcaster(()),
}

fn button<'a>(config: &HashMap<&'a str, &'a Module>, states: &mut HashMap<&'a str, Memory<'a>>) -> (usize, usize) {
    let mut fifo = VecDeque::new();
    fifo.push_back(("button", "broadcaster", false));
    let mut signal_count = [0; 2];
    while let Some((in_name, name, level)) = fifo.pop_front() {
        signal_count[level as usize] += 1;
        let (outputs, next_level) = match config.get(name).map(|m| (&m.kind, &m.outputs)) {
            Some((Flipflop, o)) => {
                if !level {
                    let m = states.get(name).unwrap();
                    if let &Memory::Flipflop(prev) = m {
                        states.insert(name, Memory::Flipflop(!prev));
                        (o, !prev)
                    } else { panic!() } // FIXME bind instance and type better together
                } else {
                    continue;
                }
            },
            Some((Conjunction, o)) => {
                let m = states.get_mut(name).unwrap();
                if let Memory::Conjunction(inps) = m {
                    inps.insert(in_name, level);
                    let pulse = if inps.values().all(|&v| v) {
                        false
                    } else {
                        true
                    };
                    (o, pulse)
                } else { panic!(); } // FIXME same
            },
            Some((Broadcaster, o)) => {
                (o, level)
            },
            None => {
                assert!(name == "output" || name == "rx");
                continue;
            }
        };
        for dest in outputs {
            fifo.push_back((name, dest, next_level));
        }
    }
    (signal_count[0], signal_count[1])
}

fn conj_input_state<'a, 'b>(config: &'a [Module], name: &str) -> HashMap<&'a str, bool> {
    config.iter().
        filter_map(|m| {
            if m.outputs.iter().any(|d| d == name) { Some((&m.name as &str, false)) } else { None }
        })
    .collect()
}

fn pulses(config: &[Module]) -> usize {
    let mut state = config.iter()
        .map(|m| {
            (&m.name as &str, match m.kind {
                Flipflop => Memory::Flipflop(false),
                Conjunction => Memory::Conjunction(conj_input_state(config, &m.name)),
                Broadcaster => Memory::Broadcaster(()),
            })
        })
    .collect();
    let config = config.iter().map(|m| (&m.name as &str, m)).collect::<HashMap<_, _>>();
    let mut pulses_lo = 0;
    let mut pulses_hi = 0;
    for _ in 0..1000 {
        let (lo, hi) = button(&config, &mut state);
        pulses_lo += lo;
        pulses_hi += hi;
    }
    println!("lo {} hi {}", pulses_lo, pulses_hi);
    pulses_lo * pulses_hi
}

fn parse_module(line: &str) -> Module {
    let mut sp = line.split(" -> ");
    let name = sp.next().unwrap();
    let outputs = sp.next().unwrap();
    let outputs = outputs.split(", ").map(|s| s.to_string()).collect();
    let (name, kind) = if name == "broadcaster" {
        (name.to_string(), Broadcaster)
    } else if name.starts_with('%') {
        (name.strip_prefix('%').unwrap().to_string(), Flipflop)
    } else if name.starts_with('&') {
        (name.strip_prefix('&').unwrap().to_string(), Conjunction)
    } else {
        panic!()
    };
    Module { name, kind, outputs }
}

fn main() {
    let config = io::stdin().lock().lines()
        .map(|row| parse_module(&row.unwrap()))
        .collect::<Vec<_>>();
    println!("{}", pulses(&config));
}
