use std::io::{self, BufRead};
use std::collections::{HashSet, VecDeque};

type Map = [Vec<u8>];
type Node = (usize, usize);

fn bfs(map: &Map, start: Node, end: Node) -> u32 {
    let w = map[0].len();
    let h = map.len();
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((start, 0));
    visited.insert(start);

    let mut push = |q: &mut VecDeque<_>, pos: Node, next: Node, d: u32| {
        let pos_elev = if pos == start { b'a' } else { map[pos.1][pos.0] };
        let next_elev = if next == end { b'z' } else { map[next.1][next.0] };
        let reachable = pos_elev + 1 == next_elev || pos_elev >= next_elev;
        let seen = visited.contains(&next);
        if reachable && !seen {
            q.push_back((next, d));
            visited.insert(next);
        }
    };

    while let Some((pos, dist)) = queue.pop_front() {
        if pos == end {
            return dist;
        }
        if pos.0 > 0 {
            push(&mut queue, pos, (pos.0 - 1, pos.1), dist + 1);
        }
        if pos.1 > 0 {
            push(&mut queue, pos, (pos.0, pos.1 - 1), dist + 1);
        }
        if pos.0 < w - 1 {
            push(&mut queue, pos, (pos.0 + 1, pos.1), dist + 1);
        }
        if pos.1 < h - 1 {
            push(&mut queue, pos, (pos.0, pos.1 + 1), dist + 1);
        }
    }

    std::i32::MAX
}

fn find_pos(map: &Map, item: u8) -> Node {
    map.iter().enumerate().flat_map(|(y, row)| {
        row.iter().enumerate().map(move |(x, &cell)| ((x, y), cell))
    }).find(|&(_, cell)| cell == item).unwrap().0
}

fn path_to_end(map: &Map) -> u32 {
    let start = find_pos(map, b'S');
    let end = find_pos(map, b'E');

    bfs(map, start, end)
}

fn best_path_to_end(map: &Map) -> i32 {
    let end = find_pos(map, b'E');

    map.iter().enumerate().flat_map(|(y, row)| {
        row.iter().enumerate().map(move |(x, &cell)| ((x, y), cell))
    })
    .filter(|&(_, cell)| cell == b'a' || cell == b'S')
    .map(|(pos, _)| bfs(map, pos, end))
    .min()
    .unwrap()
}

fn main() {
    let map: Vec<_> = io::stdin().lock().lines()
        .map(|line| line.unwrap().into_bytes())
        .collect();
    println!("{}", path_to_end(&map));
    println!("{}", best_path_to_end(&map));
}
