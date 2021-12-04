use std::io::{self, BufRead};

#[derive(Clone)]
struct Board {
    numbers: [u32; 5 * 5],
    marked: [bool; 5 * 5],
}

impl Board {
    fn new(numbers: [u32; 5 * 5]) -> Self {
        Board { numbers: numbers, marked: [false; 5 * 5] }
    }

    fn winning_sum(&self) -> u32 {
        self.numbers.iter().zip(self.marked.iter())
            .filter(|&(_n, m)| !m)
            .map(|(n, _m)| n)
            .sum()
    }

    fn row_complete(&self, i: usize) -> bool {
        self.marked.iter().skip(5 * i).take(5).all(|&x| x)
    }

    fn col_complete(&self, i: usize) -> bool {
        self.marked.chunks(5).map(|c| c[i]).all(|x| x)
    }

    fn one_complete(&self) -> bool {
        (0..5).any(|i| self.row_complete(i) || self.col_complete(i))
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

fn winning_score(boards: &mut [Board], drawn_numbers: &[u32]) -> u32 {
    for &num in drawn_numbers {
        for board in boards.iter_mut() {
            if let Some(score) = board.play_round(num) {
                return score;
            }
        }
    }
    panic!("nobody won");
}

fn squid_score(boards: &mut [Board], drawn_numbers: &[u32]) -> u32 {
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

fn parse_board(board_spec: &[String]) -> Board {
    let mut numbers = [0; 5 * 5];
    let input = board_spec.iter().flat_map(
        |spec| spec.split_whitespace().map(|n| n.parse::<u32>().unwrap())
    );
    numbers.iter_mut().zip(input).for_each(|(a, i)| *a = i);
    Board::new(numbers)
}

fn main() {
    let lines: Vec<String> = io::stdin().lock().lines().map(|l| l.unwrap()).collect();
    let drawn_numbers: Vec<u32> = lines[0].split(',')
        .map(|num| num.parse().unwrap()).collect();
    let mut boards: Vec<Board> = lines[1..].chunks(6)
        .map(parse_board)
        .collect();
    println!("{}", winning_score(&mut boards.clone(), &drawn_numbers));
    println!("{}", squid_score(&mut boards, &drawn_numbers));
}
