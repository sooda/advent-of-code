use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::slice::Iter;

fn metadata_sum(v: &mut Iter<u32>) -> u32 {
    let children = *v.next().unwrap();
    let metadata_entries = *v.next().unwrap();

    let child_sum = (0..children).map(|_| metadata_sum(v)).sum::<u32>();
    let own_sum = (0..metadata_entries).fold(0, |total, _| total + v.next().unwrap());

    child_sum + own_sum
}

fn main() {
    let file = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap())
        .lines().next().unwrap().unwrap();
    let license = file.split(" ").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>();

    println!("{}", metadata_sum(&mut license.iter()));
}
