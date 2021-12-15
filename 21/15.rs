use std::io::{self, BufRead};
use std::collections::{HashMap, BinaryHeap};

fn chiton_dijkstra(map: &HashMap<(i32, i32), i32>, origin: (i32, i32), destination: (i32, i32)) -> i32 {
    let mut heap: BinaryHeap::<(i32, (i32, i32))> = BinaryHeap::new(); // -dist, coords
    let mut distances = HashMap::<(i32, i32), i32>::new(); // coord to cost
    heap.push((0, origin));

    while let Some(current) = heap.pop() {
        let (dist_i, (xi, yi)) = current;
        let dist_i = -dist_i;

        let neighs = &[
            (xi - 1, yi),
            (xi + 1, yi),
            (xi, yi - 1),
            (xi, yi + 1),
        ];

        for &(xj, yj) in neighs {
            if let Some(dist_ij) = map.get(&(xj, yj)) {
                let dist_j = dist_i + dist_ij;

                if dist_j < *distances.get(&(xj, yj)).unwrap_or(&std::i32::MAX) {
                    heap.push((-dist_j, (xj, yj)));
                    distances.insert((xj, yj), dist_j);
                }
            }
        }
    }

    *distances.get(&destination).unwrap()
}

fn total_risk(chitons: &[Vec<u8>]) -> i32 {
    let map: HashMap<(i32, i32), i32> = chitons
        .iter().enumerate().flat_map(|(y, row)| {
            row.iter().enumerate().map(move |(x, &ch)| {
                ((x as i32, y as i32), ch as i32)
            })
        })
    .collect();
    chiton_dijkstra(&map, (0, 0), ((chitons[0].len() - 1) as i32, (chitons.len() - 1) as i32))
}

fn main() {
    let chitons: Vec<Vec<u8>> = io::stdin().lock().lines()
        .map(|line| line.unwrap().bytes().map(|b| b - b'0').collect())
        .collect();
    println!("{:?}", total_risk(&chitons));
}
