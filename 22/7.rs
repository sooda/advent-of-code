#![feature(cell_leak)]

use std::io::{self, BufRead};
use std::cell::RefCell;
use std::rc::Rc;

type DirContents = Rc<RefCell<Vec<Entry>>>;

#[derive(Debug)]
enum Entry {
    Dir(String, DirContents),
    File(usize),
}

// a tree structure is "flattened" into these
#[derive(Clone, Copy)]
enum EntryVisit<'a> {
    // marker: a dir entry has been traversed when this pops
    Visiting(usize),
    // an entry to be traversed
    Incoming(&'a Entry),
    // one result from traversing an entry (dir or file) contributing to parent visit
    ExitResult(usize),
}

// just directory entry size iter though, file entries are skipped
struct EntrySizeIter<'a> {
    // O(log n) space, and storage reused for next subtrees
    stack: Vec<EntryVisit<'a>>,
}

impl Entry {
    fn iter_dir_sizes<'a>(&'a self) -> EntrySizeIter<'a> {
        match self {
            Entry::Dir(..) => {
                EntrySizeIter {
                    // no preceding Visiting here to avoid a spurious root^2
                    stack: vec![ EntryVisit::Incoming(self) ],
                }
            },
            _ => {
                // could also return empty iterator for files,
                // but that would be dead code here
                panic!("only dirs can be traversed");
            }
        }
    }
}

impl<'a> Iterator for EntrySizeIter<'a> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.stack.pop() {
                Some(EntryVisit::Visiting(size)) => {
                    // for parent directory, if any
                    self.stack.push(EntryVisit::ExitResult(size));
                    return Some(size);
                },
                Some(EntryVisit::Incoming(&Entry::File(size))) => {
                    self.stack.push(EntryVisit::ExitResult(size));
                },
                Some(res @ EntryVisit::ExitResult(size)) => {
                    let earlier = self.stack.pop();
                    match earlier {
                        Some(EntryVisit::ExitResult(esize)) => {
                            self.stack.push(EntryVisit::ExitResult(size + esize));
                        },
                        // vsize should be 0 though, could return here already?
                        Some(EntryVisit::Visiting(vsize)) => {
                            // returned in next iteration
                            self.stack.push(EntryVisit::Visiting(size + vsize));
                        },
                        Some(incoming @ EntryVisit::Incoming(_)) => {
                            // flip the order to process the next one.
                            // guaranteed to have two exitresults then
                            self.stack.push(res);
                            self.stack.push(incoming);
                        },
                        None => {
                            // result of already visiting root directory
                            return None;
                        }
                    };
                },
                Some(EntryVisit::Incoming(Entry::Dir(_, contents))) => {
                    self.stack.push(EntryVisit::Visiting(0));
                    let evec: std::cell::Ref<'a, _> = contents.borrow();
                    // FIXME don't leak
                    let evec = std::cell::Ref::leak(evec);
                    for ent in evec.iter() {
                        self.stack.push(EntryVisit::Incoming(ent));
                    }
                },
                None => return None // hmmm, should end at exitresult
            }
        }
    }
}

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
    let total_iter = root.iter_dir_sizes()
        .filter(|&size| size <= max)
        .sum::<usize>();

    // backup check with ground truth
    let mut total_recurse = 0;
    walk_fs_sizes(root, &mut |size| {
        if size <= max {
            total_recurse += size;
        }
    });

    assert!(total_iter == total_recurse);
    total_recurse
}

fn smallest_to_delete(root: &Entry, capacity: usize, need: usize) -> usize {
    let space_used_iter = root.iter_dir_sizes().last().unwrap();
    let space_used_recu = walk_fs_sizes(root, &mut |_| { });
    assert!(space_used_iter == space_used_recu);

    let free_space = capacity - space_used_iter;

    let smallest_iter = root.iter_dir_sizes()
        .filter(|&size| free_space + size >= need)
        .min()
        .unwrap();
    let mut smallest_recu = space_used_recu;
    walk_fs_sizes(root, &mut |size| {
        if free_space + size >= need {
            smallest_recu = smallest_recu.min(size);
        }
    });
    assert!(smallest_iter == smallest_recu);

    smallest_iter
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
