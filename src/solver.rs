use engine::*;
// for debug
use ncurses::*;

use std;
use std::collections::BTreeSet;
use std::collections::HashSet;

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct Snapshot {
    player_area: Area,
    pub map: Map,
}

fn debug(map: &Map, player: &Coordinate) {
    let starty = (LINES() - map.len() as i32) / 2;
    let startx = (COLS() - map[0].len() as i32) / 2;
    for (i, row) in map.iter().enumerate() {
        for (j, column) in row.iter().enumerate() {
            match column {
                &Cell::Player => {
                    mvaddch(starty + i as i32, startx + j as i32, 'i' as u32);
                }
                &Cell::Wall => {
                    attron(A_DIM());
                    mvaddch(starty + i as i32, startx + j as i32, ACS_CKBOARD());
                    attroff(A_DIM());
                }
                &Cell::Case => {
                    mvaddch(starty + i as i32, startx + j as i32, 'o' as u32);
                }
                &Cell::Target => {
                    attron(A_DIM());
                    mvaddch(starty + i as i32, startx + j as i32, 'x' as u32);
                    attroff(A_DIM());
                }
                &Cell::Ground => {
                    mvaddch(starty + i as i32, startx + j as i32, ' ' as u32);
                }
                &Cell::PlayerOnTarget => {
                    mvaddch(starty + i as i32, startx + j as i32, 'I' as u32);
                }
                &Cell::CaseOnTarget => {
                    color_set(COLOR_RED);
                    attron(A_BOLD() | A_BLINK() | COLOR_PAIR(COLOR_RED));
                    mvaddch(starty + i as i32, startx + j as i32, 'O' as u32);
                    attroff(A_BOLD() | A_BLINK() | COLOR_PAIR(COLOR_RED));
                }
            };
        }
    }
    mvprintw(0, (COLS() - 12), "Explore Mode");
    mvaddch(starty + player.0 as i32, startx + player.1 as i32, 'I' as u32);
    if getch() == 0x71 {
        endwin();
        std::process::exit(0);
    }
    refresh();
}

type Area = BTreeSet<Coordinate>;

pub fn solve(map: &Map, coordinate: &Coordinate) -> Vec<Snapshot> {
    // we don't care about the player's coordinate in the equivalent area
    let toggle = match &map[coordinate.0][coordinate.1] {
        &Cell::Player => Cell::Ground,
        &Cell::PlayerOnTarget => Cell::Target,
        _ => panic!("won't happend"),
    };
    let mut init_map = map.clone();
    init_map[coordinate.0][coordinate.1] = toggle;
    let init_area = explore_player_area(&map, &coordinate);
    let snapshot = Snapshot {
        map: init_map,
        player_area: init_area,
    };
    let mut searched: HashSet<Snapshot> = HashSet::new();
    let mut path = Vec::<Snapshot>::new();
    searched.insert(snapshot.clone());
    path.push(snapshot);
    dfs(&mut searched, &mut path);
    path
}

fn dfs(searched: &mut HashSet<Snapshot>, path: &mut Vec<Snapshot>) -> bool {
    let len = path.len();
    mvprintw(0, 0, &format!("stack size: {}", len));
    let branches = match path.last() {
        None => {
            return false;
        }
        Some(ref current) => {
            if is_pass(&current.map) {
                return true;
            }
            if is_dead(&current.map, &current.player_area) {
                return false;
            }
            get_branches(&current.map, &current.player_area)
        }
    };
    for branch in branches.into_iter() {
        let (branch, player) = branch;
        if !searched.contains(&branch) {
            searched.insert(branch.clone());
            path.push(branch.clone());

            debug(&branch.map, &player);

            if dfs(searched, path) {
                return true;
            } else {
                path.pop();
            }
        }
    }
    false
}

pub fn get_branches(map: &Map, player_area: &Area) -> Vec<(Snapshot, Coordinate)> {
    let mut branches = vec![];
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
            let mut new_map = map.clone();
            let (_, moved) = update(&mut new_map, &(coordinate.clone(), x.clone(), y));
            if moved && map[x.0][x.1].is_case() {
                let new_area = explore_player_area(&new_map, &x);
                branches.push((
                    Snapshot {
                        map: new_map,
                        player_area: new_area,
                    },
                    x,
                ));
            }
        }
    }
    branches
}

fn is_dead(map: &Map, area: &Area) -> bool {
    for (i, row) in map.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if cell == &Cell::Case && immovable(map, &(i, j)) {
                return true;
            }
        }
    }
    false
}

fn immovable(map: &Map, c: &Coordinate) -> bool {
    let b0 = !map[c.0 - 1][c.1].reachable()
        && !map[c.0][c.1 + 1].reachable()
        && !map[c.0 - 1][c.1 + 1].reachable();
    let b1 = !map[c.0 - 1][c.1].reachable()
        && !map[c.0][c.1 - 1].reachable()
        && !map[c.0 - 1][c.1 - 1].reachable();
    let b2 = !map[c.0 + 1][c.1].reachable()
        && !map[c.0][c.1 + 1].reachable()
        && !map[c.0 + 1][c.1 + 1].reachable();
    let b3 = !map[c.0 + 1][c.1].reachable()
        && !map[c.0][c.1 - 1].reachable()
        && !map[c.0 + 1][c.1 - 1].reachable();
    b0 || b1 || b2 || b3
}

fn explore_player_area(map: &Map, coordinate: &Coordinate) -> Area {
    let mut player_area = Area::new();
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
