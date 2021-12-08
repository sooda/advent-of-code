use std::io::{self, BufRead};
use std::collections::HashSet;

type Segments = HashSet<char>;

fn count_1_4_7_8(s: &str) -> usize {
    s.split(' ').filter(|digits| {
        match digits.len() {
            2 | 3 | 4 | 7 => true,
            _ => false
        }
    }).count()
}

fn easy_digits_count(display_observation: &[(String, String)]) -> usize {
    display_observation.iter().map(|(_i, o)| count_1_4_7_8(o)).sum()
}

/*
 *  aaaa
 * b    c
 * b    c
 *  dddd
 * e    f
 * e    f
 *  gggg
 */
fn deduce_mapping_and_output(display_observation: &(String, String)) -> u32 {
    let in_values: Vec<Segments> = display_observation.0.split(' ')
        .map(|segs| segs.chars().collect())
        .collect();
    let find_n = |n| in_values.iter().find(|&segs| segs.len() == n).unwrap().clone();
    let filter_n = |n| in_values.iter().filter(move |&segs| segs.len() == n);
    let one = find_n(2);
    let seven = find_n(3);
    let four = find_n(4);
    let eight = find_n(7);
    // neither 2 or 5 have both those segments, so 3 it is
    let three = filter_n(5).find(|&segs| segs.intersection(&one).count() == 2).unwrap().clone();
    // 6 doesn't fill segs of 1, 0 would have two with 3
    let nine = filter_n(6).find(|&segs| segs.intersection(&one).count() == 2 && segs.difference(&three).count() == 1).unwrap().clone();
    // 0 would have 4 without 1
    let six = filter_n(6).find(|&segs| *segs != nine && segs.difference(&one).count() == 5).unwrap().clone();
    // simple elimination
    let zero = filter_n(6).find(|&segs| *segs != nine && *segs != six).unwrap().clone();
    // 6 includes 5 completely
    let five = filter_n(5).find(|&segs| *segs != three && segs.difference(&six).count() == 0).unwrap().clone();
    // simple elimination
    let two = filter_n(5).find(|&segs| *segs != five && *segs != three).unwrap().clone();
    // sanity check because it's early in the morning
    assert!(zero.len() == 6);
    assert!(one.len() == 2);
    assert!(two.len() == 5);
    assert!(three.len() == 5);
    assert!(four.len() == 4);
    assert!(five.len() == 5);
    assert!(six.len() == 6);
    assert!(seven.len() == 3);
    assert!(eight.len() == 7);
    assert!(nine.len() == 6);
    // double checking per the pictures, but this is not necessary
    {
        let aaaa = &seven - &one;
        let dddd = &eight - &zero;
        let gggg = &five - &three;
        let bb = &(&eight - &two) - &three;
        let cc = &eight - &six;
        let ee = &eight - &nine;
        let ff = &(&eight - &two) - &bb;
        assert!(aaaa.len() == 1);
        assert!(dddd.len() == 1);
        assert!(gggg.len() == 1);
        assert!(bb.len() == 1);
        assert!(cc.len() == 1);
        assert!(ee.len() == 1);
        assert!(ff.len() == 1);
    }
    let digits = &[zero, one, two, three, four, five, six, seven, eight, nine];
    // have to be unique
    assert!(digits.iter().map(|d| d.iter().collect::<Vec<_>>()).collect::<HashSet<_>>().len() == 10);

    display_observation.1.split(' ')
        .map(|segs| segs.chars().collect::<Segments>())
        .fold(0, |out, segments| {
            out * 10 + (digits.iter().position(|d| *d == segments).unwrap() as u32)
        }
    )
}

fn output_values(input: &[(String, String)]) -> u32 {
    input.iter().map(|i| deduce_mapping_and_output(i)).sum()
}

fn parse_line(line: &str) -> (String, String) {
    let mut sp = line.split(" | ");
    (sp.next().unwrap().to_string(), sp.next().unwrap().to_string())
}

fn main() {
    let input: Vec<(String, String)> = io::stdin().lock().lines()
        .map(|input| parse_line(&input.unwrap()))
        .collect();
    println!("{}", easy_digits_count(&input));
    println!("{}", output_values(&input));
}
