use std::fs::File;
use std::io::Read;

// rustc -L foo/deps 5.rs
extern crate crypto;
use crypto::md5::Md5;
use crypto::digest::Digest;

fn readfile(name: &str) -> String {
    let mut f = File::open(name).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    s
}

type Hash = [u8; 16];
type Hashes = Vec<Hash>;
type TripletChar = u8;

fn three_of_same(md5: Hash) -> Option<TripletChar> {
    let ch_at = |i: usize| (md5[i / 2] >> (4 * ((i + 1) & 1))) & 0xf;

    for i in 0..32-2 {
        if ch_at(i) == ch_at(i + 1) && ch_at(i) == ch_at(i + 2) {
            return Some(ch_at(i));
        }
    }

    None
}

fn five_of_same(md5: Hash, of_what: u8) -> bool {
    let ch_at = |i: usize| (md5[i / 2] >> (4 * ((i + 1) & 1))) & 0xf;

    for i in 0..32-4 {
        if (i..i+5).all(|j| ch_at(j) == of_what) {
            return true;
        }
    }
    false
}

fn get(i: usize, salt: &str, space: &mut Hashes) -> Hash {
    // only grows one at a time
    if i == space.len() {
        // ugh wat tostr
        let salted = salt.to_owned() + i.to_string().as_str();
        let mut md5 = Md5::new();
        md5.input(salted.as_bytes());

        let mut out = [0u8; 16];
        md5.result(&mut out);
        space.push(out);
    }

    space[i]
}

fn is_key(idx: usize, salt: &str, space: &mut Hashes) -> bool {
    let hash = get(idx, salt, space);
    let triplet = three_of_same(hash);
    if triplet.is_none() {
        return false;
    }
    let triplet = triplet.unwrap();
    for next in idx+1..idx+1001 {
        let hash = get(next, salt, space);
        if five_of_same(hash, triplet) {
            return true;
        }
    }
    false
}

fn key_idx(mut idx: usize, salt: &str, mut space: &mut Hashes) -> usize {
    while !is_key(idx, salt, &mut space) {
        idx += 1;
    }
    idx
}

fn index_64th(salt: &str) -> usize {
    let mut hashes = Hashes::new();
    let mut idx = 0;
    for _ in 0..64 {
        idx = key_idx(idx, salt, &mut hashes) + 1;
    }
    idx - 1
}

fn main() {
    let src = readfile(&std::env::args().nth(1).unwrap());

    assert!(index_64th("abc") == 22728);
    println!("{}", index_64th(src.trim()));
}




