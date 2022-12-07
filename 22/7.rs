use std::io::{self, BufRead};
use std::cell::RefCell;
use std::rc::Rc;

type DirContents = Rc<RefCell<Vec<Entry>>>;

#[derive(Debug)]
enum Entry {
    Dir(String, DirContents),
    File(usize),
}

// an iterator over the tree would be better though but it's harder to implement
fn walk_fs_sizes<F: FnMut(usize)>(entry: &Entry, visit: &mut F) -> usize {
    match entry {
        Entry::Dir(_, contents) => {
            let size = contents.borrow()
                .iter()
                .map(&mut |e| walk_fs_sizes(e, visit))
                .sum();
            visit(size);
            size
        }
        Entry::File(size) => *size
    }
}

fn simple_compute_dirs(root: &Entry, max: usize) -> usize {
    let mut total = 0;
    walk_fs_sizes(root, &mut |size| {
        if size <= max {
            total += size;
        }
    });
    total
}

fn smallest_to_delete(root: &Entry, capacity: usize, need: usize) -> usize {
    let space_used = walk_fs_sizes(root, &mut |_| { });
    let free_space = capacity - space_used;
    let mut smallest = space_used;
    walk_fs_sizes(root, &mut |size| {
        if free_space + size >= need {
            smallest = smallest.min(size);
        }
    });
    smallest
}

fn parse_listing(lines: &[String]) -> Entry {
    let root_dir: DirContents = Default::default();
    let mut visitstack = Vec::new();
    visitstack.push(root_dir.clone());
    let mut parts = Vec::new(); // save on per-line allocs, use the same mem
    for line in lines {
        parts.clear();
        parts.extend(line.split(' '));
        match &parts[..] {
            &["$", "ls"] => (),
            &["$", "cd", "/"] => {
                // the first line in the input
                assert!(Rc::ptr_eq(&visitstack.last().unwrap(), &root_dir));
            },
            &["$", "cd", ".."] => {
                    visitstack.pop().unwrap();
            },
            &["$", "cd", dirname] => {
                let current_dir: DirContents = visitstack.last().unwrap().clone();
                if let Some(Entry::Dir(_, contents)) = current_dir.borrow().iter().find(|d| {
                    match d {
                        Entry::Dir(n, _) => n == dirname,
                        _ => false
                    }
                }) {
                    visitstack.push(contents.clone());
                } else {
                    panic!("not found {}", dirname);
                };
            },
            &["dir", name] => {
                let mut current_dir = visitstack.last().unwrap().borrow_mut();
                current_dir.push(Entry::Dir(name.to_string(), Default::default()));
            },
            &[size, _name] => {
                let mut current_dir = visitstack.last().unwrap().borrow_mut();
                current_dir.push(Entry::File(size.parse().unwrap()));
            }
            _ => panic!()
        };
    }
    Entry::Dir("/".to_string(), root_dir)
}

fn main() {
    let terminal_listing: Vec<_> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect();
    let fs = parse_listing(&terminal_listing);
    println!("{}", simple_compute_dirs(&fs, 100000));
    println!("{}", smallest_to_delete(&fs, 70000000, 30000000));
}
