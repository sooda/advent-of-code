use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn solve(input: &str, offset: usize) -> u32 {
    let a = input.chars();
    let b = input.chars().cycle().skip(offset);
    a.zip(b).map(
        |(i, j)| if i == j { i as u32 - '0' as u32 } else { 0 }).sum()
}

fn solve_a(input: &str) -> u32 {
    solve(input, 1)
}

fn solve_b(input: &str) -> u32 {
    solve(input, input.len() / 2)
}

fn main() {
    assert!(solve_a("1122") == 3);
    assert!(solve_a("1111") == 4);
    assert!(solve_a("1234") == 0);
    assert!(solve_a("91212129") == 9);
    let input = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap())
        .lines().next().unwrap().unwrap();
    println!("{}", solve_a(&input));

    assert!(solve_b("1212") == 6);
    assert!(solve_b("1221") == 0);
    assert!(solve_b("123425") == 4);
    assert!(solve_b("123123") == 12);
    assert!(solve_b("12131415") == 4);
    println!("{}", solve_b(&input));
}
