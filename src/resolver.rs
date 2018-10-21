use engine::*;
use std::collections::HashSet;

#[derive(Eq, PartialEq, Hash, Clone)]
struct Snapshot {
    player_area: Vec<Coordinate>,
    map: Map,
}

pub fn resolve(map: &Map, coordinate: &Coordinate) {
    let searched: HashSet<Snapshot> = HashSet::new();
    let path = Vec::<Snapshot>::new();
    explore_player_area(map, coordinate);
}

fn candidates(map: &Map, player_area: &HashSet<Coordinate>) -> Vec<Map> {
    for coordinate in player_area {
        for (mi, mj, li, lj) in &[(1, 0, 2, 0), (-1, 0, -2, 0), (0, 1, 0, 2), (0, -1, 0, -2)] {
            // let x = ((head.0 as i32 + mi) as usize, (head.1 as i32 + mj) as usize);
            // let y = ((head.0 as i32 + li) as usize, (head.1 as i32 + lj) as usize);
            // let mut candidate = map.clone();
            // let origin = (
            //     self.candidate[head.0][head.1].clone(),
            //     self.candidate[x.0][x.1].clone(),
            //     self.candidate[y.0][y.1].clone(),
            // );
            // let (t, moved) = Cell::shift(&origin);
            // if moved {
            //     candidate.
            // }
        }
    }
    vec![]
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
