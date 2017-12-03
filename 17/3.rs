use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

// option 1: bruteforce the grid open until the puzzle input is found. meh.
// and part b is likely for a massive input anyway.
// (or just the explicit path maybe? then it's not important to optimize)
// option 2: figure out a pattern. let's see:
// number of squares that fit in a grid of x by x: well, x*x. x is always odd.
// so, any x is in a box of side size f(x), on the border somewhere
// f(x) = ceil(sqrt(x)), and add +1 if this isn't odd (e.g. sqrt(13) = 3,60...)
// distance from border to middle is then exactly (f(x) - 1) / 2 in one dimension
// but if the puzzle input isn't in a corner, another move is less than this
//
// x*x is always in the bottom right corner
// all i: ((x-1)*x+1)..(x*x) in bottom row,
//        x coordinate (i-1) % x (starting from 0)
// all i: ((x-2)*x+2)..((x-1)*x+1) in left row,
//        y coordinate (i+x-2) % x
// all i: ((x-3)*x+3)..((x-2)*x+2) in top row,
//        x coordinate (i+x-3) % x
// all i: ((x-4)*x+5)..((x-3)*x+3) in right row,
//        y coordinate (i+x-4) % x
//        note that bottom right isn't here, doesn't matter
// note that the corners belong to either one except for right row

fn solve(input: i32) -> i32 {
    if input == 1 {
        return 0;
    }
    let x_ = (input as f64).sqrt();
    let x = x_.ceil() as i32 + if x_.ceil() as i32 % 2 == 0 { 1 } else { 0 };
    let xx = (x - 1) / 2;
    println!("{} {}", x, x_);
    // let right_bottom = x * x;
    let left_bot = (x - 1) * x + 1;
    let left_top = (x - 2) * x + 2;
    let right_top = (x - 3) * x + 3;
    let right_bot_minus_1 = (x - 4) * x + 5;
    println!("{} {} {} {}", left_bot, left_top, right_top, right_bot_minus_1);
    if input >= left_bot {
        println!("a {} {}",  xx , ((input - 1) % x - xx).abs());
        return xx + ((input - 1) % x - xx).abs();
    } else if input >= left_top {
        println!("b {} {}",  xx , ((input - 2) % x - xx).abs());
        return xx + ((input - 2) % x - xx).abs();
    } else if input >= right_top {
        println!("c {} {}",  xx , ((input - 3) % x - xx).abs());
        return xx + ((input - 3) % x - xx).abs();
    } else if input >= right_bot_minus_1 {
        println!("c {} {}",  xx , ((input - 4) % x - xx).abs());
        return xx + ((input - 4) % x - xx).abs();
    } else {
        unreachable!()
    }
}

fn main() {
    assert!(solve(1) == 0);
    assert!(solve(12) == 3);
    assert!(solve(23) == 2);
    assert!(solve(1024) == 31);
    let input = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap())
        .lines().next().unwrap().unwrap().parse::<i32>().unwrap();
    println!("{}", solve(input));
}
