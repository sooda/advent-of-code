use std::io::{self, BufRead};

fn unlink(links: &mut [(usize, usize)], pos: usize) {
    let (left, right) = links[pos];
    links[left].1 = right;
    links[right].0 = left;
}

fn insert_after(links: &mut [(usize, usize)], pos: usize, link: usize) {
    let (left, right) = (pos, links[pos].1);
    links[left].1 = link;
    links[right].0 = link;
    links[link] = (left, right);
}

fn insert_before(links: &mut [(usize, usize)], pos: usize, link: usize) {
    insert_after(links, links[pos].0, link);
}

fn move_right(links: &mut [(usize, usize)], src: usize, steps: usize) {
    if steps == 0 {
        // unlink and insert like that not compatible with moving to itself
        return;
    }
    let mut after = src;
    for _ in 0..steps {
        // move B one right: a B c d -> a c B d
        after = links[after].1;
    }
    unlink(links, src);
    insert_after(links, after, src);
}

fn move_left(links: &mut [(usize, usize)], src: usize, steps: usize) {
    if steps == 0 {
        return;
    }
    let mut before = src;
    for _ in 0..steps {
        // move C one left: a b C d -> a C b d
        // same as moving b right though, but this is more self-documenting
        before = links[before].0;
    }
    unlink(links, src);
    insert_before(links, before, src);
}

fn print_list(file: &[i64], links: &[(usize, usize)]) {
    let mut p = 0;
    // not drawn as in the examples, but it's a loop and thus equivalent
    for _j in 0..file.len() {
        print!("{} ", file[p]);
        p = links[p].1;
    }
    println!();
}

fn basic_links(n: usize) -> Vec<(usize, usize)> {
    let mut links = vec![(0usize, 0usize); n];
    for i in 0..n {
        links[i] = (
            (i + n - 1) % n,
            (i + 1) % n
        );
    }
    links
}

fn mix(file: &[i64], mut links: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    /*
     *           ->
     * 4, 5, 6, 1, 7, 8, 9
     * 4, 5, 6, 7, 1, 8, 9
     *-----             <--
     * 4,-2, 5, 6, 7, 8, 9
     * 4, 5, 6, 7, 8,-2, 9
     *
     * 0  1  2  3  4  5  6
     */

    // links[i] is prev and next pointers for the ith number in the orig list

    if false {
        println!("initial:");
        print_list(file, &links);
    }

    for i in 0..file.len() {
        if file[i] == 0 {
            continue;
        } else if file[i] > 0 {
            move_right(&mut links, i, (file[i] as usize) % (file.len() - 1));
        } else { /* < 0 */
            move_left(&mut links, i, ((-file[i]) as usize) % (file.len() - 1));
        }

        if false {
            print_list(file, &links);
        }
    }

    links
}

fn mix_result(file: &[i64], mixes: usize) -> i64 {
    let mut links = basic_links(file.len());
    for _ in 0..mixes {
        let next_links = mix(file, links);
        links = next_links;
    }

    let mut pos = file.iter().position(|&n| n == 0).unwrap();
    let mut ret = 0;
    for _ in 0..3 {
        for _ in 0..1000 {
            pos = links[pos].1;
        }
        ret += file[pos];
    }

    ret
}

fn mix_result_keyed(file: &mut [i64]) -> i64 {
    file.iter_mut().for_each(|x| *x *= 811589153);
    mix_result(file, 10)
}

fn main() {
    let mut file: Vec<i64> = io::stdin().lock().lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();
    println!("{}", mix_result(&file, 1));
    println!("{}", mix_result_keyed(&mut file));
}
