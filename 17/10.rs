use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

// to nicely reverse slices such that the reversed areas wrap, a buffer that has a duplicate mirror
// of the data is used. The wrapping part is copied to the beginning, and then the non-wrapping
// part is balanced to the mirror again.

fn cycled_rev(buf: &mut [u32], pos: usize, len: usize) -> &[u32] {
    assert!(buf.len() % 2 == 0);
    let data_len = buf.len() / 2;
    //println!("{:?} {} p:{} l:{}", buf, data_len, pos, len);

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
            //println!("!! {:?} p:{} skip:{} len:{}", nums, position, skip_size, length);
            cycled_rev(&mut nums, position, length);
            position += length + skip_size;
            position %= size;
            skip_size += 1;
        }
    }
}

fn solve_a(lengths: &[usize], size: usize) -> u32 {
    let mut nums = (0..size as u32).collect::<Vec<_>>();
    nums.extend(0..size as u32); // make the initial mirror part
    knot(&mut nums, lengths, size, 1);

    nums[0] * nums[1]
}

fn squeeze_hash(data: &[u32]) -> String {
    let xorred = data.chunks(16).map(|x| x.iter().fold(0, |acc, &x| acc ^ x));
    let formatted = xorred.map(|x| format!("{:02x}", x));
    // bleh, how do i concat these nicely
    let mut ret = String::from("");
    ret.extend(formatted);

    ret
}

fn dense_hash(input: &str) -> String {
    let mut lengths = input.bytes().map(|x| x as usize).collect::<Vec<_>>();
    let size = 256;
    lengths.extend(&[17, 31, 73, 47, 23]);

    let mut nums = (0..size as u32).collect::<Vec<_>>();
    nums.extend(0..size as u32);

    knot(&mut nums, &lengths, size, 64);

    squeeze_hash(&nums[0..size])
}

fn main() {
    assert!(cycled_rev(&mut [0, 1, 2, 3, 4, 0, 1, 2, 3, 4], 3, 4)
            == [4, 3, 2, 1, 0, 4, 3, 2, 1, 0]);
    assert!(cycled_rev(&mut [0, 1, 2, 3, 4, 0, 1, 2, 3, 4], 4, 3)
            == [0, 4, 2, 3, 1, 0, 4, 2, 3, 1]);
    assert!(cycled_rev(&mut [0, 1, 2, 3, 4, 0, 1, 2, 3, 4], 0, 3)
            == [2, 1, 0, 3, 4, 2, 1, 0, 3, 4]);
    assert!(solve_a(&mut [3, 4, 1, 5], 5) == 12);

    let input_line = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap())
        .lines().next().unwrap().unwrap();

    let lengths_part_a = input_line.clone().split(",")
        .map(|n| n.parse::<usize>().unwrap()).collect::<Vec<_>>();
    println!("{}", solve_a(&lengths_part_a, 256));

    assert!(dense_hash("") == "a2582a3a0e66e6e86e3812dcb672a272");
    assert!(dense_hash("AoC 2017") == "33efeb34ea91902bb2f59c9920caa6cd");
    assert!(dense_hash("1,2,3") == "3efbe78a8d82f29979031a4aa0b16a9d");
    assert!(dense_hash("1,2,4") == "63960835bcdc130f0b66d7ff4f6a5a8e");

    println!("{}", dense_hash(&input_line));
}
