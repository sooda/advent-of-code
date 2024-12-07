use std::io::{self, BufRead};

// result and test values
type Equation = (i64, Vec<i64>);

fn try_resolve(result: i64, current: i64, remaining: &[i64]) -> bool {
    if remaining.len() == 0 {
        current == result
    } else {
        try_resolve(result, current + remaining[0], &remaining[1..]) ||
            try_resolve(result, current * remaining[0], &remaining[1..])
    }
}

fn possibly_true(eq: &Equation) -> bool {
    try_resolve(eq.0, eq.1[0], &eq.1[1..])
}

fn total_calibration_result(eqs: &[Equation]) -> i64 {
    eqs.iter().filter(|&e| possibly_true(e)).map(|e| e.0).sum()
}

fn parse(line: &str) -> Equation {
    let mut sp = line.split(": ");
    let res = sp.next().unwrap().parse().unwrap();
    let tv = sp.next().unwrap().split(' ').map(|n| n.parse().unwrap()).collect();
    (res, tv)
}

fn main() {
    let equations: Vec<Equation> = io::stdin().lock().lines()
        .map(|line| parse(&line.unwrap())
            ).collect();
    println!("{}", total_calibration_result(&equations));
}

