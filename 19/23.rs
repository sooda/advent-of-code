use std::io::{self, BufRead};
use std::collections::VecDeque;

const NIC_COUNT: usize = 50;
const ANSWER_ADDRESS: i64 = 255;

#[derive(PartialEq, Eq)]
enum StepState {
    Step,
    Input,
    Output(i64),
    Stop
}
use StepState::*;

fn step<'a, I: Iterator<Item = i64>>(program: &'a mut [i64], ip: usize, base: i64, input: &mut I) -> Option<(usize, i64, Option<i64>, bool)> {
    let opcode = program[ip] % 100;
    if opcode == 99 {
        return None;
    }
    let mode0 = program[ip] / 100 % 10;
    let mode1 = program[ip] / 1000 % 10;
    let mode2 = program[ip] / 10000 % 10;
    assert!(mode0 <= 2);
    assert!(mode1 <= 2);
    assert!(mode2 <= 2);
    let immflags = (mode0 == 1, mode1 == 1, mode2 == 1);
    let relflags = (mode0 == 2, mode1 == 2, mode2 == 2);

    let rel0 = if relflags.0 { base } else { 0 };
    let rel1 = if relflags.1 { base } else { 0 };
    let rel2 = if relflags.2 { base } else { 0 };
    let imm0 = || program[ip + 1];
    let imm1 = || program[ip + 2];
    let val0 = || if immflags.0 { imm0() } else { program[(imm0() + rel0) as usize ] };
    let val1 = || if immflags.1 { imm1() } else { program[(imm1() + rel1) as usize ] };

    let mut0 = |program: &'a mut [i64]| {
        assert!(!immflags.0); &mut program[(program[ip + 1] + rel0) as usize] };
    let mut2 = |program: &'a mut [i64]| {
        assert!(!immflags.2); &mut program[(program[ip + 3] + rel2) as usize] };

    match opcode {
        1 => {
            *mut2(program) = val0() + val1();
            Some((ip + 4, base, None, false))
        },
        2 => {
            *mut2(program) = val0() * val1();
            Some((ip + 4, base, None, false))
        },
        3 => {
            *mut0(program) = input.next().unwrap();
            Some((ip + 2, base, None, true))
        }
        4 => {
            Some((ip + 2, base, Some(val0()), false))
        },
        5 => {
            if val0() != 0 {
                Some((val1() as usize, base, None, false))
            } else {
                Some((ip + 3, base, None, false))
            }
        },
        6 => {
            if val0() == 0 {
                Some((val1() as usize, base, None, false))
            } else {
                Some((ip + 3, base, None, false))
            }
        },
        7 => {
            *mut2(program) = if val0() < val1() { 1 } else { 0 };
            Some((ip + 4, base, None, false))
        },
        8 => {
            *mut2(program) = if val0() == val1() { 1 } else { 0 };
            Some((ip + 4, base, None, false))
        },
        9 => {
            Some((ip + 2, base + val0(), None, false))
        },
        _ => panic!("something went wrong at {}: {}", ip, program[ip])
    }
}

#[derive(Clone)]
struct Computer {
    program: Vec<i64>,
    ip: usize,
    base: i64,
}

fn drive_io<'a, I: Iterator<Item = i64>>(computer: &'a mut Computer, input: &mut I) -> StepState {
    if let Some((newip, newbase, newout, inputted)) =
            step(&mut computer.program, computer.ip, computer.base, input) {
        computer.ip = newip;
        computer.base = newbase;
        if let Some(out) = newout {
            assert!(!inputted);
            Output(out)
        } else if inputted {
            Input
        } else {
            Step
        }
    } else {
        // stopped without input
        Stop
    }
}

fn drive_i(computer: &mut Computer, input: i64) {
    loop {
        let state = drive_io(computer, &mut [input].iter().cloned());
        if state == Input {
            return;
        }
        assert!(state == Step);
    }
}

fn drive_o(computer: &mut Computer) -> i64 {
    loop {
        let state = drive_io(computer, &mut [].into_iter().cloned());
        if let Output(data) = state {
            return data;
        }
        assert!(state == Step);
    }
}

type Address = i64;
type Packet = (i64, i64);

fn receive_packet(nic: &mut Computer, address: Address, queues: &mut [VecDeque<Packet>]) -> Option<i64> {
    let x = drive_o(nic);
    let y = drive_o(nic);
    if address == ANSWER_ADDRESS {
        Some(y)
    } else {
        queues[address as usize].push_back((x, y));
        None
    }
}

fn transmit_value(nic: &mut Computer, value: i64, _queues: &mut [VecDeque<Packet>]) {
    loop {
        match drive_io(nic, &mut ([value]).into_iter().cloned()) {
            Step => {
                // continue
            },
            Input => {
                // boarding completed
                return;
            },
            Output(_address) => {
                // this would be the start of a packet leaving this nic in between parsing an
                // incoming packet to it. can this happen here? the spec is ambiguous.
                panic!();
                //receive_packet(nic, address, queues);
            },
            Stop => {
                panic!("stopped while transmitting");
            }
        }
    }
}

// transmit a packet *to* this NIC
fn transmit_packet(nic: &mut Computer, packet: Packet, queues: &mut [VecDeque<Packet>]) {
    transmit_value(nic, packet.0, queues);
    transmit_value(nic, packet.1, queues);
}

// listen for a packet *from* this NIC
fn listen_packet(nic: &mut Computer, queues: &mut [VecDeque<Packet>]) -> Option<i64> {
    match drive_io(nic, &mut [-1].into_iter().cycle().cloned()) {
        Step => {
            // continue
            None
        },
        Input => {
            // continue, consumed -1
            None
        },
        Output(address) => {
            receive_packet(nic, address, queues)
        },
        Stop => {
            panic!("stopped, not sure what to do");
        }
    }
}

fn run_one(nics: &mut [Computer], queues: &mut [VecDeque<Packet>]) -> Option<i64> {
    // can't zip() because the push_back below would cause another borrow
    for (i, nic) in nics.iter_mut().enumerate() {
        if let Some(packet) = queues[i].pop_front() {
            transmit_packet(nic, packet, queues);
        } else {
            if let Some(stopdata) = listen_packet(nic, queues) {
                return Some(stopdata);
            }
        }
    }
    None
}

fn run(nics: &mut [Computer], queues: &mut [VecDeque<Packet>]) -> i64 {
    loop {
        if let Some(out) = run_one(nics, queues) {
            return out;
        }
    }
}

fn first_packet_to_255(program: &[i64]) -> i64 {
    let mut prog = program.to_vec();
    prog.resize(prog.len() + prog.len(), 0);

    let mut nics = Vec::new();
    let mut queues = Vec::new();
    for i in 0..NIC_COUNT {
        let mut nic = Computer {
            program: prog.clone(),
            ip: 0,
            base: 0
        };
        drive_i(&mut nic, i as i64);
        nics.push(nic);

        queues.push(VecDeque::new());
    }

    run(&mut nics, &mut queues)
}

fn main() {
    let program: Vec<i64> = io::stdin().lock().lines().next().unwrap().unwrap()
        .split(',').map(|n| n.parse().unwrap()).collect();

    println!("{}", first_packet_to_255(&program));
}
