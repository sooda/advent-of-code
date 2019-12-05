use std::io::{self, BufRead};

fn aluop(program: &mut [i32], ip: usize, imm: (bool, bool, bool), op: &dyn Fn(i32, i32) -> i32) {
    let p1 = program[ip + 1];
    let p2 = program[ip + 2];
    let p3 = program[ip + 3];
    let a = if imm.0 { p1 } else { program[p1 as usize] };
    let b = if imm.1 { p2 } else { program[p2 as usize] };
    assert!(!imm.2);
    let dest = &mut program[p3 as usize];
    *dest = op(a, b);
}

fn step(program: &mut [i32], ip: usize, input: i32) -> Option<(usize, Option<i32>)> {
    let opcode = program[ip] % 100;
    let mode1 = program[ip] / 100 % 10;
    let mode2 = program[ip] / 1000 % 10;
    let mode3 = program[ip] / 10000 % 10;
    assert!(mode1 <= 1);
    assert!(mode2 <= 1);
    assert!(mode3 <= 1);
    let immflags = (mode1 == 1, mode2 == 1, mode3 == 1);

    match opcode {
        1 => {
            aluop(program, ip, immflags, &|a, b| a + b);
            Some((ip + 4, None))
        },
        2 => {
            aluop(program, ip, immflags, &|a, b| a * b);
            Some((ip + 4, None))
        },
        3 => {
            let dest = program[ip + 1];
            program[dest as usize] = input;
            Some((ip + 2, None))
        }
        4 => {
            let src = program[ip + 1];
            let out = if immflags.0 { src } else { program[src as usize] };
            Some((ip + 2, Some(out)))
        },
        99 => None,
        _ => panic!("something went wrong at {}: {}", ip, program[ip])
    }
}

fn execute(program: &[i32], input: i32) -> Option<i32> {
    let mut program = program.to_vec();
    let mut ip = 0;
    let mut output = None;

    while let Some((newip, newout)) = step(&mut program, ip, input) {
        // all outputs must be zero except the final diagnostic code,
        // or the diagnostic program wasn't run successfully
        if newout.is_some() {
            assert_eq!(output.unwrap_or_else(|| 0), 0);
        }

        ip = newip;
        output = newout;
    }

    output
}

fn main() {
    assert_eq!(execute(&[1002,4,3,4,33], 1), None);
    assert_eq!(execute(&[1101,100,-1,4,0], 1), None);

    let program: Vec<i32> = io::stdin().lock().lines().next().unwrap().unwrap()
        .split(',').map(|n| n.parse().unwrap()).collect();

    println!("{:?}", execute(&program, 1));
}
