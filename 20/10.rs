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

// ways to arrange the rest of adapter chain starting from i. memoize results to ways
fn ways_to_arrange(adapters: &[u32], i: usize, ways: &mut [Option<usize>]) -> usize {
    if let Some(w) = ways[i] {
        return w;
    }
    let these_ways = if i == adapters.len() - 1 {
        1
    } else {
        let mut tot = 0;
        let a = adapters[i];
        for (j, &b) in (&adapters[i + 1..]).iter().take(3).enumerate() {
            if b - a <= 3 {
                tot += ways_to_arrange(adapters, i + 1 + j, ways);
            }
        }
        tot
    };
    ways[i] = Some(these_ways);
    these_ways
}

fn main() {
    let mut adapters: Vec<u32> = io::stdin().lock().lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();
    adapters.sort_unstable();
    let one_three = use_adapters(&adapters);
    println!("{} {} {}", one_three.0, one_three.1, one_three.0 * one_three.1);

    // avoid special case with the arrangement, could have this for the part 1 too though...
    adapters.insert(0, 0);
    // no need to add the device for this, by definition it's reachable only from the last so it
    // would not contribute to path count
    println!("{}", ways_to_arrange(&adapters, 0, &mut vec![None; adapters.len()]));
}
