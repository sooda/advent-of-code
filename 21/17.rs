use std::io::{self, BufRead};

fn iterate(yvel0: i32, y0: i32, y1: i32) -> Option<(i32, i32)> {
    let mut y = 0;
    let mut yvel = yvel0;
    let mut yhigh = 0;
    while y >= y0 {
        y += yvel;
        yvel -= 1;
        yhigh = yhigh.max(y);
        if y >= y0 && y <= y1 {
            return Some((yvel0, yhigh));
        }
    }
    None
}

fn highest_shot(area: (i32, i32, i32, i32)) -> i32 {
    let (_, _, y0, y1) = area;
    // looping from 1 onwards can result in a shot that just passes through the area such that a
    // higher one would hit it better in the center, so loop with a reasonable heuristic limit.
    // Any too high limit always runs over the area; see the comment below.
    let highscore = (1..(2 * -y0)).rev()
        .map(|yvel| iterate(yvel, y0, y1))
        .find(|x| x.is_some())
        .unwrap().unwrap();
    // note: the starting velocity is suspiciously close to y0 so there must be a direct solution;
    // the y curve is a symmetric "parabola" after all
    println!("{:?} {:?}", area, highscore);
    highscore.1
}

fn parse_target_area(line: &str) -> (i32, i32, i32, i32) {
    // "target area: x=20..30, y=-10..-5"
    let mut lsp = line.split(", y=");
    let mut xsp = lsp.next().unwrap().split("x=").nth(1).unwrap().split("..");
    let mut ysp = lsp.next().unwrap().split("..");
    (
        xsp.next().unwrap().parse().unwrap(),
        xsp.next().unwrap().parse().unwrap(),
        ysp.next().unwrap().parse().unwrap(),
        ysp.next().unwrap().parse().unwrap()
    )
}

fn main() {
    let target_area = parse_target_area(&io::stdin().lock().lines()
        .next().unwrap().unwrap());
    println!("{}", highest_shot(target_area));
}
