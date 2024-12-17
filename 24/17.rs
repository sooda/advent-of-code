use std::io::{self, Read};

struct Computer {
    regs: [i32; 3], // a, b, c
    program: Vec<i32>,
}

fn combo(operand: i32, regs: [i32; 3]) -> i32 {
    match operand {
        0 ..= 3 => operand,
        4 ..= 6 => regs[operand as usize - 4],
        7 => panic!("not valid program"),
        _ => panic!("big operand"),
    }
}

fn execute(computer: Computer) -> (Vec<i32>, [i32; 3]) {
    let mut output = Vec::new();
    let mut regs = computer.regs;
    let mut ip = 0;
    while ip as usize + 1 < computer.program.len() {
        let (opcode, operand) = (computer.program[ip as usize], computer.program[ip as usize + 1]);
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

fn output_str(computer: Computer) -> String {
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
    assert_eq!(execute(Computer { regs: [0, 0, 9], program: [2,6].to_vec() }).1[1], 1);
    assert_eq!(execute(Computer { regs: [10, 0, 0], program: [5,0,5,1,5,4].to_vec() }).0, [0,1,2]);
    assert_eq!(execute(Computer { regs: [2024, 0, 0], program: [0,1,5,4,3,0].to_vec() }).0, [4,2,5,6,7,7,7,7,3,1,0]);
    assert_eq!(execute(Computer { regs: [0, 29, 0], program: [1,7].to_vec() }).1[1], 26);
    assert_eq!(execute(Computer { regs: [0, 2024, 43690], program: [4,0].to_vec() }).1[1], 44354);

    println!("{}", output_str(computer));
}
