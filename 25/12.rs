use std::io::{self, Read};

type Pos = (i32, i32);
type Present = Vec<Pos>;
#[derive(Debug)]
struct Tree {
    size: (i32, i32),
    quantities: Vec<usize>,
}

fn fit(presents: &[Present], tree: &Tree) -> bool {
    let cell_count = presents.iter()
        .zip(tree.quantities.iter())
        .map(|(p, n)| n * p.len())
        .sum::<usize>() as i32;
    let opportunistic = cell_count <= tree.size.0 * tree.size.1;
    let box_count = tree.quantities.iter().sum::<usize>() as i32;
    let conservative = box_count <= (tree.size.0 / 3) * (tree.size.1 / 3);
    assert_eq!(opportunistic, conservative);
    opportunistic
}

fn regions_fit(presents: &[Present], trees: &[Tree]) -> usize {
    trees.iter().filter(|&t| fit(presents, t)).count()
}

/*
 * 5:
 * #.#
 * ###
 * ##.
 *
 * 44x35: 29 25 21 25 28 26
*/
fn parse(file: &str) -> (Vec<Present>, Vec<Tree>) {
    let is_present = |t: &str| t.split_once('\n').unwrap().0.ends_with(':');
    let present_txt = file.split("\n\n").take_while(|&t| is_present(t));
    let mut tree_txt = file.split("\n\n").skip_while(|&t| is_present(t));
    let presents = present_txt.map(|t| {
        t.lines().skip(1).enumerate()
            .flat_map(|(y, row)| {
                row.chars().enumerate()
                    .filter_map(move |(x, ch)| match ch {
                        '#' => Some((x as i32, y as i32)),
                        '.' => None,
                        _ => panic!("bad present"),
                    })
            }).collect::<Vec<Pos>>()
    }).collect::<Vec<Present>>();
    let trees = tree_txt.next().unwrap()
        .lines()
        .map(|l| {
            let (sz, quant) = l.split_once(": ").unwrap();
            let (w, h) = sz.split_once('x').unwrap();
            let size = (w.parse().unwrap(), h.parse().unwrap());
            let quantities = quant.split(' ').map(|q| q.parse().unwrap()).collect();
            Tree { size, quantities }
        })
    .collect::<Vec<Tree>>();
    (presents, trees)
}

fn main() {
    let mut file = String::new();
    io::stdin().read_to_string(&mut file).unwrap();
    let (presents, trees) = parse(&file);
    println!("{}", regions_fit(&presents, &trees));
}
