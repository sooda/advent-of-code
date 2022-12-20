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
    // So done with off-by-one bugs this morning, so this is as stupid as it gets
    for _ in 0..steps {
        // move B one right: a B c d -> a c B d
        unlink(links, src);
        insert_after(links, links[src].1, src);
    }
}

fn move_left(links: &mut [(usize, usize)], src: usize, steps: usize) {
    for _ in 0..steps {
        // move C one left: a b C d -> a C b d
        // same as moving b right though, but this is more self-documenting
        unlink(links, src);
        insert_before(links, links[src].0, src);
    }
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

fn mix(file: &mut Vec<i64>) -> Vec<(usize, usize)> {
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
    let mut links = vec![(0usize, 0usize); file.len()];
    for i in 0..file.len() {
        links[i] = (
            (i + file.len() - 1) % file.len(),
            (i + 1) % file.len()
        );
    }

    if false {
        println!("initial:");
        print_list(file, &links);
    }

    for i in 0..file.len() {
        if file[i] == 0 {
            continue;
        } else if file[i] > 0 {
            move_right(&mut links, i, file[i] as usize);
        } else { /* < 0 */
            move_left(&mut links, i, (-file[i]) as usize);
        }

        if false {
            print_list(file, &links);
        }
    }

    links
}

fn mix_result(mut file: Vec<i64>) -> i64 {
    let links = mix(&mut file);

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

fn main() {
    let file: Vec<i64> = io::stdin().lock().lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();
    println!("{}", mix_result(file));
}
