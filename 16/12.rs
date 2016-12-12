use std::fs::File;
use std::io::Read;

fn readfile(name: &str) -> String {
    let mut f = File::open(name).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    s
}

// return program counter diff
fn action(input: &str, regs: &mut [i64; 4]) -> i64 {
    //println!("{:?} {:?}", input, regs);
    let ops = input.split(" ").collect::<Vec<_>>();
    match ops[0] {
        "cpy" => {
            if let Ok(not_reg_but_number) = ops[1].parse::<i64>() {
                regs[(ops[2].as_bytes()[0] - ('a' as u8)) as usize] = not_reg_but_number;
            } else {
                regs[(ops[2].as_bytes()[0] - ('a' as u8)) as usize] =
                    regs[(ops[1].as_bytes()[0] - ('a' as u8)) as usize];
            }
        },
        "inc" => { regs[(ops[1].as_bytes()[0] - ('a' as u8)) as usize] += 1; },
        "dec" => { regs[(ops[1].as_bytes()[0] - ('a' as u8)) as usize] -= 1; },
        "jnz" => {
            if let Ok(not_reg_but_number) = ops[1].parse::<i64>() {
                if not_reg_but_number != 0 {
                    return ops[2].parse().unwrap();
                }
            } else if regs[(ops[1].as_bytes()[0] - ('a' as u8)) as usize] != 0 {
                return ops[2].parse().unwrap();
            }
        },
        _ => unreachable!()
    }

    1
}

fn main() {
    let src = readfile(&std::env::args().nth(1).unwrap());
    let mut regs = [0i64; 4];
    let mut pc = 0i64;
    let ops = src.trim().split("\n").collect::<Vec<_>>();
    while (pc as usize) != ops.len() {
        pc += action(ops[pc as usize], &mut regs);
    }
    println!("{:?}", regs);
}


