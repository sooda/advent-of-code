use std::io::{self, BufRead};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum Entry {
    Dir(String, Rc<RefCell<Vec<Entry>>>),
    File(usize),
}

fn simple_compute_dirs_do(entry: &Entry, max: usize, total: &mut usize) -> usize {
    match entry {
        Entry::Dir(_n, contents) => {
            let sz = contents.borrow().iter().map(|c| simple_compute_dirs_do(c, max, total)).sum();
            if sz <= max {
                *total += sz;
            }
            sz
        },
        Entry::File(size) => *size,
    }
}

fn simple_compute_dirs(entry: &Entry, max: usize) -> usize {
    let mut total = 0;
    simple_compute_dirs_do(entry, max, &mut total);
    total
}

fn walk_fs_tree<F: FnMut(&Entry, usize)>(entry: &Entry, visit: &mut F) -> usize {
    let size = match entry {
        Entry::Dir(_, contents) => {
            // contents.borrow().iter().map(&mut |e| walk_fs_tree(e, visit)).sum(),
            let mut s = 0;
            for e in contents.borrow().iter() {
                s += walk_fs_tree(e, visit);
            }
            s
        }
        Entry::File(size) => *size,
    };
    visit(entry, size);
    size
}

fn smallest_to_delete(root: &Entry, capacity: usize, need: usize) -> usize {
    let mut space_used = 0;
    // an iterator over the tree would be better though but it's harder to implement
    walk_fs_tree(root, &mut |entry, _| {
        match entry {
            Entry::File(sz) => space_used += sz,
            _ => (),
        }
    });
    let free_space = capacity - space_used;
    let mut deletion_candidate_sizes = Vec::new();
    walk_fs_tree(root, &mut |entry, size| {
        match entry {
            Entry::Dir(..) => if free_space + size >= need { deletion_candidate_sizes.push(size) },
            _ => (),
        }
    });
    *deletion_candidate_sizes.iter().min().unwrap()
}

fn parse_listing(lines: &[String]) -> Entry {
    let root_dir = Rc::new(RefCell::new(Vec::new()));
    let mut visitstack = Vec::new();
    // like top of visitstack but more comfortable
    let mut current_dir = Rc::downgrade(&root_dir);
    for line in lines {
        if line.starts_with("$ ls") {
            // ignored
        } else if line == "$ cd /" {
            // this is always the first line, and thus useless
            // assert!(current_dir == &root_dir);
        } else if line.starts_with("$ cd ") {
            let dirname: &str = line.rsplit(' ').next().unwrap();
            assert!(dirname != "/");
            if dirname == ".." {
                current_dir = visitstack.pop().unwrap();
            } else {
                // something like the hashmap entry api would be cool here
                let current_dir_up = current_dir.upgrade().unwrap();
                let current_dir_vec = current_dir_up.borrow();
                let direntry = current_dir_vec.iter().find(|d| {
                    match d {
                        Entry::Dir(n, _) => n == dirname,
                        _ => false
                    }
                });
                if let Some(Entry::Dir(_, contents)) = direntry {
                    visitstack.push(current_dir.clone());
                    current_dir = Rc::downgrade(contents);
                } else {
                    // haven't visited here yet
                    panic!("not found {}", dirname);
                }
            }
        } else {
            let mut sp = line.split(' ');
            let spec = sp.next().unwrap();
            let name = sp.next().unwrap();
            if spec == "dir" {
                current_dir.upgrade().unwrap().borrow_mut().push(Entry::Dir(name.to_string(), Rc::new(RefCell::new(Vec::new()))));
            } else {
                current_dir.upgrade().unwrap().borrow_mut().push(Entry::File(spec.parse().unwrap()));
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
