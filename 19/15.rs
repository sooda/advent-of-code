use std::io::{self, BufRead};
use std::collections::HashMap;
use std::collections::VecDeque;

fn step<'a, 'b, I: Iterator<Item = &'b i64>>(program: &'a mut [i64], ip: usize, base: i64, input: &mut I) -> Option<(usize, i64, Option<i64>)> {
    let opcode = program[ip] % 100;
    if opcode == 99 {
        // short circuit this, the discontinuity is annoying
        return None;
    }
    if false {
        let desc = &[
            "????????????????",
            "add",
            "mul",
            "in",
            "out",
            "tnz", // "test not zero"
            "tz",
            "bl", // "branch less"
            "be",
            "base",
        ];

        println!("run ip {} op {} ({})", ip, opcode, desc[opcode as usize]);
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

struct Computer {
    program: Vec<i64>,
    ip: usize,
    base: i64,
}

fn execute(computer: &mut Computer, next_in: i64) -> Option<i64> {
    let inputs = &[next_in];
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

const LOCATION_WALL: i64 = 0;
const LOCATION_OPEN: i64 = 1;
const LOCATION_OXYGEN: i64 = 2;

const DIR_NORTH: i64 = 1;
const DIR_SOUTH: i64 = 2;
const DIR_WEST: i64 = 3;
const DIR_EAST: i64 = 4;

type Grid = HashMap<(i32, i32), i64>;

fn dump(grid: &Grid, droidpos: (i32, i32)) {
    println!("<map>");
    let minx = grid.keys().map(|&(x, _)| x).min().unwrap();
    let maxx = grid.keys().map(|&(x, _)| x).max().unwrap();
    let miny = grid.keys().map(|&(_, y)| y).min().unwrap();
    let maxy = grid.keys().map(|&(_, y)| y).max().unwrap();
    for y in miny..=maxy {
        for x in minx..=maxx {
            let ch = if (x, y) == droidpos {
                'D'
            } else {
                match grid.get(&(x, y)) {
                    Some(&LOCATION_WALL) => '#',
                    Some(&LOCATION_OPEN) => '.',
                    Some(&LOCATION_OXYGEN) => 'O',
                    None => ' ',
                    _ => panic!("map corrupted")
                }
            };
            print!("{}", ch);
        }
        println!();
    }
    println!("</map>");
}

fn explore(computer: &mut Computer, grid: &mut Grid, x: i32, y: i32, current_tile: i64) {
    if grid.contains_key(&(x, y)) {
        // around a corner we've seen already
        return;
    }
    grid.insert((x, y), current_tile);
    if false { // animation
        println!("at {} {}", x, y);
        dump(grid, (x, y));
    }

    if current_tile == LOCATION_WALL {
        // the robot didn't actually go here; just mark it in the map
        return;
    }
    assert!(current_tile == LOCATION_OPEN || current_tile == LOCATION_OXYGEN);

    let attempts = &[
        (x, y - 1, DIR_NORTH, DIR_SOUTH),
        (x, y + 1, DIR_SOUTH, DIR_NORTH),
        (x - 1, y, DIR_WEST,  DIR_EAST),
        (x + 1, y, DIR_EAST,  DIR_WEST),
    ];
    for &(xi, yi, dircmd, backcmd) in attempts {
        let reply = execute(computer, dircmd).unwrap(); // never stop the madness
        explore(computer, grid, xi, yi, reply);
        if reply != LOCATION_WALL {
            let back = execute(computer, backcmd).unwrap();
            assert_eq!(back, current_tile);
        };
    }
}

fn bfs(grid: &Grid, destination: i64) -> usize {
    let mut queue = VecDeque::new();
    let mut distances = HashMap::new();

    queue.push_back(((0, 0), 0));
    distances.insert((0, 0), 0);

    while let Some(current) = queue.pop_front() {
        let ((xi, yi), dist) = current;
        // other ends of the graph "edge" from (xi, yi)
        let steps = &[
            (xi - 1, yi),
            (xi + 1, yi),
            (xi, yi - 1),
            (xi, yi + 1)
        ];
        for nextpos in steps {
            let unknown = !distances.contains_key(nextpos);
            let floor = *grid.get(nextpos).unwrap();
            if floor == destination {
                // break out of the search when found; this must be minimal in bfs with
                // equal-weight edges
                return dist + 1;
            }
            let passable = floor != LOCATION_WALL;
            if unknown && passable {
                queue.push_back((*nextpos, dist + 1));
                distances.insert(*nextpos, dist + 1);
            }
        }
    }

    panic!("oxygen 404");
}

fn oxygen_system_distance(program: &[i64]) -> usize {
    let mut prog = program.to_vec();
    prog.resize(prog.len() + 1000, 0);
    let mut computer = Computer {
        program: prog,
        ip: 0,
        base: 0
    };
    let mut grid = HashMap::new();
    // dfs the map using the robot, don't stop at the oxygen tile yet
    explore(&mut computer, &mut grid, 0, 0, LOCATION_OPEN);
    // then search the minimal distance to the goal using the full map
    bfs(&grid, LOCATION_OXYGEN)
}

fn main() {
    let program: Vec<i64> = io::stdin().lock().lines().next().unwrap().unwrap()
        .split(',').map(|n| n.parse().unwrap()).collect();

    println!("{}", oxygen_system_distance(&program));
}
