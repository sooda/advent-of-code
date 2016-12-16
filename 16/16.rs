use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn checksum(source: &str) -> String {
    source.as_bytes().chunks(2).map(|a| if a[0] == a[1] { '1' } else { '0' }).collect::<String>()
}

fn dragon(input: &str, disk_size: usize) -> String {
    let mut random_data = input.to_owned();
    while random_data.len() < disk_size {
        let flipped = random_data.chars().rev().collect::<String>()
            .replace("0", "Z").replace("1", "0").replace("Z", "1");
        random_data = random_data + "0" + &flipped;
    }

    let to_disk = &random_data[0..disk_size];
    let mut sum = checksum(to_disk);
    while sum.len() % 2 == 0 {
        sum = checksum(&sum);
    }

    sum
}

fn main() {
    // oh dear, so many ways to fail
    let input = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap()).lines().next().unwrap().unwrap();

    println!("{}", dragon(&input, 20));
    println!("{}", dragon(&input, 272));
    println!("{}", dragon(&input, 35651584));
}
