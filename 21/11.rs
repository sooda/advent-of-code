use std::io::{self, BufRead};
use std::collections::HashSet;

fn flash(octos: &mut [Vec<u8>], x: i32, y: i32, flash_set: &mut HashSet::<(i32, i32)>) {
    if flash_set.contains(&(x, y)) {
        return;
    }
    flash_set.insert((x, y));
    for &(nx, ny) in [
        (x - 1, y - 1), (x, y - 1), (x + 1, y - 1),
        (x - 1, y),                 (x + 1, y),
        (x - 1, y + 1), (x, y + 1), (x + 1, y + 1),
    ].iter().filter(|p| p.0 >= 0 && p.0 <= 9 && p.1 >= 0 && p.1 <= 9) {
        octos[ny as usize][nx as usize] += 1;
        if octos[ny as usize][nx as usize] > 9 {
            flash(octos, nx, ny, flash_set);
        }
    }
    octos[y as usize][x as usize] = 0;
}

fn iterate(octos: &mut [Vec<u8>]) -> usize {
    octos.iter_mut().for_each(|row| row.iter_mut().for_each(|oct| *oct += 1));
    let mut flash_set = HashSet::<(i32, i32)>::new();
    for y in 0..10 {
        for x in 0..10 {
            if octos[y as usize][x as usize] > 9 {
                flash(octos, x, y, &mut flash_set);
            }
        }
    }
    for &(x, y) in flash_set.iter() {
        octos[y as usize][x as usize] = 0;
    }
    flash_set.len()
}

fn visualize(octos: &[Vec<u8>]) {
    for row in octos {
        for &octo in row {
            print!("{}", (b'0' + octo) as char);
        }
        println!();
    }
    println!();
}

fn total_flashes(octos: &mut [Vec<u8>], iterations: usize) -> usize {
    println!("Before any steps:");
    visualize(octos);
    (0..iterations).fold(0, |flash_count, step| {
        let n = flash_count + iterate(octos);
        println!("After step {}:", 1 + step);
        visualize(octos);
        n
    })
}

fn main() {
    let mut octos: Vec<Vec<u8>> = io::stdin().lock().lines()
        .map(|line| line.unwrap().bytes().map(|b| b - b'0').collect())
        .collect();
    println!("{}", total_flashes(&mut octos, 100));
}
