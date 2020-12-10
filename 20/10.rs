use std::io::{self, BufRead};

fn use_adapters(adapters: &[u32]) -> (u32, u32) {
    let mut jumps_of_one = 0;
    let mut jumps_of_three = 0;
    // std::iter::once() would be nice but iterators have no windows(). Could also require that the
    // input contains the wall of zero jolts...
    match adapters[0] {
        1 => jumps_of_one += 1,
        2 => {},
        3 => jumps_of_three += 1,
        _ => panic!()
    }
    for ab in adapters.windows(2) {
        match ab[1] - ab[0] {
            1 => jumps_of_one += 1,
            2 => {},
            3 => jumps_of_three += 1,
            _ => panic!()
        }
    }
    // "your device's built-in adapter is always 3 higher than the highest adapter"
    jumps_of_three += 1;
    (jumps_of_one, jumps_of_three)
}

fn main() {
    let mut adapters: Vec<u32> = io::stdin().lock().lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();
    adapters.sort_unstable();
    let adapters = adapters;

    let one_three = use_adapters(&adapters);
    println!("{} {} {}", one_three.0, one_three.1, one_three.0 * one_three.1);
}
