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
type Triplets = [Option<u8>; 10]; // 32 fits at most ten three-in-a-rows

fn three_of_same(md5: Hash) -> Triplets {
    let mut tris = [None; 10];
    let ch_at = |i: usize| (md5[i / 2] >> (4 * ((i + 1) & 1))) & 0xf;
    let mut t = 0;

    for i in 0..32-2 {
        //println!("{} {}", i, ch_at(i));
        if ch_at(i) == ch_at(i + 1) && ch_at(i) == ch_at(i + 2) {
            tris[t] = Some(ch_at(i));
            //println!("yes {} {}", i, ch_at(i));
            t += 1;
        }
    }

    tris
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
        println!("h {} {} {:?}", i, md5.result_str(), out);
        space.push(out);
    }

    space[i]
}

fn is_key(idx: usize, salt: &str, space: &mut Hashes) -> bool {
    let hash = get(idx, salt, space);
    let triplets = three_of_same(hash);
    if triplets[0].is_none() {
        return false;
    }
    println!("contains {} {:?}", idx, triplets);
    for next in idx+1..idx+1001 {
        let hash = get(next, salt, space);
        // could triplets.take_while(|x| x.is_some()) to optimize the last ones out immediately
        for tri_ch in triplets.iter().filter_map(|&x| x) {
            if five_of_same(hash, tri_ch) {
                println!("yes! {} {}", next, tri_ch);
                return true;
            }
        }
    }
    false
}

fn key_idx(mut idx: usize, salt: &str, mut space: &mut Hashes) -> usize {
    println!("start at {}", idx);
    while !is_key(idx, salt, &mut space) {
        idx += 1;
    }
    println!("got {}", idx);
    idx
}

fn index_64th(salt: &str) -> usize {
    let mut hashes = Hashes::new();
    let mut idx = 0;
    for _ in 0..65 {
        idx = key_idx(idx, salt, &mut hashes) + 1;
    }
    println!("final {}", idx-1);
    idx - 1
}

fn main() {
    let src = readfile(&std::env::args().nth(1).unwrap());

    //assert!(index_64th("abc") == 22728);
    println!("{}", index_64th(src.trim()));
}




