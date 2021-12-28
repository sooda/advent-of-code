use std::io::{self, BufRead};

#[derive(Debug)]
enum Opcode {
    Inp,
    Add,
    Mul,
    Div,
    Mod,
    Eql,
}
use Opcode::*;

// variable index: wxyz = 0123
type Var = usize;

#[derive(Debug, PartialEq, Eq)]
enum Arg {
    Var(Var),
    Num(i64),
}

const VAR_W: Var = 0;
const VAR_X: Var = 1;
const VAR_Y: Var = 2;
const VAR_Z: Var = 3;

const AVAR_W: Arg = Arg::Var(VAR_W);
const AVAR_X: Arg = Arg::Var(VAR_X);
const AVAR_Y: Arg = Arg::Var(VAR_Y);
const AVAR_Z: Arg = Arg::Var(VAR_Z);

#[derive(Debug)]
struct Instruction {
    opcode: Opcode,
    a: Var,
    b: Arg,
}

#[derive(Debug, PartialEq)]
struct Machine {
    vars: [i64; 4],
    input: Vec<i64>,
}

fn step_instruction(machine: &mut Machine, inst: &Instruction) {
    let aval = machine.vars[inst.a];
    let bval = match inst.b {
        Arg::Var(v) => machine.vars[v],
        Arg::Num(n) => n
    };
    let dest = &mut machine.vars[inst.a];
    *dest = match inst.opcode {
        Inp => machine.input.pop().expect("out of input"),
        Add => aval + bval,
        Mul => aval * bval,
        Div => aval / bval,
        Mod => aval % bval,
        Eql => (aval == bval) as i64,
    };
}

fn execute(mach: &mut Machine, program: &[Instruction]) {
    for inst in program {
        step_instruction(mach, inst);
    }
}

fn reg_z(program: &[Instruction], input: Vec<i64>) -> i64 {
    let mut mach = Machine { vars: [0; 4], input };
    execute(&mut mach, program);

    mach.vars[3]
}

const MAX_DIGIT: i64 = 9;
const MIN_DIGIT: i64 = 1;

struct MonadLogic {
    // from digit to digit and offset, e.g. digit9 == digit0 + 2
    // offset is never negative, so (da, db, o) means da == db + o, o >= 0
    rules: [(usize, usize, i64); 7],
}

impl MonadLogic {
    fn from_slice(rules: &[(usize, usize, i64)]) -> MonadLogic {
        assert_eq!(rules.len(), 7);
        let mut r = [(0, 0, 0); 7];
        r.copy_from_slice(&rules);
        // positive offset
        assert!(r.iter().all(|r| r.2 >= 0));
        // all digits present
        assert_eq!(r.iter().map(|r| r.0 + r.1).sum::<usize>(), (0..14).sum());
        MonadLogic { rules: r }
    }
    fn decode_max(&self) -> Vec<i64> {
        let mut digits = vec![0; 14];
        for (da, db, offset) in self.rules {
            digits[da] = MAX_DIGIT;
            digits[db] = MAX_DIGIT - offset;
        }
        digits
    }

    fn decode_min(&self) -> Vec<i64> {
        let mut digits = vec![0; 14];
        for (da, db, offset) in self.rules {
            digits[da] = MIN_DIGIT + offset;
            digits[db] = MIN_DIGIT;
        }
        digits
    }
}

fn validate_monad_number(program: &[Instruction], mut digits: Vec<i64>) -> i64 {
    let digits_as_number = digits.iter().fold(0, |acc, x| acc * 10 + x);
    digits.reverse();
    assert_eq!(reg_z(program, digits), 0);
    digits_as_number
}

fn largest_accepted_monad_number(program: &[Instruction], magic: &MonadLogic) -> i64 {
    validate_monad_number(program, magic.decode_max())
}

fn smallest_accepted_monad_number(program: &[Instruction], magic: &MonadLogic) -> i64 {
    validate_monad_number(program, magic.decode_min())
}

fn analyze_monad_program(program: &[Instruction]) -> MonadLogic {
    let disasm = false;
    if disasm {
        println!("fn execute_native(mach: &mut Machine) {{");
        println!("let mut z = 0;");
        println!();
    }

    let mut stack = Vec::new();
    let mut rules = Vec::new();

    for (i, chunk) in program.chunks(18).enumerate() {
        match chunk {
            &[
                Instruction { opcode: Inp, a: VAR_W, b: Arg::Num(0) },
                Instruction { opcode: Mul, a: VAR_X, b: Arg::Num(0) },
                Instruction { opcode: Add, a: VAR_X, b: AVAR_Z },
                Instruction { opcode: Mod, a: VAR_X, b: Arg::Num(26) },
                Instruction { opcode: Div, a: VAR_Z, b: Arg::Num(division) },
                Instruction { opcode: Add, a: VAR_X, b: Arg::Num(compare) },
                Instruction { opcode: Eql, a: VAR_X, b: AVAR_W },
                Instruction { opcode: Eql, a: VAR_X, b: Arg::Num(0) },
                Instruction { opcode: Mul, a: VAR_Y, b: Arg::Num(0) },
                Instruction { opcode: Add, a: VAR_Y, b: Arg::Num(25) },
                Instruction { opcode: Mul, a: VAR_Y, b: AVAR_X },
                Instruction { opcode: Add, a: VAR_Y, b: Arg::Num(1) },
                Instruction { opcode: Mul, a: VAR_Z, b: AVAR_Y },
                Instruction { opcode: Mul, a: VAR_Y, b: Arg::Num(0) },
                Instruction { opcode: Add, a: VAR_Y, b: AVAR_W },
                Instruction { opcode: Add, a: VAR_Y, b: Arg::Num(offset) },
                Instruction { opcode: Mul, a: VAR_Y, b: AVAR_X },
                Instruction { opcode: Add, a: VAR_Z, b: AVAR_Y },
            ] => {
                if division == 1 {
                    /*
                     * this looks like:
                     * if peek26() + 11 != digit0 {
                     *     put26(14 + digit0);
                     * }
                     *
                     * Only put, no get. The condition is not suitable for 1..9 digits so branch is
                     * always taken in accepted direction
                    */
                    stack.push((i, offset));
                    // not possible to take the branch because this holds
                    assert!(compare >= 9);
                } else if division == 26 {
                    /*
                     * this looks like:
                     * if get26() + -3 != digit4 {
                     *     put26(14 + digit4);
                     * }
                     * These must always pop one number so the comparisons must be made to hold, or
                     * else the stack (z) would hold at least one nonzero number at the end, making
                     * the model number invalid.
                     */
                    let (j, joffset) = stack.pop().expect("pop always has space in this");
                    let total_offset = joffset + compare;
                    if disasm {
                        println!("// digit{} + {} + {} == digit{} == digit{} + {}",
                                 j, joffset, compare, i,
                                 j, total_offset);
                    }
                    let (smaller, bigger) = if total_offset >= 0 { (i, j) } else { (j, i) };
                    rules.push((smaller, bigger, total_offset.abs()));
                } else {
                    panic!("what div");
                }
                if disasm {
                    println!("let digit = mach.input.pop().unwrap();");
                    match division {
                        1 => {
                            println!("if (z % 26) + {compare} != digit {{", compare=compare);
                            println!("    z *= 26;");
                            println!("    z += {offset} + digit;", offset=offset);
                            println!("}}");
                        },
                        _ => {
                            println!("if (z % 26) + {compare} != digit {{", compare=compare);
                            println!("    z /= {division};", division=division);
                            println!("    z *= 26;");
                            println!("    z += {offset} + digit;", offset=offset);
                            println!("}} else {{");
                            println!("    z /= {division};", division=division);
                            println!("}}");
                        },
                    };
                    println!();
                }
            }
            // this is also good for printing the template for the above match before it existed
            other => panic!("non-conformant chunk: {:#?}", other)
        }
    }

    if disasm {
        println!("mach.vars[3] = z;");
        println!("}}");
    }

    MonadLogic::from_slice(&rules)
}

fn parse_instruction(input: &str) -> Instruction {
    let mut sp = input.split(' ');
    let opcode = match sp.next().unwrap() {
        "inp" => Inp,
        "add" => Add,
        "mul" => Mul,
        "div" => Div,
        "mod" => Mod,
        "eql" => Eql,
        _ => panic!("bad opcode"),
    };
    let a = match sp.next().unwrap() {
        "w" => 0,
        "x" => 1,
        "y" => 2,
        "z" => 3,
        _ => panic!("bad a"),
    };
    // note: placeholder "0" for inp that does not have a second parameter
    let b = match sp.next().unwrap_or("0") {
        "w" => Arg::Var(0),
        "x" => Arg::Var(1),
        "y" => Arg::Var(2),
        "z" => Arg::Var(3),
        n => Arg::Num(n.parse().unwrap())
    };
    Instruction { opcode, a, b }
}

fn main() {
    let program: Vec<_> = io::stdin().lock().lines()
        .map(|line| parse_instruction(&line.unwrap()))
        .collect();
    let magic = analyze_monad_program(&program);
    println!("{}", largest_accepted_monad_number(&program, &magic));
    println!("{}", smallest_accepted_monad_number(&program, &magic));
}
