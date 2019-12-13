use std::io::{self, BufRead};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball
}
use Tile::*;

fn step<'a, 'b, I: Iterator<Item = &'b i64>>(program: &'a mut [i64], ip: usize, base: i64, input: &mut I) -> Option<(usize, i64, Option<i64>)> {
    let opcode = program[ip] % 100;
    if opcode == 99 {
        // short circuit this, the discontinuity is annoying
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

    // indirection via closures instead of direct variables because indexing might go off with
    // arbitrary values: only evaluate when it's known that indexing is okay. Need this to avoid
    // repetition in the opcode decoding

    let rel0 = if relflags.0 { base } else { 0 };
    let rel1 = if relflags.1 { base } else { 0 };
    let rel2 = if relflags.2 { base } else { 0 };
    let imm0 = || program[ip + 1];
    let imm1 = || program[ip + 2];
    let val0 = || if immflags.0 { imm0() } else { program[(imm0() + rel0) as usize ] };
    let val1 = || if immflags.1 { imm1() } else { program[(imm1() + rel1) as usize ] };
    // program as input for lifetime; using imm0 here would cause more lifetime trouble
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

fn execute(program: &[i64]) -> HashMap<(i64, i64), Tile> {
    let mut panel = HashMap::new();
    let mut program = program.to_vec();
    // FIXME: program should probably be a hashmap, but this works for now
    program.resize(program.len() + 1000, 0);

    let mut ip = 0;
    let mut base = 0;

    let mut x = 0;
    let mut y = 0;

    let mut outmode = 0;
    while let Some((newip, newbase, newout)) =
            step(&mut program, ip, base, &mut [].iter()) {
        if newout.is_some() {
            match outmode {
                0 => {
                    x = newout.unwrap() as i64;
                },
                1 => {
                    y = newout.unwrap() as i64;
                },
                2 => {
                    panel.insert((x, y), match newout.unwrap() {
                        0 => Empty,
                        1 => Wall,
                        2 => Block,
                        3 => Paddle,
                        4 => Ball,
                        _ => panic!("bad tile")
                    });
                },
                _ => unreachable!()
            }
            outmode = (outmode + 1) % 3;
        }
        ip = newip;
        base = newbase;
    }

    panel
}

fn dump(panel: &HashMap<(i64, i64), Tile>) {
    let minx = panel.keys().map(|&(x, _)| x).min().unwrap();
    let maxx = panel.keys().map(|&(x, _)| x).max().unwrap();
    let miny = panel.keys().map(|&(_, y)| y).min().unwrap();
    let maxy = panel.keys().map(|&(_, y)| y).max().unwrap();
    for y in miny..=maxy {
        for x in minx..=maxx {
            let ch = match *panel.get(&(x, y)).unwrap_or(&Empty) {
                Empty => ' ',
                Wall => '*',
                Block => 'x',
                Paddle => '-',
                Ball => 'o',
            };
            print!("{}", ch);
        }
        println!();
    }
}

fn main() {
    let program: Vec<i64> = io::stdin().lock().lines().next().unwrap().unwrap()
        .split(',').map(|n| n.parse().unwrap()).collect();

    let panel = execute(&program);
    println!("{}", panel.values().filter(|&&x| x == Block).count());
    dump(&panel);
}
