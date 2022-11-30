#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use std::io::{self, BufRead};

#[derive(Clone)]
struct Board<const N: usize> where [(); N * N]: {
    numbers: [u32; N * N],
    marked: [bool; N * N],
}

impl<const N: usize> Board<N> where [(); N * N]: {
    fn new(numbers: [u32; N * N]) -> Self {
        Board { numbers: numbers, marked: [false; N * N] }
    }

    fn from_string_spec(board_spec: &[String]) -> Self {
        let mut numbers = [0; N * N];
        let input = board_spec.iter().flat_map(
            |spec| spec.split_whitespace().map(|n| n.parse::<u32>().unwrap())
            );
        numbers.iter_mut().zip(input).for_each(|(a, i)| *a = i);
        Board::new(numbers)
    }

    fn winning_sum(&self) -> u32 {
        self.numbers.iter().zip(self.marked.iter())
            .filter(|&(_n, m)| !m)
            .map(|(n, _m)| n)
            .sum()
    }

    fn row_complete(&self, i: usize) -> bool {
        self.marked.iter().skip(N * i).take(N).all(|&x| x)
    }

    fn col_complete(&self, i: usize) -> bool {
        self.marked.chunks(N).map(|c| c[i]).all(|x| x)
    }

    fn one_complete(&self) -> bool {
        (0..N).any(|i| self.row_complete(i) || self.col_complete(i))
    }

    fn play_round(&mut self, number: u32) -> Option<u32> {
        if let Some(index) = self.numbers.iter().position(|&n| n == number) {
            self.marked[index] = true;
            if self.one_complete() {
                Some(number * self.winning_sum())
            } else {
                None
            }
        } else {
            None
        }
    }

}

type SquidBoard = Board<5>;

fn winning_score(boards: &mut [SquidBoard], drawn_numbers: &[u32]) -> u32 {
    for &num in drawn_numbers {
        for board in boards.iter_mut() {
            if let Some(score) = board.play_round(num) {
                return score;
            }
        }
    }
    panic!("nobody won");
}

fn squid_score(boards: &mut [SquidBoard], drawn_numbers: &[u32]) -> u32 {
    let mut won_boards: Vec<bool> = boards.iter().map(|_b| false).collect();
    for &num in drawn_numbers {
        for (i, board) in boards.iter_mut().enumerate() {
            if let Some(score) = board.play_round(num) {
                won_boards[i] = true;
                if won_boards.iter().all(|&w| w) {
                    return score;
                }
            }
        }
    }
    panic!("nobody won");
}

fn main() {
    let lines: Vec<String> = io::stdin().lock().lines().map(|l| l.unwrap()).collect();
    let drawn_numbers: Vec<u32> = lines[0].split(',')
        .map(|num| num.parse().unwrap()).collect();
    let mut boards: Vec<SquidBoard> = lines[1..].chunks(6)
        .map(SquidBoard::from_string_spec)
        .collect();
    println!("{}", winning_score(&mut boards.clone(), &drawn_numbers));
    println!("{}", squid_score(&mut boards, &drawn_numbers));
}
