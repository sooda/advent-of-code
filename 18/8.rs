#![recursion_limit="10"]
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

// ignore contents, traverse iterator
fn skip_node_recursive(v: &mut Iter<u32>) {
    let children = *v.next().unwrap();
    let metadata_entries = *v.next().unwrap();

    for _ in 0..children {
        skip_node_recursive(v);
    }
    for _ in 0..metadata_entries {
        v.next();
    }
}

// v is skipped to the start of next node
fn node_value(v: &mut Iter<u32>) -> u32 {
    let children = *v.next().unwrap();
    let metadata_entries = *v.next().unwrap();

    if children == 0 {
        // just sum of metadata
        (0..metadata_entries).fold(0, |total, _| total + v.next().unwrap())
    } else {
        // sum of child node values indexed by metadata entries of this node

        // Hard to index children, easy to index metadata entries. Skip another iterator to the
        // start of the metadata block.
        let mut metadata = v.clone();
        for _ in 0..children {
            skip_node_recursive(&mut metadata);
        }

        let metas = metadata.take(metadata_entries as usize).collect::<Vec<_>>();

        let mut sum = 0;
        for i in 0..children {
            let val = node_value(v);
            let n = metas.iter().filter(|&&&v| v == 1 + i).count() as u32;
            sum += n * val;
        }

        // Now v is at the metadata front, skip to back so the parent stays in sync.
        for _ in 0..metadata_entries {
            v.next();
        }

        sum
    }
}

fn main() {
    let file = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap())
        .lines().next().unwrap().unwrap();
    let license = file.split(" ").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>();

    println!("{}", metadata_sum(&mut license.iter()));
    println!("{}", node_value(&mut license.iter()));
}

