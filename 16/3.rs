use std::fs::File;
use std::io::Read;

fn readfile(name: &str) -> String {
    let mut f = File::open(name).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    s
}

fn process(row: &str) -> usize {
    let mut nums: Vec<_> = row.split(" ").filter_map(|x| x.parse::<u32>().ok()).collect();
    nums.sort();

    (nums[0] + nums[1] > nums[2]) as usize

}

fn main1a() {
    let src = readfile(&std::env::args().nth(1).unwrap());
    let n = src.trim().split("\n").map(process).sum::<usize>();
    println!("{}", n);
}

fn good_tri(a: u32, b: u32, c: u32) -> u32 {
    let mut indices = [a, b, c];
    indices.sort();
    (indices[0] + indices[1] > indices[2]) as u32
}

fn main() {
    let src = readfile(&std::env::args().nth(1).unwrap());
    // from 101 301 501 102 302 502 103 303 503
    // to   101 102 103 301 302 303 501 502 503
    let mut numbers = src.trim().split_whitespace().filter_map(|x| x.parse::<u32>().ok());
    let mut n = 0u32;
    'outer: loop {
        let mut tris: [u32; 9] = [0; 9];
        // meh, might be neater with iterators, grouping these somehow
        for i in tris.iter_mut() {
            // exact amount expected in the input, so when None, tris is empty
            *i = match numbers.next() {
                Some(n) => n,
                _ => break 'outer
            };
        }
        n += good_tri(tris[0], tris[3], tris[6]);
        n += good_tri(tris[1], tris[4], tris[7]);
        n += good_tri(tris[2], tris[5], tris[8]);
    }
    println!("{}", n);
}

