use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn exactly_n(id: &str, n: usize) -> bool {
    let good_letter = |&l: &u8| id.bytes().filter(|&c| c == l).count() == n;
    return (b'a'..=b'z').find(good_letter).is_some();
}

fn compare_boxes(a: &str, b: &str) -> Option<String> {
    let mut indexed_mismatches = a.chars().zip(b.chars()).enumerate()
        .filter(|(_, (x, y))| x != y);
    if let Some(first_mismatch) = indexed_mismatches.next() {
        if indexed_mismatches.next().is_some() {
            // must have just one differing letter
            None
        } else {
            // first and the only
            let uncommon_pos = first_mismatch.0;
            let chs = a.chars().enumerate()
                .filter(|&(i, _)| i != uncommon_pos)
                .map(|(_, c)| c).collect::<Vec<_>>();

            Some(chs.into_iter().collect())
        }
    } else {
        // exactly the same
        unreachable!();
    }
}
fn proto_fabric_boxes(ids: &[String]) -> String {
    let pairs = ids.iter().flat_map(|b| ids.iter().map(move |a| (a, b)));
    return pairs.filter(|(a, b)| a != b)
        .find_map(|(a, b)| compare_boxes(&a, &b)).unwrap();
}

fn main() {
    let ids = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap())
        .lines().map(|x| x.unwrap()).collect::<Vec<_>>();

    let twos = ids.iter().filter(|x| exactly_n(x, 2)).count();
    let threes = ids.iter().filter(|x| exactly_n(x, 3)).count();
    println!("{}", twos * threes);
    println!("{}", proto_fabric_boxes(&ids));
}
