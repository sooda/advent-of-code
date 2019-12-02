use std::io::{self, BufRead};

fn step(program: &mut [u32], ip: usize) -> Option<usize> {
    let opcode = program[ip];
    if opcode == 99 {
        None
    } else {
        let a = program[program[ip + 1] as usize];
        let b = program[program[ip + 2] as usize];
        let dest = &mut program[program[ip + 3] as usize];
        match opcode {
            1 => {
                *dest = a + b;
            },
            2 => {
                *dest = a * b;
            },
            _ => panic!("something went wrong")
        };
        Some(ip + 4)
    }
}

fn execute(program: &[u32]) -> u32 {
    let mut program = program.to_vec();
    let mut ip = 0;

    while let Some(newip) = step(&mut program, ip) {
        ip = newip;
    }

    program[0]
}

fn exec_input(program: &[u32], noun: u32, verb: u32) -> u32 {
    let mut program = program.to_vec();

    program[1] = noun;
    program[2] = verb;

    execute(&mut program)
}

fn bruteforce(program: &[u32], output: u32) -> u32 {
    // the puzzle answer is 100 * noun + verb which is conveniently the position in this flattened
    // list of 2d search parameters: for each noun we have 100 verbs
    (0..100).flat_map(|noun| (0..100).map(move |verb| (noun, verb)))
        .position(|(noun, verb)| exec_input(&program, noun, verb) == output)
        .unwrap() as u32
}

fn main() {
    assert_eq!(execute(&[1,9,10,3,2,3,11,0,99,30,40,50]), 3500);
    assert_eq!(execute(&[1,0,0,0,99]), 2);
    assert_eq!(execute(&[2,3,0,3,99]), 2);
    assert_eq!(execute(&[2,4,4,5,99,0]), 2);
    assert_eq!(execute(&[1,1,1,4,99,5,6,0,99]), 30);

    let program: Vec<u32> = io::stdin().lock().lines().next().unwrap().unwrap()
        .split(',').map(|n| n.parse().unwrap()).collect();

    println!("{}", exec_input(&program, 12, 2));
    println!("{}", bruteforce(&program, 19690720));
}
