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

fn _insert_before(links: &mut [(usize, usize)], pos: usize, link: usize) {
    insert_after(links, links[pos].0, link);
}

fn list_nth(links: &[(usize, usize)], mut link: usize, steps: usize) -> usize {
    for _ in 0..steps {
        link = links[link].1;
    }
    link
}

fn move_right(links: &mut [(usize, usize)], src: usize, steps: usize) {
    if steps == 0 {
        // unlink and insert like that not compatible with moving to itself
        return;
    }
    let after = list_nth(links, src, steps);
    unlink(links, src);
    insert_after(links, after, src);
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
        let right = if file[i] >= 0 {
            (file[i] as usize) % (file.len() - 1)
        } else {
            let left = ((-file[i]) as usize) % (file.len() - 1);
            // cyclic, and moving n-1 moves a full cycle etc
            // e.g. len six: 0 1 _2_ 3 4 5 two left = three right: _2_ 0 1 3 4 5
            file.len() - left - 1
        };
        move_right(&mut links, i, right);

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
        pos = list_nth(&links, pos, 1000);
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
