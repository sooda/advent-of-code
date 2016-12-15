use std::fs::File;
use std::io::Read;

// rustc -L foo/deps 14.rs
extern crate crypto;
use crypto::md5::Md5;
use crypto::digest::Digest;

fn readfile(name: &str) -> String {
    let mut f = File::open(name).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    s
}

type HashResult = [u8; 16];

trait HashSource {
    fn hash(&self, input: &str) -> HashResult;
}

struct HashSourceMd5 {
}

struct HashSourceMd52016 {
}

impl HashSource for HashSourceMd5 {
    fn hash(&self, input: &str) -> HashResult {
        let mut md5 = Md5::new();
        md5.input(input.as_bytes());

        let mut out = [0u8; 16];
        md5.result(&mut out);
        out
    }
}

impl HashSource for HashSourceMd52016 {
    fn hash(&self, input: &str) -> HashResult {
        let mut input = input.to_owned();
        let mut md5 = Md5::new();

        for _ in 0..2017 {
            md5 = Md5::new();
            md5.input(input.as_bytes());
            input = md5.result_str();
        }

        let mut out = [0u8; 16];
        md5.result(&mut out);
        out
    }
}

type TripletChar = u8;

fn three_of_same(md5: HashResult) -> Option<TripletChar> {
    let ch_at = |i: usize| (md5[i / 2] >> (4 * ((i + 1) & 1))) & 0xf;

    for i in 0..32-2 {
        if ch_at(i) == ch_at(i + 1) && ch_at(i) == ch_at(i + 2) {
            return Some(ch_at(i));
        }
    }

    None
}

fn five_of_same(md5: HashResult, of_what: u8) -> bool {
    let ch_at = |i: usize| (md5[i / 2] >> (4 * ((i + 1) & 1))) & 0xf;

    for i in 0..32-4 {
        if (i..i+5).all(|j| ch_at(j) == of_what) {
            return true;
        }
    }
    false
}

struct KeyStore<T: HashSource> {
    salt: String,
    hash: T,
    memory: Vec<HashResult>
}

impl<T: HashSource> KeyStore<T> {
    fn new(salt: &str, hash: T) -> Self {
        KeyStore::<T> { salt: salt.to_owned(), hash: hash, memory: vec![] }
    }

    fn get(&mut self, i: usize) -> HashResult {
        // only grows one at a time
        if i == self.memory.len() {
            // ugh wat tostr
            let salted = self.salt.clone() + &i.to_string();
            let out = self.hash.hash(&salted);
            self.memory.push(out);
        }

        self.memory[i]
    }

    fn is_key(&mut self, idx: usize) -> bool {
        let hash = self.get(idx);
        let triplet = three_of_same(hash);
        if let Some(triplet) = triplet {
            for next in idx+1..idx+1001 {
                let hash = self.get(next);
                if five_of_same(hash, triplet) {
                    return true;
                }
            }
        }
        false
    }

    fn next_key(&mut self, mut idx: usize) -> usize {
        while !self.is_key(idx) {
            idx += 1;
        }
        idx
    }
}

fn index_64th<T: HashSource>(salt: &str, source: T) -> usize {
    let mut store = KeyStore::new(salt, source);
    let mut idx = 0;
    for _ in 0..64 {
        idx = store.next_key(idx) + 1;
    }
    idx - 1
}

fn main() {
    let src = readfile(&std::env::args().nth(1).unwrap());

    assert!(index_64th("abc", HashSourceMd5 {}) == 22728);
    println!("{}", index_64th(src.trim(), HashSourceMd5 {}));

    assert!(index_64th("abc", HashSourceMd52016 {}) == 22551);
    println!("{}", index_64th(src.trim(), HashSourceMd52016 {}));
}




