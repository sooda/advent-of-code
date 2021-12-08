use std::io::{self, BufRead};

fn count_1_4_7_8(s: &str) -> usize {
    s.split(' ').filter(|digits| {
        match digits.len() {
            2 | 3 | 4 | 7 => true,
            _ => false
        }
    }).count()
}

fn easy_digits_count(input: &[(String, String)]) -> usize {
    input.iter().map(|(_i, o)| count_1_4_7_8(o)).sum()
}

fn parse_line(line: &str) -> (String, String) {
    let mut sp = line.split(" | ");
    (sp.next().unwrap().to_string(), sp.next().unwrap().to_string())
}

fn main() {
    let input: Vec<(String, String)> = io::stdin().lock().lines()
        .map(|input| parse_line(&input.unwrap()))
        .collect();
    println!("{}", easy_digits_count(&input));
}
