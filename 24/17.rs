use std::io::{self, Read};
use std::collections::HashSet;

/*
 * Program: 2,4, 1,5, 7,5, 0,3, 4,1, 1,6, 5,5, 3,0
 * b = a & 7
 * b = b ^ 5
 * c = a / (1 << b)
 * a = a / 8
 * b = b ^ c
 * b = b ^ 6
 * output b & 7
 * if a != 0 goto start
 *
 * do {
 *     c = a / (1 << ((a % 8) ^ 5))
 *     b = (a % 8) ^ c ^ 3   // b = (a % 8) ^ 5 ^ c ^ 6
 *     a = a / 8
 *     output b & 7
 * } while (a != 0);
 *
 * - clearly one output per three bits (0..7)
 * - output is affected by low bits in a % 8 and whole thing in a / ...
 * - whole thing is divided by 2**(itself%8) and then % 8 which limits variation
 * - may need to try also 8 variations in steps of 2**7=128, and 8*128=1024, 10 bits
 */

fn executefast(i: i64) -> Vec<i64> {
    let mut a = i;
    let mut b;
    let mut c;
    let mut out = Vec::new();
    loop {
        b = a & 7;
        b = b ^ 5;
        c = a / (1 << b);
        a = a / 8;
        b = b ^ c;
        b = b ^ 6;
        out.push((b & 7) as i64);
        if a == 0 {
            break;
        }
    }
    out
}

// each 3 bits produce one digit but consume 10 bits because a/(2**(a%8)) that wraps % 8 after 1023
fn search(computer: &Computer, have: i64, depth: usize, visited: &mut HashSet<(usize, i64)>) -> i64 {
    if !visited.insert((depth, have)) {
        return std::i64::MAX;
    }
    let mut best = std::i64::MAX;
    for i in 0..1024 {
        let next = have + (i << (3 * depth));
        let out = if true {
            run([next, 0, 0], &computer.program).0
        } else {
            // just for my input, to double check things
            executefast(next)
        };

        if out.len() > computer.program.len() {
            // too many digits so i must be too big from now on
            break;
        }

        if out.len() >= depth+1 && out[0..=depth] == computer.program[0..=depth] {
            let regval = if depth == computer.program.len() - 1 {
                next
            } else {
                let next = have + ((i & 7) << (3 * depth));
                search(computer, next, depth + 1, visited)
                // but could be clever and jump to more than depth+1 because this is so may bits
            };
            best = best.min(regval);
        }
    }

    best
}

fn solve(computer: &Computer) -> i64 {
    search(computer, 0, 0, &mut HashSet::new())
}

struct Computer {
    regs: [i64; 3], // a, b, c
    program: Vec<i64>,
}

fn combo(operand: i64, regs: [i64; 3]) -> i64 {
    match operand {
        // possible fun cheat: have 6 regs and make the first three just constants
        0 ..= 3 => operand,
        4 ..= 6 => regs[operand as usize - 4],
        7 => panic!("not valid program"),
        _ => panic!("big operand"),
    }
}

fn execute(computer: &Computer) -> (Vec<i64>, [i64; 3]) {
    run(computer.regs, &computer.program)
}

fn run(mut regs: [i64; 3], program: &[i64]) -> (Vec<i64>, [i64; 3]) {
    let mut output = Vec::new();
    let mut ip = 0;
    while ip as usize + 1 < program.len() {
        let (opcode, operand) = (program[ip as usize], program[ip as usize + 1]);
        let mut ip2 = ip + 2;
        match opcode {
            0 => regs[0] /= 1 << combo(operand, regs),
            1 => regs[1] ^= operand,
            2 => regs[1] = combo(operand, regs) & 7,
            3 => if regs[0] != 0 { ip2 = operand; },
            4 => regs[1] ^= regs[2],
            5 => output.push(combo(operand, regs) & 7),
            6 => regs[1] = regs[0] / (1 << combo(operand, regs)),
            7 => regs[2] = regs[0] / (1 << combo(operand, regs)),
            _ => panic!("big opcode"),
        };
        ip = ip2;
    }
    (output, regs)
}

fn output_str(computer: &Computer) -> String {
    execute(computer).0.iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn parse(file: &str) -> Computer {
    let mut sp = file.split("\n\n");
    let regs = sp.next().unwrap()
        .lines()
        .map(|l| {
            l.split(' ')
                .last().unwrap()
                .parse().unwrap()
        })
    .collect::<Vec<_>>();
    let regs = [regs[0], regs[1], regs[2]];
    let program = sp.next().unwrap()
        .trim_end()
        .split(' ').last().unwrap()
        .split(',').map(|n| n.parse().unwrap())
        .collect();
    Computer { regs, program }
}

fn main() {
    let mut file = String::new();
    io::stdin().read_to_string(&mut file).unwrap();
    let computer = parse(&file);
    assert_eq!(execute(&Computer { regs: [0, 0, 9], program: [2,6].to_vec() }).1[1], 1);
    assert_eq!(execute(&Computer { regs: [10, 0, 0], program: [5,0,5,1,5,4].to_vec() }).0, [0,1,2]);
    assert_eq!(execute(&Computer { regs: [2024, 0, 0], program: [0,1,5,4,3,0].to_vec() }).0, [4,2,5,6,7,7,7,7,3,1,0]);
    assert_eq!(execute(&Computer { regs: [0, 29, 0], program: [1,7].to_vec() }).1[1], 26);
    assert_eq!(execute(&Computer { regs: [0, 2024, 43690], program: [4,0].to_vec() }).1[1], 44354);
    assert_eq!(execute(&Computer { regs: [117440, 0, 0], program: [0,3,5,4,3,0].to_vec() }).0, [0,3,5,4,3,0]);

    println!("{}", output_str(&computer));
    println!("{}", solve(&computer));
}
