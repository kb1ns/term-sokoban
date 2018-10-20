use engine::*;
use std::collections::HashSet;

#[derive(Eq, PartialEq, Hash, Clone)]
struct Snapshot {
    player_area: Vec<(usize, usize)>,
    map: Map,
}

pub fn resolve(map: &Map, coordinate: (usize, usize)) {
    let searched: HashSet<Snapshot> = HashSet::new();
    let path = Vec::<Snapshot>::new();
}

fn candidates(map: &Map, player_area: &HashSet<(usize, usize)>) -> Vec<Map> {

    vec![]
}

fn test(map: &Map) -> bool {
    false
}

fn explore_player_area(map: &Map, coordinate: (usize, usize)) -> HashSet<(usize, usize)> {
    let mut player_area = HashSet::<(usize, usize)>::new();
    // player_area.insert(coordinate);
    // let mut queue = Vec::<(usize, usize)>::new();
    // queue.push(coordinate);
    // while !queue.is_empty() {
    //     let head = queue.remove(0);
    //     for (i, j) in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
    //         match map[head.0 + i][head.1 + j] {
    //             Cell::Empty(x, y) => {
    //                 queue.push((x, y));
    //                 player_area.insert((x, y));
    //             }
    //             Cell::Target(x, y) => {
    //                 queue.push((x, y));
    //                 player_area.insert((x, y));
    //             }
    //             _ => {}
    //         }
    //     }
    // }
    player_area
}
