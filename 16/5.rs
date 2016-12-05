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

fn password(door_id: &str) -> String {
    let mut password = String::new();
    let mut i = 0;

    while password.len() != 8 {
        let mut x = Md5::new();
        // ugh wat tostr
        let salted = door_id.to_owned() + i.to_string().as_str();
        x.input(salted.as_bytes());

        let mut out = [0u8; 16];
        x.result(&mut out);

        if out[0] == 0 && out[1] == 0 && (out[2] & 0xf0) == 0 {
            let res_str = x.result_str();
            let ch = res_str.chars().nth(5).unwrap();
            println!("{:?} {} {}", res_str, i, ch);
            password.push(ch);
        }

        i += 1;
    }

    password
}

fn main() {
    assert!(password("abc") == "18f47a30");
    let src = readfile(&std::env::args().nth(1).unwrap());
    println!("{}", password(src.trim()));
}



