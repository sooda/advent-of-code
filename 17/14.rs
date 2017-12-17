use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

// some copypasta from 10.rs. ugh

fn cycled_rev(buf: &mut [u32], pos: usize, len: usize) -> &[u32] {
    assert!(buf.len() % 2 == 0);
    let data_len = buf.len() / 2;

    // reversing is easy due to the explicit double-buffered cycle
    buf[pos..pos + len].reverse();
    {
        // may not wrap, but copy to mirror anyway
        let unwrap_part_len = std::cmp::min(data_len - pos, len);
        let wrap_part_len = len - unwrap_part_len;
        let (data, mirror) = buf.split_at_mut(data_len);
        // cycle back to front, if any. wrap_part_len can be 0
        data[0..wrap_part_len].copy_from_slice(&mirror[0..wrap_part_len]);
        // update the end part of mirror
        mirror[pos..pos + unwrap_part_len].copy_from_slice(&data[pos..pos + unwrap_part_len]);
    }

    // This is just for making the tests in the asserts easier. The split_at_mut lifetime stays
    // inside the above block.
    buf
}

fn knot(mut nums: &mut [u32], lengths: &[usize], size: usize, rounds: usize) {
    let mut position: usize = 0;
    let mut skip_size: usize = 0;
    for _ in 0..rounds {
        for &length in lengths.iter() {
            cycled_rev(&mut nums, position, length);
            position += length + skip_size;
            position %= size;
            skip_size += 1;
        }
    }
}

// an array of 16 would be nicer here, but this'll do
fn squeeze_hash(data: &[u32]) -> Vec<u32> {
    data.chunks(16).map(|x| x.iter().fold(0, |acc, &x| acc ^ x)).collect()
}

fn squash_bytes(parts: &[u32]) -> u64 {
    assert!(parts.iter().all(|&x| x < 256));
    parts.iter().take(8)
        .fold(0, |acc, &bits|
              (acc << 8) | bits as u64)
}

fn hash_bits(input: &str) -> (u64, u64) {
    let mut lengths = input.bytes().map(|x| x as usize).collect::<Vec<_>>();
    let size = 256;
    lengths.extend(&[17, 31, 73, 47, 23]);

    let mut nums = (0..size as u32).collect::<Vec<_>>();
    nums.extend(0..size as u32);

    knot(&mut nums, &lengths, size, 64);

    let chunks = squeeze_hash(&nums[0..size]);

    (squash_bytes(&chunks[0..8]), squash_bytes(&chunks[8..16]))
}

fn used_squares(key: &str) -> u32 {
    (0..128)
        .map(|row| {
            let id = String::from(key) + "-" + &row.to_string();
            let hash = hash_bits(&id);
            let bits = hash.0.count_ones() + hash.1.count_ones();
            bits
        }).sum()
}

// here a zero x is in the right side, but it doesn't matter
fn bit_get(x: (u64, u64), i: i32) -> bool {
    if i < 64 {
        (x.1 & (1 << i)) != 0
    } else {
        (x.0 & (1 << (i - 64))) != 0
    }
}

fn bit_set(x: &mut (u64, u64), i: i32) {
    if i < 64 {
        x.1 |= 1 << i;
    } else {
        x.0 |= 1 << (i - 64);
    }
}

fn map_get(map: &[(u64, u64)], x: i32, y: i32) -> bool {
    bit_get(map[y as usize], x)
}

fn map_set(map: &mut [(u64, u64)], x: i32, y: i32) {
    bit_set(&mut map[y as usize], x)
}

// start a depth search to mark as visited all squares accessible from here
fn traverse(map: &[(u64, u64)], mut visited: &mut [(u64, u64)], x: i32, y: i32) {
    // done already
    if map_get(visited, x, y) {
        return;
    }

    // not an occupied square, stop search
    if !map_get(map, x, y) {
        return;
    }

    // mark this and children recursively

    map_set(&mut visited, x, y);

    for &(xx, yy) in &[(x, y - 1), (x + 1, y), (x, y + 1), (x - 1, y)] {
        if xx >= 0 && xx <= 127 && yy >= 0 && yy <= 127 {
            traverse(map, visited, xx, yy);
        }
    }
}

fn test_mark_cell(map: &[(u64, u64)], visited: &mut [(u64, u64)], x: i32, y: i32) -> bool {
    if map_get(visited, x, y) || !map_get(map, x, y) {
        false
    } else {
        traverse(map, visited, x, y);
        true
    }
}

fn region_count(key: &str) -> usize {
    let map = (0..128)
        .map(|row| {
            let id = String::from(key) + "-" + &row.to_string();
            hash_bits(&id)
        }).collect::<Vec<_>>();
    let mut visited = [(0u64, 0u64); 128];
    let mut test_pos = |x, y| test_mark_cell(&map, &mut visited, x, y);
    // Generate the coordinates and test them; if true, this was the first time we're visiting
    // and thus a new tree in the forest was found.
    //
    // Beginner Rust note: have to flatten the coordinates first instead of doing stuff like
    // map(|foo| test_pos(...)) inside flat_map's closure because flat_map wants FnMut. The mutable
    // borrow in test_pos fights against that; a closure to return that map iterator with a mut ref
    // would be FnOnce. This way is cleaner anyway, so no problem.
    (0i32..128)
        .flat_map(|y| (0i32..128).map(move |x| (x, y)))
        .map(|(x, y)| test_pos(x, y)) // I could filter() with test_pos already, but this way
        .filter(|&found| found)       // is more clear about the side effects, IMO.
        .count()
}

fn main() {
    assert!(used_squares("flqrgnkx") == 8108);
    assert!(region_count("flqrgnkx") == 1242);

    let input = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap())
        .lines().next().unwrap().unwrap();
    println!("{} {}", used_squares(&input), region_count(&input));
}
