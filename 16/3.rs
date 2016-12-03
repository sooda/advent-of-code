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

fn main() {
    let src = readfile(&std::env::args().nth(1).unwrap());
    let n = src.trim().split("\n").map(process).sum::<usize>();
    println!("{}", n);
}

