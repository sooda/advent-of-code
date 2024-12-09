#![feature(let_chains)]

use std::io::{self, BufRead};

fn checksum(diskmap: &[usize]) -> usize {
    let mut disk = Vec::new();
    for (id2, &blocks) in diskmap.iter().enumerate() {
        if id2 & 1 == 0 {
            disk.extend(std::iter::repeat(Some(id2 / 2)).take(blocks));
        } else {
            disk.extend(std::iter::repeat(None).take(blocks));
        }
    }
    let mut pos = disk.iter().position(|data| data.is_none()).unwrap();
    loop {
        let last = disk.pop().unwrap();
        disk[pos] = last;

        if let Some(off) = disk[pos..].iter().position(|data| data.is_none()) {
            pos += off;
        } else {
            break;
        }
    }
    disk.iter().enumerate().map(|(i, id)| i * id.unwrap()).sum()
}

#[derive(Copy, Clone, Debug)]
enum Info {
    Data(usize, usize), // id, len
    Empty(usize), // len
}
use Info::*;

impl Info {
    fn data(&self) -> Option<(usize, usize)> {
        match *self {
            Data(id, len) => Some((id, len)),
            _ => None
        }
    }

    fn empty(&self) -> Option<usize> {
        match *self {
            Empty(len) => Some(len),
            _ => None
        }
    }

    fn is_data(&self) -> bool {
        self.data().is_some()
    }
}

fn maybe_join(disk: &mut Vec<Info>, left_idx: usize) {
    if let Some(right_len) = disk[left_idx + 1].empty() {
        disk.remove(left_idx + 1);
        disk[left_idx] = Empty(disk[left_idx].empty().unwrap() + right_len);
    }
}

fn take_data(disk: &mut Vec<Info>, src_idx: usize) -> Info {
    assert!(src_idx > 0); // because the first block is never moved
    let data = disk[src_idx];
    disk[src_idx] = Empty(data.data().unwrap().1);
    // rightmost one doesn't have a pair
    if src_idx < disk.len() - 1 {
        maybe_join(disk, src_idx);
    }
    if !disk[src_idx - 1].is_data() {
        // not really "maybe", now we know both are empty
        maybe_join(disk, src_idx - 1);
    }
    data
}

fn place_data(disk: &mut Vec<Info>, dest_idx: usize, data: Info) {
    let len = data.data().unwrap().1;
    let capacity = disk[dest_idx].empty().unwrap();
    assert!(capacity >= len);
    if capacity > len {
        disk.insert(dest_idx + 1, Empty(capacity - len));
    }
    disk[dest_idx] = data;
}

fn defrag(disk: &mut Vec<Info>, src_idx: usize, dest_idx: usize) {
    assert!(dest_idx < src_idx);
    let data = take_data(disk, src_idx);
    place_data(disk, dest_idx, data);
}

fn defrag_checksum(diskmap: &[usize]) -> usize {
    // build a "compressed" queue of disk content descriptors: each Data is unique in id and no two
    // Empty entries go together, or if there would become two sequentially, they are immediately
    // joined to one. Note that zero-sized empty entries are acceptable.
    let mut disk = diskmap.iter().enumerate().map(|(id2, &blocks)|{
        if id2 & 1 == 0 {
            Data(id2 / 2, blocks)
        } else {
            Empty(blocks)
        }
    }).collect::<Vec<Info>>();

    let max_id = disk.len() / 2; // input is odd in length and ends with data, never space
    // Here, a good example of how indexing is logically very unsafe, proceed with caution. The
    // descriptors are scanned right-to-left and src_idx marks the position of src_id.
    let mut src_idx = disk.len() - 1; // last data is here
    for src_id in (1..=max_id).rev() {
        // a slow naive search would be:
        //let src_idx = disk.iter()
        //    .position(|i| i.data().map(|(id, _)| id == src_id).unwrap_or(false))
        //    .unwrap();
        while disk[src_idx].data().map(|(id, _)| id != src_id).unwrap_or(true) {
            src_idx -= 1;
        }

        if let Some((id, len)) = disk[src_idx].data() {
            assert!(len > 0);
            assert_eq!(id, src_id);
            let fits = |info: &Info| info.empty().map(|e| e >= len).unwrap_or(false);
            // only move to the left, not beyond src_idx
            if let Some(dest_idx) = disk.iter().take(src_idx).position(fits) {
                // src take may decrease size by:
                // [A][X][B] - [A][B]: 1
                // [.][X][B] - [.][B]: 1
                // [A][X][.] - [A][.]: 1
                // [.][X][.] - [.]: 2
                // dest put may increase size by:
                // [A][.][B] - [A][X][B]: 0
                // [A][.][B] - [A][X][.][B]: 1
                // total size change: 0 - 1 = -1 or 0 - 2 = -2 or 1 - 1 = 0 or 1 - 2 = -1
                let a = disk.len();
                defrag(&mut disk, src_idx, dest_idx);
                let b = disk.len();
                // keep src_idx within bounds
                src_idx -= a - b;
            }
        } else {
            panic!("logic error");
        }
    }
    let mut checksum = 0;
    let mut pos = 0;
    for info in disk {
        match info {
            Data(id, len) => {
                let arith_sum = len * (pos + pos+len-1) / 2;
                checksum += arith_sum * id;
                pos += len;
            },
            Empty(len) => {
                pos += len;
            }
        }
    }
    checksum
}

fn main() {
    let diskmap = io::stdin().lock().lines()
        .next().unwrap()
        .unwrap()
        .bytes()
        .map(|b| (b - b'0') as usize)
        .collect::<Vec<_>>();
    println!("{}", checksum(&diskmap));
    println!("{}", defrag_checksum(&diskmap));
}
