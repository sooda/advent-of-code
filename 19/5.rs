use std::io::{self, BufRead};

fn step<'a>(program: &'a mut [i32], ip: usize, input: i32) -> Option<(usize, Option<i32>)> {
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
            *mut0(program) = input;
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

fn execute(program: &[i32], input: i32) -> Option<i32> {
    let mut program = program.to_vec();
    let mut ip = 0;
    let mut output = None;

    //println!(". {:?} {} {:?}", &program[..ip], program[ip], &program[ip+1..]);
    while let Some((newip, newout)) = step(&mut program, ip, input) {
        // all outputs must be zero except the final diagnostic code,
        // or the diagnostic program wasn't run successfully
        if newout.is_some() {
            assert_eq!(output.unwrap_or_else(|| 0), 0);
            output = newout;
        }

        ip = newip;
        //println!("  {:?} {} {:?}", &program[..ip], program[ip], &program[ip+1..]);
    }

    output
}

fn main() {
    assert_eq!(execute(&[1002,4,3,4,33], 1), None);
    assert_eq!(execute(&[1101,100,-1,4,0], 1), None);

    assert_eq!(execute(&[3,9,8,9,10,9,4,9,99,-1,8], 7), Some(0));
    assert_eq!(execute(&[3,9,8,9,10,9,4,9,99,-1,8], 8), Some(1));
    assert_eq!(execute(&[3,9,8,9,10,9,4,9,99,-1,8], 9), Some(0));

    assert_eq!(execute(&[3,9,7,9,10,9,4,9,99,-1,8], 7), Some(1));
    assert_eq!(execute(&[3,9,7,9,10,9,4,9,99,-1,8], 8), Some(0));
    assert_eq!(execute(&[3,9,7,9,10,9,4,9,99,-1,8], 9), Some(0));

    assert_eq!(execute(&[3,3,1108,-1,8,3,4,3,99], 7), Some(0));
    assert_eq!(execute(&[3,3,1108,-1,8,3,4,3,99], 8), Some(1));
    assert_eq!(execute(&[3,3,1108,-1,8,3,4,3,99], 9), Some(0));

    assert_eq!(execute(&[3,3,1107,-1,8,3,4,3,99], 7), Some(1));
    assert_eq!(execute(&[3,3,1107,-1,8,3,4,3,99], 8), Some(0));
    assert_eq!(execute(&[3,3,1107,-1,8,3,4,3,99], 9), Some(0));

    assert_eq!(execute(&[3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], 0), Some(0));
    assert_eq!(execute(&[3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], 42), Some(1));

    assert_eq!(execute(&[3,3,1105,-1,9,1101,0,0,12,4,12,99,1], 0), Some(0));
    assert_eq!(execute(&[3,3,1105,-1,9,1101,0,0,12,4,12,99,1], 42), Some(1));

    assert_eq!(execute(&[3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
            1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
            999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], 8), Some(1000));

    assert_eq!(execute(&[3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
            1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
            999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], 7), Some(999));

    assert_eq!(execute(&[3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
            1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
            999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], 9), Some(1001));

    let program: Vec<i32> = io::stdin().lock().lines().next().unwrap().unwrap()
        .split(',').map(|n| n.parse().unwrap()).collect();

    println!("{:?}", execute(&program, 1));
    println!("{:?}", execute(&program, 5));
}
