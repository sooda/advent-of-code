use std::collections::HashMap;

enum State {
    A, B, C, D, E, F
}
use State::*;

fn sample(state: State, current: bool) -> (bool, i64, State) {
    match state {
        A if !current => (true,   1, B),
        A if  current => (false, -1, B),
        B if !current => (true,  -1, A),
        B if  current => (true,   1, A),
        _ => unreachable!()
    }
}

fn input(state: State, current: bool) -> (bool, i64, State) {
    match state {
        A if !current => (true,   1, B),
        A if  current => (false,  1, C),
        B if !current => (false, -1, A),
        B if  current => (false,  1, D),
        C if !current => (true,   1, D),
        C if  current => (true,   1, A),
        D if !current => (true,  -1, E),
        D if  current => (false, -1, D),
        E if !current => (true,   1, F),
        E if  current => (true,  -1, B),
        F if !current => (true,   1, A),
        F if  current => (true,   1, E),
        _ => unreachable!()
    }
}

fn ones<F>(n: usize, f: F) -> usize
    where F: Fn(State, bool) -> (bool, i64, State) {
    let mut tape = HashMap::new();
    let mut cursor = 0;
    let mut state = A;

    for _ in 0..n {
        let current = tape.entry(cursor).or_insert(false);

        let (value, dir, nextstate) = f(state, *current);

        *current = value;
        cursor += dir;
        state = nextstate;
    }
    tape.iter().filter(|&(_, &v)| v).count()
}

fn main() {
    assert!(ones(6, sample) == 3);
    println!("{}", ones(12399302, sample));
    println!("{}", ones(12399302, input));
}
