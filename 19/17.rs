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

struct Computer {
    program: Vec<i64>,
    ip: usize,
    base: i64,
}

fn execute(computer: &mut Computer) -> Option<i64> {
    let inputs = &[];
    let mut input = inputs.iter();
    while let Some((newip, newbase, newout)) =
            step(&mut computer.program, computer.ip, computer.base, &mut input) {
        computer.ip = newip;
        computer.base = newbase;
        if newout.is_some() {
            // return *after* updating the ip or the computer will keep replying infinitely... doh.
            return newout;
        }
    }
    None
}

fn read_map(program: &[i64]) -> HashMap<(i64, i64), char> {
    let mut prog = program.to_vec();
    prog.resize(prog.len() + 2000, 0);
    let mut computer = Computer {
        program: prog,
        ip: 0,
        base: 0
    };

    let mut pixels = HashMap::new();
    let mut x = 0;
    let mut y = 0;
    while let Some(out) = execute(&mut computer) {
        let ch = out as u8 as char;
        if ch == '\n' {
            y += 1;
            x = 0;
        } else {
            pixels.insert((x, y), ch);
            x += 1;
        }
    }
    pixels
}

fn dump(pixels: &HashMap<(i64, i64), char>) {
    let minx = pixels.keys().map(|&(x, _)| x).min().unwrap();
    let maxx = pixels.keys().map(|&(x, _)| x).max().unwrap();
    let miny = pixels.keys().map(|&(_, y)| y).min().unwrap();
    let maxy = pixels.keys().map(|&(_, y)| y).max().unwrap();
    for y in miny..=maxy {
        for x in minx..=maxx {
            print!("{}", pixels[&(x, y)]);
        }
        println!();
    }
}

fn alignment_parameters(program: &[i64]) -> i64 {
    let pixels = read_map(program);
    dump(&pixels);

    let mut calib = 0;
    for (&(px, py), &pixel) in &pixels {
        if pixel == '#'
                && pixels.get(&(px - 1, py)) == Some(&'#')
                && pixels.get(&(px + 1, py)) == Some(&'#')
                && pixels.get(&(px, py - 1)) == Some(&'#')
                && pixels.get(&(px, py + 1)) == Some(&'#') {
            calib += px * py;
        }
    }

    calib
}

fn execute_dusting(computer: &mut Computer, inputs: &[i64]) -> Option<i64> {
    let mut input = inputs.iter();
    let mut dust = None;
    while let Some((newip, newbase, newout)) =
            step(&mut computer.program, computer.ip, computer.base, &mut input) {
        computer.ip = newip;
        computer.base = newbase;
        if let Some(out) = newout {
            if out <= 127 {
                // the animation is epic
                print!("{}", newout.unwrap() as u8 as char);
            } else {
                assert!(dust.is_none());
                dust = Some(out);
            }
        }
    }
    dust
}


fn dust_scaffold(program: &[i64], functions: &[&str]) -> i64 {
    let mut prog = program.to_vec();
    prog.resize(prog.len() + 2000, 0);
    prog[0] = 2;
    let mut computer = Computer {
        program: prog,
        ip: 0,
        base: 0
    };
    let input = functions.join("\n");
    let mut input = input.as_bytes().iter().map(|&x| x as i64).collect::<Vec<_>>();
    input.push('\n' as i64);
    input.push('y' as i64);
    input.push('\n' as i64);

    execute_dusting(&mut computer, &input).unwrap()
}

fn walk_scaffold(pixels: &HashMap<(i64, i64), char>) -> Vec<String> {
    let map = |x, y| pixels.get(&(x, y)).copied().unwrap_or('.');
    // the robot seems to be always facing up and at the end of the scaffold
    let (&(mut robot_x, mut robot_y), &_) = pixels.iter().find(|&(&(_, _), &px)| px == '^').unwrap();
    let (mut dx, mut dy) = (0, 1); // dy 1 is up

    let mut trip = Vec::new();
    let mut forward = 0;

    loop {
        let (nx, ny) = (robot_x + dx, robot_y - dy);
        let tile = map(nx, ny);
        if tile == '#' {
            // scaffold continues
            robot_x = nx;
            robot_y = ny;
            forward += 1;
        } else {
            // next step would throw the robot tumbling through space uncontrollably, so turn
            assert!(tile == '.');
            let leftd = (-dy, dx);
            let rightd = (dy, -dx);
            let leftpos = (robot_x + leftd.0, robot_y - leftd.1);
            let rightpos = (robot_x + rightd.0, robot_y - rightd.1);
            let leftway = map(leftpos.0, leftpos.1) == '#';
            let rightway = map(rightpos.0, rightpos.1) == '#';
            if !leftway && !rightway {
                // no way, reached the end
                break;
            }
            // one of these is set; the map contains only turns, no T junctions
            assert_ne!(leftway, rightway);

            // the very first step is likely a turn (is in my input and some samples)
            if forward > 0 {
                // log the walked path
                trip.push(forward.to_string());
                forward = 0;
            }

            // log the turn and do it
            if leftway {
                trip.push('L'.to_string());
                dx = leftd.0;
                dy = leftd.1;
            } else {
                trip.push('R'.to_string());
                dx = rightd.0;
                dy = rightd.1;
            }
        }
    }

    trip
}
fn sweep_clean(program: &[i64]) -> i64 {
    let pixels = read_map(program);
    let steps = walk_scaffold(&pixels);

    // TODO: less manual algorithm
    println!("{}", steps.join(","));
    let functions = &[
        "A,B,A,B,C,B,C,A,B,C",
        "R,4,R,10,R,8,R,4",
        "R,10,R,6,R,4",
        "R,4,L,12,R,6,L,12"
    ];
    // find other robots, notify of the impending solar flare, and clean the path
    dust_scaffold(program, functions)
}

fn main() {
    let program: Vec<i64> = io::stdin().lock().lines().next().unwrap().unwrap()
        .split(',').map(|n| n.parse().unwrap()).collect();

    println!("{}", alignment_parameters(&program));
    println!("{}", sweep_clean(&program));
}
