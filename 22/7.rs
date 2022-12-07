use std::io::{self, BufRead};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum Entry {
    Dir(String, Rc<RefCell<Vec<Entry>>>),
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
    let root_dir = Rc::new(RefCell::new(Vec::new()));
    let mut visitstack = Vec::new();
    visitstack.push(root_dir.clone());
    for line in lines {
        if line == "$ ls" {
            // ignored
        } else if line == "$ cd /" {
            // this is always the first line, and thus useless
            assert!(Rc::ptr_eq(&visitstack.last().unwrap(), &root_dir));
        } else if line.starts_with("$ cd ") {
            let dirname: &str = line.rsplit(' ').next().unwrap();
            assert!(dirname != "/");
            if dirname == ".." {
                visitstack.pop().unwrap();
            } else {
                // something like the hashmap entry api would be cool here
                let current_dir = visitstack.last().unwrap().clone();
                let current_dir_vec = current_dir.borrow();
                if let Some(Entry::Dir(_, contents)) = current_dir_vec.iter().find(|d| {
                    match d {
                        Entry::Dir(n, _) => n == dirname,
                        _ => false
                    }
                }) {
                    visitstack.push(contents.clone());
                } else {
                    panic!("not found {}", dirname);
                }
            }
        } else {
            let mut sp = line.split(' ');
            let spec = sp.next().unwrap();
            let name = sp.next().unwrap();
            let mut current_dir = visitstack.last().unwrap().borrow_mut();
            if spec == "dir" {
                current_dir.push(Entry::Dir(name.to_string(), Rc::new(RefCell::new(Vec::new()))));
            } else {
                current_dir.push(Entry::File(spec.parse().unwrap()));
            }
        }
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
