use std::io::{self, BufRead};

type Lidx = u16;
type Link = (Lidx, Lidx);

fn at(links: &[Link], pos: Lidx) -> Link {
    links[pos as usize]
}

fn at_mut(links: &mut [Link], pos: Lidx) -> &mut Link {
    &mut links[pos as usize]
}

fn unlink(links: &mut [Link], pos: Lidx) {
    let (left, right) = at(links, pos);
    at_mut(links, left).1 = right;
    at_mut(links, right).0 = left;
}

fn insert_after(links: &mut [Link], pos: Lidx, link: Lidx) {
    let (left, right) = (pos, at(links, pos).1);
    at_mut(links, left).1 = link;
    at_mut(links, right).0 = link;
    *at_mut(links, link) = (left, right);
}

fn insert_before(links: &mut [Link], pos: Lidx, link: Lidx) {
    insert_after(links, at(links, pos).0, link);
}

fn list_nth(links: &[Link], mut link: Lidx, steps: Lidx) -> Lidx {
    for _ in 0..steps {
        link = at(links, link).1;
    }
    link
}

fn move_right(links: &mut [Link], src: Lidx, steps: Lidx) {
    let after = list_nth(links, src, steps);
    let before = at(links, after).1; // manage also steps == 0 branchless
    unlink(links, src);
    insert_before(links, before, src);
}

fn print_list(file: &[i64], links: &[Link]) {
    let mut p = 0;
    // not drawn as in the examples, but it's a loop and thus equivalent
    for _j in 0..file.len() {
        print!("{} ", file[p as usize]);
        p = at(links, p).1;
    }
    println!();
}

fn basic_links(n: Lidx) -> Vec<Link> {
    let mut links = vec![(0, 0); n as usize];
    for i in 0..n {
        links[i as usize] = (
            (i + n - 1) % n,
            (i + 1) % n
        );
    }
    links
}

fn mix(file: &[i64], mut links: Vec<Link>) -> Vec<Link> {
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
            (file[i] % (file.len() as i64 - 1)) as Lidx
        } else {
            let left = (((-file[i])) % (file.len() as i64 - 1)) as Lidx;
            // cyclic, and moving n-1 moves a full cycle etc
            // e.g. len six: 0 1 _2_ 3 4 5 two left = three right: _2_ 0 1 3 4 5
            file.len() as Lidx - left - 1
        };
        move_right(&mut links, i as Lidx, right);

        if false {
            print_list(file, &links);
        }
    }

    links
}

fn mix_result(file: &[i64], mixes: Lidx) -> i64 {
    let mut links = basic_links(file.len() as Lidx);
    for _ in 0..mixes {
        let next_links = mix(file, links);
        links = next_links;
    }

    let mut pos = file.iter().position(|&n| n == 0).unwrap() as Lidx;
    let mut ret = 0;
    for _ in 0..3 {
        pos = list_nth(&links, pos, 1000);
        ret += file[pos as usize];
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
