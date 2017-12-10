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

fn solve(lengths: &[usize], size: usize) -> u32 {
    let mut nums = (0..size as u32).collect::<Vec<_>>();
    nums.extend(0..size as u32); // make the initial mirror part
    let mut position: usize = 0;
    let mut skip_size: usize = 0;
    for &length in lengths {
        //println!("!! {:?} p:{} skip:{} len:{}", nums, position, skip_size, length);
        cycled_rev(&mut nums, position, length);
        position += length + skip_size;
        position %= size;
        skip_size += 1;
    }

    nums[0] * nums[1]
}

fn main() {
    assert!(cycled_rev(&mut [0, 1, 2, 3, 4, 0, 1, 2, 3, 4], 3, 4)
            == [4, 3, 2, 1, 0, 4, 3, 2, 1, 0]);
    assert!(cycled_rev(&mut [0, 1, 2, 3, 4, 0, 1, 2, 3, 4], 4, 3)
            == [0, 4, 2, 3, 1, 0, 4, 2, 3, 1]);
    assert!(cycled_rev(&mut [0, 1, 2, 3, 4, 0, 1, 2, 3, 4], 0, 3)
            == [2, 1, 0, 3, 4, 2, 1, 0, 3, 4]);
    assert!(solve(&mut [3, 4, 1, 5], 5) == 12);
    let input = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap())
        .lines().next().unwrap().unwrap().split(",")
        .map(|n| n.parse::<usize>().unwrap()).collect::<Vec<_>>();
    println!("{}", solve(&input, 256));
}
