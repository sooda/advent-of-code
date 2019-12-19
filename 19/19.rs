use std::io::{self, BufRead};
use std::collections::HashMap;

fn step<'a, 'b, I: Iterator<Item = &'b i64>>(program: &'a mut [i64], ip: usize, base: i64, input: &mut I) -> Option<(usize, i64, Option<i64>)> {
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
            Some((ip + 4, base, None))
        },
        2 => {
            *mut2(program) = val0() * val1();
            Some((ip + 4, base, None))
        },
        3 => {
            *mut0(program) = *input.next().unwrap();
            Some((ip + 2, base, None))
        }
        4 => {
            Some((ip + 2, base, Some(val0())))
        },
        5 => {
            if val0() != 0 {
                Some((val1() as usize, base, None))
            } else {
                Some((ip + 3, base, None))
            }
        },
        6 => {
            if val0() == 0 {
                Some((val1() as usize, base, None))
            } else {
                Some((ip + 3, base, None))
            }
        },
        7 => {
            *mut2(program) = if val0() < val1() { 1 } else { 0 };
            Some((ip + 4, base, None))
        },
        8 => {
            *mut2(program) = if val0() == val1() { 1 } else { 0 };
            Some((ip + 4, base, None))
        },
        9 => {
            Some((ip + 2, base + val0(), None))
        },
        _ => panic!("something went wrong at {}: {}", ip, program[ip])
    }
}

#[derive(Debug, Clone)]
struct Computer {
    program: Vec<i64>,
    ip: usize,
    base: i64,
}

fn execute(computer: &mut Computer, inputs: &[i64]) -> Option<i64> {
    let mut input = inputs.iter();
    while let Some((newip, newbase, newout)) =
            step(&mut computer.program, computer.ip, computer.base, &mut input) {
        computer.ip = newip;
        computer.base = newbase;
        if newout.is_some() {
            return newout;
        }
    }
    None
}

type Grid = HashMap<(i64, i64), bool>;

fn dump(grid: &Grid) {
    println!("<map>");
    let minx = grid.keys().map(|&(x, _)| x).min().unwrap();
    let maxx = grid.keys().map(|&(x, _)| x).max().unwrap();
    let miny = grid.keys().map(|&(_, y)| y).min().unwrap();
    let maxy = grid.keys().map(|&(_, y)| y).max().unwrap();
    for y in miny..=maxy {
        for x in minx..=maxx {
            let ch = match *grid.get(&(x, y)).unwrap() {
                true => '#',
                false => '.',
            };
            print!("{}", ch);
        }
        println!();
    }
    println!("</map>");
}

fn scan(computer: &Computer, grid: &mut Grid, x0: i64, y0: i64, x1: i64, y1: i64) {
    for y in y0..=y1 {
        for x in x0..=x1 {
            grid.insert((x, y), execute(&mut computer.clone(), &[x, y]).unwrap() == 1);
        }
    }
}

fn analyze_beam(program: &[i64]) -> usize {
    let mut prog = program.to_vec();
    prog.resize(prog.len() + 1000, 0);
    let computer = Computer {
        program: prog,
        ip: 0,
        base: 0
    };

    let mut grid = HashMap::new();
    scan(&computer, &mut grid, 0, 0, 49, 49);
    dump(&grid);
    grid.values().filter(|&&v| v).count()
}

fn main() {
    let program: Vec<i64> = io::stdin().lock().lines().next().unwrap().unwrap()
        .split(',').map(|n| n.parse().unwrap()).collect();

    println!("{:?}", analyze_beam(&program));
}
