use std::io::{self, BufRead};

fn paper(map: &[Vec<char>], pos: (i32, i32), dir: (i32, i32)) -> bool {
    let w = map[0].len() as i32;
    let h = map.len() as i32;
    let pos = (pos.0 + dir.0, pos.1 + dir.1);
    let bad = pos.0 < 0 || pos.0 >= w || pos.1 < 0 || pos.1 >= h;
    !bad && map[pos.1 as usize][pos.0 as usize] == '@'
}

fn paper_rolls(map: &[Vec<char>], pos: (i32, i32)) -> usize {
    let delta = &[
        (-1, -1), (0, -1), (1, -1),
        (-1,  0),          (1,  0),
        (-1,  1), (0,  1), (1,  1),
    ];
    delta.into_iter().filter(|&&d| paper(map, pos, d)).count()
}

fn good_positions(map: &[Vec<char>]) -> impl Iterator<Item=(usize, usize)> + use<'_> {
    let w = map[0].len();
    let h = map.len();
    (0..h)
        .flat_map(move |y| (0..w).map(move |x| (x, y)))
        .filter(|&(x, y)| map[y][x] == '@')
        .filter(|&(x, y)| paper_rolls(map, (x as i32, y as i32)) < 4)
}

fn forklift_accessible(map: &[Vec<char>]) -> usize {
    good_positions(map).count()
}

fn remove_paper(map: &Vec<Vec<char>>) -> (Vec<Vec<char>>, usize) {
    let mut next = map.clone();
    let n = good_positions(map)
        .map(|(x, y)| next[y][x] = '.')
        .count();
    (next, n)
}

fn removable_paper(mut map: Vec<Vec<char>>) -> usize {
    let mut tot = 0;
    if true {
        let remove = |m| {
            let (next, n) = remove_paper(&m);
            if n > 0 { Some((next, n)) } else { None }
        };
        while let Some((next, n)) = remove(map) {
            tot += n;
            map = next;
        }
    } else { // equivalent
        loop {
            let (next, n) = remove_paper(&map);
            if n == 0 {
                break;
            }
            tot += n;
            map = next;
        }
    }
    tot
}

fn main() {
    let map: Vec<Vec<char>> = io::stdin().lock().lines()
        .map(
            |line| line.unwrap().chars().collect()
            ).collect();
    println!("{}", forklift_accessible(&map));
    println!("{}", removable_paper(map));
}
