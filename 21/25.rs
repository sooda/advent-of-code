use std::io::{self, BufRead};

type Floor = Vec<Vec<char>>;

fn move_cucumbers(floor: &Floor) -> Floor {
    let w = floor[0].len();
    let h = floor.len();
    let mut next_floor = vec![vec!['.'; w]; h];
    for (y, row) in floor.iter().enumerate() {
        for (x, ch) in row.iter().enumerate() {
            match ch {
                '.' => (),
                '>' => {
                    if floor[y][(x + 1) % w] == '.' {
                        next_floor[y][(x + 1) % w] = '>';
                    } else {
                        next_floor[y][x] = '>';
                    }
                },
                'v' => (),
                _ => panic!("bad floor")
            }
        }
    }
    for (y, row) in floor.iter().enumerate() {
        for (x, ch) in row.iter().enumerate() {
            match ch {
                '.' => (),
                '>' => (),
                'v' => {
                    if floor[(y + 1) % h][x] != 'v' && next_floor[(y + 1) % h][x] != '>' {
                        next_floor[(y + 1) % h][x]= 'v';
                    } else {
                        next_floor[y][x] = 'v';
                    }
                },
                _ => panic!("bad floor")
            }
        }
    }
    next_floor
}

fn dump(floor: &Floor) {
    for (_y, row) in floor.iter().enumerate() {
        for (_x, ch) in row.iter().enumerate() {
            print!("{}", ch);
        }
        println!();
    }
}

fn movement_duration(mut floor: Floor) -> usize {
    for i in 0.. {
        if false {
            println!("{}:", i);
            dump(&floor);
            println!();
        }
        let next_floor = move_cucumbers(&floor);
        if next_floor == floor {
            return i + 1;
        }
        floor = next_floor;
    }
    unreachable!();
}

fn main() {
    let floor_scan: Vec<Vec<char>> = io::stdin().lock().lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();
    println!("{:?}", movement_duration(floor_scan));
}
