use std::io::{self, BufRead};

#[derive(Clone)]
struct Hanoi {
    stacks: Vec<Vec<char>>,
    moves: Vec<(usize, usize, usize)>,
}

fn top_stack_result(mut game: Hanoi) -> String {
    for (count, src, dest) in game.moves {
        for _ in 0..count {
            let x = game.stacks[src].pop().unwrap();
            game.stacks[dest].push(x);
        }
    }
    game.stacks.iter().map(|stack| stack.last().unwrap()).collect()
}

fn top_stack_result_9001(mut game: Hanoi) -> String {
    for (count, src, dest) in game.moves {
        let srclen = game.stacks[src].len();
        // would be nice if rust could be told that dest != src, and:
        // game.stacks[dest].extend(&game.stacks[src][srclen - count..]);
        let (a, b) = game.stacks.split_at_mut(src.max(dest));
        let (svec, dvec) = if src < dest {
            // a is [0, .., src, ..], b is [dest, ..]
            (&mut a[src], &mut b[0])
        } else if src > dest {
            // b is [src, ..], a is [0, .., dest, ..]
            (&mut b[0], &mut a[dest])
        } else {
            panic!("src == dest disallowed")
        };
        dvec.extend(&svec[srclen - count..]);
        // always truncated though so no '?' will happen
        svec.resize(srclen - count, '?');
    }
    game.stacks.iter().map(|stack| stack.last().unwrap()).collect()
}

fn parse_game(input: &[String]) -> Hanoi {
    let mut sp = input.split(|x| x == "");
    let mut stacks = Vec::new();
    let spec_len = "[X] ".len();
    // skip the last row that lists the trivial indices
    for line in sp.next().unwrap().iter().filter(|l| !l.starts_with(" 1 ")) {
        // the lines are padded at the end; this really gets resized only for the first line
        let nstacks = (line.len() + 1) / spec_len;
        stacks.resize(nstacks, Vec::new());
        // "    [N]     [V] [V] [H] [L] [J] [D]"
        for (ch, stack) in line.chars()
            .skip(1).step_by(spec_len)
            .zip(stacks.iter_mut()) {
                if ch != ' ' {
                    stack.push(ch);
                }
        }
    }
    for stack in &mut stacks {
        stack.reverse();
    }
    let moves = sp.next().unwrap().iter().map(|line| {
        // "move 19 from 8 to 6"
        let mut sp = line.split(' ').skip(1).step_by(2);
        let movecount = sp.next().unwrap().parse::<usize>().unwrap();
        let srcstack = sp.next().unwrap().parse::<usize>().unwrap();
        let deststack = sp.next().unwrap().parse::<usize>().unwrap();

        (movecount, srcstack - 1, deststack - 1)
    }).collect();

    Hanoi { stacks, moves }
}

fn main() {
    let input: Vec<_> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect();
    let game = parse_game(&input);
    println!("{}", top_stack_result(game.clone()));
    println!("{}", top_stack_result_9001(game));
}
