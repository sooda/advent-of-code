use std::io::{self, BufRead};
use std::collections::{HashSet, VecDeque};

type Map = [Vec<u8>];
type Node = (usize, usize);

fn bfs(map: &Map, start: Node, end: Option<Node>) -> (u32, u32) {
    let w = map[0].len();
    let h = map.len();
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((start, 0));
    visited.insert(start);
    let mut min_a = std::u32::MAX;

    let mut push = |q: &mut VecDeque<_>, pos: Node, next: Node, d: u32| {
        let pos_elev = if pos == start { b'z' } else { map[pos.1][pos.0] };
        let next_elev = if Some(next) == end { b'a' } else { map[next.1][next.0] };
        // traversal happens in inverse order wrt. mission instructions
        let reachable = next_elev + 1 == pos_elev || next_elev >= pos_elev;
        let seen = visited.contains(&next);
        if reachable && !seen {
            q.push_back((next, d));
            visited.insert(next);
        }
    };

    while let Some((pos, dist)) = queue.pop_front() {
        if Some(pos) == end {
            return (dist, min_a);
        }
        if map[pos.1][pos.0] == b'a' {
            min_a = min_a.min(dist);
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

    (std::u32::MAX, min_a)
}

fn find_pos(map: &Map, item: u8) -> Node {
    map.iter().enumerate().flat_map(|(y, row)| {
        row.iter().enumerate().map(move |(x, &cell)| ((x, y), cell))
    }).find(|&(_, cell)| cell == item).unwrap().0
}

fn path_to_end(map: &Map) -> u32 {
    let start = find_pos(map, b'E');
    let end = find_pos(map, b'S');

    bfs(map, start, Some(end)).0
}

fn best_path_to_end(map: &Map) -> u32 {
    let start = find_pos(map, b'E');

    // could use the same bfs above and just not exit early,
    // but this is separate on purpose
    bfs(map, start, None).1
}

fn main() {
    let map: Vec<_> = io::stdin().lock().lines()
        .map(|line| line.unwrap().into_bytes())
        .collect();
    println!("{}", path_to_end(&map));
    println!("{}", best_path_to_end(&map));
}
