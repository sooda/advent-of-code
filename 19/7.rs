use std::io::{self, BufRead};

fn step<'a, 'b, I: Iterator<Item = &'b i32>>(program: &'a mut [i32], ip: usize, input: &mut I) -> Option<(usize, Option<i32>)> {
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
    let mut0 = |program: &'a mut [i32]| { assert!(!immflags.0); &mut program[program[ip + 1] as usize] };
    let mut2 = |program: &'a mut [i32]| { assert!(!immflags.2); &mut program[program[ip + 3] as usize] };

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

fn execute(program: &[i32], inputs: &[i32]) -> Option<i32> {
    let mut program = program.to_vec();
    let mut ip = 0;
    let mut output = None;
    let mut input = inputs.iter();

    //println!(". {:?} {} {:?}", &program[..ip], program[ip], &program[ip+1..]);
    while let Some((newip, newout)) = step(&mut program, ip, &mut input) {
        output = newout;
        ip = newip;
        //println!("  {:?} {} {:?}", &program[..ip], program[ip], &program[ip+1..]);
    }

    output
}

fn swap_codes(a: i32, i: usize, j: usize) -> i32 {
    let ai = (a >> (4 * i)) & 0xf;
    let aj = (a >> (4 * j)) & 0xf;
    let mi = !(0xf << (4 * i));
    let mj = !(0xf << (4 * j));

    let b = a & mi & mj | (ai << (4 * j)) | (aj << (4 * i));
    b
}

fn permutations(k: usize, a: &mut i32, out: &mut Vec<i32>) {
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

fn thruster_signal(program: &[i32], mut phasecode: i32) -> i32 {
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

fn max_thruster_signal(program: &[i32]) -> i32 {
    let mut pers = Vec::new();
    // 5 digits, each is 0..4, encoded in i32
    permutations(5, &mut 0x43210, &mut pers);
    assert_eq!(pers.len(), 120);
    pers.iter().map(|&phasecode| (phasecode, thruster_signal(program, phasecode)))
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

    let program: Vec<i32> = io::stdin().lock().lines().next().unwrap().unwrap()
        .split(',').map(|n| n.parse().unwrap()).collect();

    println!("{}", max_thruster_signal(&program));
}
