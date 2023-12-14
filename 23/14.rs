use std::io::{self, BufRead};

fn slide_north(mut map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let w = map[0].len();
    let h = map.len();
    for x in 0..w {
        let mut free_start = 0;
        loop {
            let free_space = map.iter().skip(free_start).map(|r| r[x]).position(|ch| ch == '.');
            if let Some(ystart_off) = free_space {
                let mut moved = false;
                let ystart = free_start + ystart_off;
                // FIXME this could be more readable in iterator form
                for yi in (ystart+1)..h {
                    let ch = map[yi][x];
                    if ch == '#' {
                        break;
                    } else if ch == 'O' {
                        map[yi][x] = '.';
                        map[ystart][x] = 'O';
                        moved = true;
                        break;
                    }
                }
                if !moved {
                    free_start += 1;
                }
            } else {
                break;
            }
        }
    }
    map
}

fn total_north_load(map: &[Vec<char>]) -> usize {
    let h = map.len();
    map.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter().map(move |&ch| {
                if ch == 'O' { h - y } else { 0 }
            }).sum::<usize>()
        })
        .sum()
}

fn main() {
    let map = io::stdin().lock().lines()
        .map(|row| row.unwrap().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    println!("{}", total_north_load(&slide_north(map.clone())));
}
