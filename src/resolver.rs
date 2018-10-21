use engine::*;
use std::collections::HashSet;

#[derive(Eq, PartialEq, Hash, Clone)]
struct Snapshot {
    player_area: Vec<Coordinate>,
    map: Map,
}

pub fn resolve(map: &Map, coordinate: &Coordinate) {
    let mut searched: HashSet<Snapshot> = HashSet::new();
    let mut path = Vec::<Snapshot>::new();
}

pub fn get_branches(map: &Map, player: &Coordinate) -> Vec<(Map, Coordinate)> {
    let player_area = explore_player_area(map, player);
    let mut candidates = vec![];
    for coordinate in player_area {
        for (mi, mj, li, lj) in &[(1, 0, 2, 0), (-1, 0, -2, 0), (0, 1, 0, 2), (0, -1, 0, -2)] {
            let x = (
                (coordinate.0 as i32 + mi) as usize,
                (coordinate.1 as i32 + mj) as usize,
            );
            let y = (
                (coordinate.0 as i32 + li) as usize,
                (coordinate.1 as i32 + lj) as usize,
            );
            let mut candidate = map.clone();
            let toggle = match &candidate[player.0][player.1] {
                &Cell::Player => Cell::Ground,
                &Cell::PlayerOnTarget => Cell::Target,
                _ => panic!("won't happend"),
            };
            candidate[player.0][player.1] = toggle;
            let (_, v) = update(&mut candidate, &(coordinate.clone(), x.clone(), y));
            if v {
                candidates.push((candidate, x));
            }
        }
    }
    candidates
}

fn test(map: &Map) -> bool {
    false
}

fn explore_player_area(map: &Map, coordinate: &Coordinate) -> HashSet<Coordinate> {
    let mut player_area = HashSet::<Coordinate>::new();
    let mut queue = Vec::<Coordinate>::new();
    player_area.insert(coordinate.clone());
    queue.push(coordinate.clone());
    while !queue.is_empty() {
        let head = queue.remove(0);
        for (i, j) in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let x = (head.0 as i32 + i) as usize;
            let y = (head.1 as i32 + j) as usize;
            if map[x][y].reachable() && player_area.insert((x, y)) {
                queue.push((x, y));
            }
        }
    }
    player_area
}
