use std::io::{self, BufRead};

fn step<'a, 'b, I: Iterator<Item = &'b i64>>(program: &'a mut [i64], ip: usize, input: &mut I) -> Option<(usize, Option<i64>)> {
    let opcode = program[ip] % 100;
    if opcode == 99 {
        // short circuit this, the discontinuity is annoying
        return None;
    }

    let mode0 = program[ip] / 100 % 10;
    let mode1 = program[ip] / 1000 % 10;
    let mode2 = program[ip] / 10000 % 10;
    assert!(mode0 <= 1);
    assert!(mode1 <= 1);
    assert!(mode2 <= 1);
    let immflags = (mode0 == 1, mode1 == 1, mode2 == 1);

    // indirection via closures instead of direct variables because indexing might go off with
    // arbitrary values: only evaluate when it's known that indexing is okay. Need this to avoid
    // repetition in the opcode decoding

    let imm0 = || program[ip + 1];
    let imm1 = || program[ip + 2];
    let val0 = || if immflags.0 { imm0() } else { program[imm0() as usize ] };
    let val1 = || if immflags.1 { imm1() } else { program[imm1() as usize ] };
    // program as input for lifetime; using imm0 here would cause more lifetime trouble
    let mut0 = |program: &'a mut [i64]| { assert!(!immflags.0); &mut program[program[ip + 1] as usize] };
    let mut2 = |program: &'a mut [i64]| { assert!(!immflags.2); &mut program[program[ip + 3] as usize] };

    match opcode {
        1 => {
            *mut2(program) = val0() + val1();
            Some((ip + 4, None))
        },
        2 => {
            *mut2(program) = val0() * val1();
            Some((ip + 4, None))
        },
        3 => {
            *mut0(program) = *input.next().unwrap();
            Some((ip + 2, None))
        }
        4 => {
            Some((ip + 2, Some(val0())))
        },
        5 => {
            if val0() != 0 {
                Some((val1() as usize, None))
            } else {
                Some((ip + 3, None))
            }
        },
        6 => {
            if val0() == 0 {
                Some((val1() as usize, None))
            } else {
                Some((ip + 3, None))
            }
        },
        7 => {
            *mut2(program) = if val0() < val1() { 1 } else { 0 };
            Some((ip + 4, None))
        },
        8 => {
            *mut2(program) = if val0() == val1() { 1 } else { 0 };
            Some((ip + 4, None))
        },
        _ => panic!("something went wrong at {}: {}", ip, program[ip])
    }
}

fn execute(program: &[i64], inputs: &[i64]) -> Option<i64> {
    let mut program = program.to_vec();
    let mut ip = 0;
    let mut output = None;
    let mut input = inputs.iter();

    while let Some((newip, newout)) = step(&mut program, ip, &mut input) {
        output = newout;
        ip = newip;
    }

    output
}

fn swap_codes(a: i64, i: usize, j: usize) -> i64 {
    let ai = (a >> (4 * i)) & 0xf;
    let aj = (a >> (4 * j)) & 0xf;
    let mi = !(0xf << (4 * i));
    let mj = !(0xf << (4 * j));

    let b = a & mi & mj | (ai << (4 * j)) | (aj << (4 * i));
    b
}

fn permutations(k: usize, a: &mut i64, out: &mut Vec<i64>) {
    if k == 1 {
        assert!(!out.contains(a));
        out.push(*a);
    } else {
        permutations(k - 1, a, out);
        for i in 0..k-1 {
            if k & 1 == 0 {
                *a = swap_codes(*a, i, k - 1);
            } else {
                *a = swap_codes(*a, 0, k - 1);
            }
            permutations(k - 1, a, out);
        }
    }
}

fn thruster_signal(program: &[i64], mut phasecode: i64) -> i64 {
    let mut prev = 0;
    let mut x = Vec::new();
    for _amp in 0..5 { // A..E
        let phase = phasecode & 0xf;
        phasecode >>= 4;
        x.push(phase);
        let out = execute(program, &[phase, prev]).unwrap();
        prev = out;
    }
    prev
}

fn max_thruster_signal(program: &[i64]) -> i64 {
    let mut pers = Vec::new();
    // 5 digits, each is 0..4, encoded in i64
    permutations(5, &mut 0x43210, &mut pers);
    assert_eq!(pers.len(), 120);
    pers.iter().map(|&phasecode| (phasecode, thruster_signal(program, phasecode)))
        .max_by_key(|&(_, sig)| sig).unwrap().1
}

struct Computer<'a> {
    program: &'a mut [i64],
    ip: usize
}

fn drive_output(machine: &mut Computer, inputs: &[i64]) -> Option<i64> {
    let program = &mut machine.program;
    let mut ip = machine.ip;
    let mut input = inputs.iter();

    while let Some((newip, out)) = step(program, ip, &mut input) {
        ip = newip;
        if out.is_some() {
            machine.ip = ip;
            return out;
        }
    }

    // stopped without input
    None
}

fn thruster_signal_loop(program: &[i64], mut phasecode: i64) -> i64 {
    let amps = &mut [
        Computer { program: &mut program.to_vec(), ip: 0 },
        Computer { program: &mut program.to_vec(), ip: 0 },
        Computer { program: &mut program.to_vec(), ip: 0 },
        Computer { program: &mut program.to_vec(), ip: 0 },
        Computer { program: &mut program.to_vec(), ip: 0 },
    ];
    let mut prev = 0;

    // prime the phases in
    for mut amp in amps.iter_mut() {
        let phase = phasecode & 0xf;
        phasecode >>= 4;
        let next = drive_output(&mut amp, &[phase, prev]);
        prev = next.unwrap();
    }

    // now the programs only input one thing and output another thing
    'outer: loop {
        for (i, mut amp) in amps.iter_mut().enumerate() {
            match drive_output(&mut amp, &[prev]) {
                Some(next) => prev = next,
                None => {
                    // must stop at the end of a full iteration
                    assert!(i == 0);
                    break 'outer;
                }
            }
        }
    }
    prev
}

fn max_thruster_signal_loop(program: &[i64]) -> i64 {
    let mut pers = Vec::new();
    // 5 digits, each is 0..4, encoded in i64
    permutations(5, &mut 0x98765, &mut pers);
    assert_eq!(pers.len(), 120);
    pers.iter().map(|&phasecode| (phasecode, thruster_signal_loop(program, phasecode)))
        .max_by_key(|&(_, sig)| sig).unwrap().1
}

fn main() {
    assert_eq!(swap_codes(0x12345, 0, 1), 0x12354);
    assert_eq!(swap_codes(0x12345, 1, 3), 0x14325);

    assert_eq!(max_thruster_signal(&[3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0]), 43210);
    assert_eq!(max_thruster_signal(&[3,23,3,24,1002,24,10,24,1002,23,-1,23,
            101,5,23,23,1,24,23,23,4,23,99,0,0]), 54321);
    assert_eq!(max_thruster_signal(&[3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,
            1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0]), 65210);

    assert_eq!(thruster_signal_loop(&[
            3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,
            27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5], 0x56789), 139629729);
    assert_eq!(max_thruster_signal_loop(&[
            3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,
            27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5]), 139629729);

    assert_eq!(max_thruster_signal_loop(&[
            3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,
            -5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,
            53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10]), 18216);

    let program: Vec<i64> = io::stdin().lock().lines().next().unwrap().unwrap()
        .split(',').map(|n| n.parse().unwrap()).collect();

    println!("{}", max_thruster_signal(&program));
    println!("{}", max_thruster_signal_loop(&program));
}
