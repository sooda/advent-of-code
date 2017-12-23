use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

// Can't index a vector of registers with char, and indexing a mut hashmap is annoying, so the a-h
// range is modified to just numbers.
type Reg = usize;
type Val = i64;

// some instructions can take either a register or an immediate value
#[derive(Debug)]
enum Arg {
    RegisterArg(Reg),
    ValueArg(Val)
}
use Arg::*;

#[derive(Debug)]
enum Instruction {
    Set(Reg, Arg),
    Sub(Reg, Arg),
    Mul(Reg, Arg),
    Jnz(Arg, Arg)
}
use Instruction::*;

fn parse_line(line: &str) -> Instruction {
    let words = line.split(" ").collect::<Vec<_>>();

    let read_reg = |idx: usize| (words[idx].as_bytes()[0] - b'a') as Reg;
    let read_arg = |idx: usize| {
        if let Ok(number) = words[idx].parse::<Val>() {
            ValueArg(number)
        } else {
            RegisterArg(read_reg(idx))
        }
    };

    match words[0] {
        "set" => Set(read_reg(1), read_arg(2)),
        "sub" => Sub(read_reg(1), read_arg(2)),
        "mul" => Mul(read_reg(1), read_arg(2)),
        "jnz" => Jnz(read_arg(1), read_arg(2)),
        _ => unreachable!()
    }
}

struct Coprocessor {
    pc: usize,
    regs: Vec<Val>,
    mul_invocations: usize
}

fn execute_debugmode(machine: &mut Coprocessor, program: &[Instruction]) {
    match program[machine.pc] {
        Set(dst, RegisterArg(src)) =>
            machine.regs[dst] = machine.regs[src],
        Set(dst, ValueArg(val)) =>
            machine.regs[dst] = val,
        Sub(dst, RegisterArg(src)) =>
            machine.regs[dst] -= machine.regs[src],
        Sub(dst, ValueArg(val)) =>
            machine.regs[dst] -= val,
        Mul(dst, RegisterArg(src)) => {
            machine.regs[dst] *= machine.regs[src];
            machine.mul_invocations += 1;
        },
        Mul(dst, ValueArg(val)) => {
            machine.regs[dst] *= val;
            machine.mul_invocations += 1;
        },
        Jnz(ref cmp, ref off) => {
            let cmp = match *cmp {
                RegisterArg(reg) => machine.regs[reg],
                ValueArg(arg) => arg
            };
            let off = match *off {
                RegisterArg(reg) => machine.regs[reg],
                ValueArg(arg) => arg
            };
            if cmp != 0 {
                machine.pc = (machine.pc as Val + off) as usize;
                return;
            }
        }
    }

    machine.pc += 1;
}

fn mul_count(program: &[Instruction]) -> usize {
    // char iteration is kludgy unfortunately
    let regs = vec![0; (b'h' - b'a' + 1) as usize];
    let mut machine = Coprocessor { pc: 0, regs: regs, mul_invocations: 0 };
    while machine.pc < program.len() {
        execute_debugmode(&mut machine, program);
    }

    machine.mul_invocations
}

// lol, crazy slow
fn reg_h_final(program: &[Instruction]) -> Val {
    // char iteration is kludgy unfortunately
    let mut regs = vec![0; (b'h' - b'a' + 1) as usize];
    regs[0] = 1;
    let mut machine = Coprocessor { pc: 0, regs: regs, mul_invocations: 0 };
    while machine.pc < program.len() {
        execute_debugmode(&mut machine, program);
    }

    machine.regs[7]
}

// still crazy slow, like four seconds per major iteration
fn problem_translated() -> Val {
    let (mut b, mut c, mut d, mut e, mut f, mut g, mut h) = (0i64, 0i64, 0i64, 0i64, 0i64, 0i64, 0i64);
    let (mut b0, mut c0, mut d0, mut e0, mut f0, mut g0, mut h0) = (0i64, 0i64, 0i64, 0i64, 0i64, 0i64, 0i64);
    b = 79;
    c = b; // 79
    b *= 100; // b = 7900
    b -= -100000; // b = 107900
    c = b; // c = 107900
    c -= -17000; // c = 124900
    loop {
        f = 1;
        d = 2;
        loop { // d goes from 2 to b
            e = 2;
            loop { // e goes from 2 to b
                g = d;
                g *= e;
                g -= b;
                if g == 0 { // d * e == b?
                    // hey, does this test if b is a prime or not? h increases by one if found some
                    // factors, otherwise it stays as-is, so it's the number of non-primes in this
                    // range
                    f = 0;
                }
                e -= -1;
                g = e;
                g -= b;
                if g == 0 { // e == b?
                    break;
                }
            }
            d -= -1;
            g = d;
            g -= b;
            if g == 0 { // d == b?
                break;
            }
        }
        if f == 0 { // d * e == b for some d, e in [2, b]
            h -= -1;
        }
        g = b;
        g -= c;

        // own additions for analysis:
        {
            println!("{} {} {} {} {} {} {}", b, c, d, e, f, g, h);
            println!("  {} {} {} {} {} {} {}", b - b0, c - c0, d - d0, e - e0, f - f0, g - g0, h - h0);
            b0 = b;
            c0 = c;
            d0 = d;
            e0 = e;
            f0 = f;
            g0 = g;
            h0 = h;
        }

        if g == 0 { // b == c?
            return h;
        }
        b -= -17;
    }
}

fn nonprimes_in_range() -> Val {
    let (mut b, mut c, mut h) = (0i64, 0i64, 0i64);

    b = 79;
    b *= 100; // b = 7900
    b -= -100000; // b = 107900
    c = b; // c = 107900
    c -= -17000; // c = 124900
    // max*max >= 124900 and this is ~354x speedup
    let max = 354i64;

    loop { // b goes from 107900 to 124900 in steps of 17, 1001 iterations
        let mut factors_found = false;

        'find_factors: for d in 2..max {
            for e in 2..b {
                if d * e == b {
                    factors_found = true;
                    break 'find_factors;
                }
            }
        }

        if factors_found {
            h += 1;
        }

        if b == c {
            return h;
        }

        b += 17;
    }
}

fn main() {
    let program = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap())
        .lines().map(|x| parse_line(&x.unwrap())).collect::<Vec<_>>();
    println!("{}", mul_count(&program));
    //println!("{}", reg_h_final(&program));
    //println!("{}", problem_translated());
    println!("{}", nonprimes_in_range());
}
