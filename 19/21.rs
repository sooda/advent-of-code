use std::io::{self, BufRead};

fn step<'a, I: Iterator<Item = i64>>(program: &'a mut [i64], ip: usize, base: i64, input: &mut I) -> Option<(usize, i64, Option<i64>)> {
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
            *mut0(program) = input.next().unwrap();
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

fn execute_springscript(computer: &mut Computer, inputs: &str) -> Option<i64> {
    let mut input = inputs.bytes().map(|b| b as i64);
    let mut damage = None;
    while let Some((newip, newbase, newout)) =
            step(&mut computer.program, computer.ip, computer.base, &mut input) {
        computer.ip = newip;
        computer.base = newbase;
        if let Some(out) = newout {
            if out <= 127 {
                // the animation is epic
                print!("{}", newout.unwrap() as u8 as char);
            } else {
                assert!(damage.is_none());
                damage = Some(out);
            }
        }
    }
    damage
}

/*
rules: jump springs the droid four forward but if the fifth is a hole, then it's game over

         can walk, can jump, certain death
 0 @.... no        no        :(
 1 @...x no        yes
 2 @..x. no        yes
 3 @..xx no        yes
 4 @.x.. no        no        :(
 5 @.x.x no        yes
 6 @.xx. no        no        :(
 7 @.xxx no        yes
 8 @x... yes       no
 9 @x..x yes       yes
10 @x.x. yes       no
11 @x.xx yes       yes
12 @xx.. yes       no
13 @xx.x yes       yes
14 @xxx. yes       no
15 @xxxx yes       yes
    ABCDE

- jump mandatory if !A
- jump disallowed if !D because the droid lands at D and that would be a hole
- droid still falls over if !E but that's not known??
- jump if !A in any case; if !A && !D then the trip is impossible
- certainly E if !C or else D could not be reached and the trip would be impossible
- B doesn't seem to matter

- so, logic: !A | (!C & D)

!A      | (!C       & D)
NOT A J
           NOT C T
                    AND D T
        OR T J
*/
fn research_hull_damage(program: &[i64]) -> i64 {
    let mut prog = program.to_vec();
    prog.resize(prog.len() + prog.len(), 0);
    let mut computer = Computer {
        program: prog,
        ip: 0,
        base: 0
    };
    let script = "NOT A J\n\
                 NOT C T\n\
                 AND D T\n\
                 OR T J\n\
                 WALK\n";
    execute_springscript(&mut computer, script).unwrap_or(0)
}

fn main() {
    let program: Vec<i64> = io::stdin().lock().lines().next().unwrap().unwrap()
        .split(',').map(|n| n.parse().unwrap()).collect();

    println!("{}", research_hull_damage(&program));
}
