use std::collections::HashMap;

// use ncurses::*;

pub enum Cell {
    PLAYER(usize, usize),
    WALL(usize, usize),
    BOX(usize, usize),
    EMPTY(usize, usize),
    TARGET(usize, usize),
    PLAYER_ON_TARGET(usize, usize),
    BOX_ON_TARGET(usize, usize),
}

pub struct Level {
    index: i32,
    height: usize,
    width: usize,
    layout: Box<[&'static str]>,
    pub map: Vec<Vec<Cell>>,
    player: (usize, usize),
    record: Vec<(usize, usize, Cell, usize, usize, Cell, usize, usize, Cell)>,
}

impl Clone for Cell {
    fn clone(&self) -> Self {
        match *self {
            Cell::PLAYER(i, j) => Cell::PLAYER(i, j),
            Cell::PLAYER_ON_TARGET(i, j) => Cell::PLAYER_ON_TARGET(i, j),
            Cell::WALL(i, j) => Cell::WALL(i, j),
            Cell::BOX(i, j) => Cell::BOX(i, j),
            Cell::BOX_ON_TARGET(i, j) => Cell::BOX_ON_TARGET(i, j),
            Cell::TARGET(i, j) => Cell::TARGET(i, j),
            Cell::EMPTY(i, j) => Cell::EMPTY(i, j),
        }
    }
}

impl Cell {
    fn volume(&self) -> u32 {
        match *self {
            Cell::BOX_ON_TARGET(_, _) => 1,
            Cell::BOX(_, _) => 1,
            Cell::WALL(_, _) => 1,
            Cell::PLAYER_ON_TARGET(_, _) => 0,
            Cell::PLAYER(_, _) => 0,
            Cell::EMPTY(_, _) => 0,
            Cell::TARGET(_, _) => 0,
        }
    }

    fn mv(from: &Cell, push: &Cell, to: &Cell) -> (Cell, Cell, Cell, bool) {
        match *from {
            Cell::PLAYER_ON_TARGET(pl, pc) => {
                match *push {
                    Cell::WALL(wl, wc) => {
                        (Cell::PLAYER_ON_TARGET(pl, pc), Cell::WALL(wl, wc), to.clone(), false)
                    }
                    Cell::EMPTY(el, ec) => {
                        (Cell::TARGET(pl, pc), Cell::PLAYER(el, ec), to.clone(), true)
                    }
                    Cell::TARGET(tl, tc) => {
                        (Cell::TARGET(pl, pc), Cell::PLAYER_ON_TARGET(tl, tc), to.clone(), true)
                    }
                    Cell::BOX(bl, bc) => {
                        match *to {
                            Cell::EMPTY(el, ec) => {
                                (Cell::TARGET(pl, pc),
                                 Cell::PLAYER(bl, bc),
                                 Cell::BOX(el, ec),
                                 true)
                            }
                            Cell::TARGET(tl, tc) => {
                                (Cell::TARGET(pl, pc),
                                 Cell::PLAYER(bl, bc),
                                 Cell::BOX_ON_TARGET(tl, tc),
                                 true)
                            }
                            _ => (from.clone(), push.clone(), to.clone(), false),
                        }
                    }
                    Cell::BOX_ON_TARGET(bl, bc) => {
                        match *to {
                            Cell::EMPTY(el, ec) => {
                                (Cell::TARGET(pl, pc),
                                 Cell::PLAYER_ON_TARGET(bl, bc),
                                 Cell::BOX(el, ec),
                                 true)
                            }
                            Cell::TARGET(tl, tc) => {
                                (Cell::TARGET(pl, pc),
                                 Cell::PLAYER_ON_TARGET(bl, bc),
                                 Cell::BOX_ON_TARGET(tl, tc),
                                 true)
                            }
                            _ => (from.clone(), push.clone(), to.clone(), false),
                        }
                    }
                    _ => (from.clone(), push.clone(), to.clone(), false),
                }
            }
            Cell::PLAYER(pl, pc) => {
                match *push {
                    Cell::WALL(_, _) => (from.clone(), push.clone(), to.clone(), false),
                    Cell::EMPTY(el, ec) => {
                        (Cell::EMPTY(pl, pc), Cell::PLAYER(el, ec), to.clone(), true)
                    }
                    Cell::TARGET(tl, tc) => {
                        (Cell::EMPTY(pl, pc), Cell::PLAYER_ON_TARGET(tl, tc), to.clone(), true)
                    }
                    Cell::BOX(bl, bc) => {
                        match *to {
                            Cell::EMPTY(el, ec) => {
                                (Cell::EMPTY(pl, pc), Cell::PLAYER(bl, bc), Cell::BOX(el, ec), true)
                            }
                            Cell::TARGET(tl, tc) => {
                                (Cell::EMPTY(pl, pc),
                                 Cell::PLAYER(bl, bc),
                                 Cell::BOX_ON_TARGET(tl, tc),
                                 true)
                            }
                            _ => (from.clone(), push.clone(), to.clone(), false),
                        }
                    }
                    Cell::BOX_ON_TARGET(bl, bc) => {
                        match *to {
                            Cell::EMPTY(el, ec) => {
                                (Cell::EMPTY(pl, pc),
                                 Cell::PLAYER_ON_TARGET(bl, bc),
                                 Cell::BOX(el, ec),
                                 true)
                            }
                            Cell::TARGET(tl, tc) => {
                                (Cell::EMPTY(pl, pc),
                                 Cell::PLAYER_ON_TARGET(bl, bc),
                                 Cell::BOX_ON_TARGET(tl, tc),
                                 true)
                            }
                            _ => (from.clone(), push.clone(), to.clone(), false),
                        }
                    }
                    _ => (from.clone(), push.clone(), to.clone(), false),
                }
            }
            _ => (from.clone(), push.clone(), to.clone(), false),
        }
    }
}


impl Level {
    pub fn new(i: i32, cellstr: Box<[&'static str]>) -> Self {
        let (line, col, mm, pp) = Level::build_map(&cellstr);
        Level {
            index: i,
            layout: cellstr,
            height: line,
            width: col,
            player: pp,
            map: mm,
            record: Vec::new(),
        }
    }

    fn is_dead(&self) -> bool {
        for l in self.map.as_slice() {
            for c in l {
                match c {
                    &Cell::BOX(bl, bc) => {
                        return self.map[bl - 1][bc].volume() + self.map[bl][bc - 1].volume() == 2 ||
                               self.map[bl - 1][bc].volume() + self.map[bl][bc + 1].volume() == 2 ||
                               self.map[bl + 1][bc].volume() + self.map[bl][bc - 1].volume() == 2 ||
                               self.map[bl + 1][bc].volume() + self.map[bl][bc + 1].volume() == 2;
                    }
                    _ => {}
                }
            }
        }
        false
    }

    pub fn is_pass(&self) -> bool {
        for l in self.map.as_slice() {
            for c in l {
                match c {
                    &Cell::BOX(_, _) => return false,
                    _ => {}
                }
            }
        }
        true
    }

    pub fn reset(&mut self) {
        self.map.clear();
        let (h, w, mut m, p) = Level::build_map(&self.layout);
        self.player = p;
        self.record.clear();
        self.map.append(&mut m);
        self.width = w;
        self.height = h;
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    fn build_map(cellstr: &Box<[&str]>) -> (usize, usize, Vec<Vec<Cell>>, (usize, usize)) {
        let mut m: Vec<Vec<Cell>> = Vec::new();
        let mut h: usize = cellstr.as_ref().len();
        let mut w: usize = 0;
        let mut p: (usize, usize) = (0, 0);
        for (l, line) in cellstr.as_ref().iter().enumerate() {
            w = line.len();
            let mut mm: Vec<Cell> = Vec::new();
            for (c, col) in line.chars().enumerate() {
                match col {
                    ' ' => {
                        mm.push(Cell::EMPTY(l, c));
                    }
                    '#' => {
                        mm.push(Cell::WALL(l, c));
                    }
                    'o' => {
                        mm.push(Cell::BOX(l, c));
                    }
                    'x' => {
                        mm.push(Cell::TARGET(l, c));
                    }
                    'i' => {
                        mm.push(Cell::PLAYER(l, c));
                        p = (l, c);
                    }
                    'I' => {
                        mm.push(Cell::PLAYER_ON_TARGET(l, c));
                        p = (l, c);
                    }
                    'O' => {
                        mm.push(Cell::BOX_ON_TARGET(l, c));
                    }
                    _ => {
                        panic!("Err: Illegal char in map.");
                    }
                };
            }
            m.push(mm);
        }
        (h, w, m, p)
    }

    pub fn move_right(&mut self) -> bool {
        let (l, c) = (self.player.0, self.player.1);
        let from = self.map[l].remove(c);
        let push = self.map[l].remove(c);
        let to = self.map[l].remove(c);
        let (nfrom, npush, nto, moved) = Cell::mv(&from, &push, &to);
        self.map[l].insert(c, nfrom);
        self.map[l].insert(c + 1, npush);
        self.map[l].insert(c + 2, nto);
        if moved {
            self.player = (l, c + 1);
            self.record.push((l, c, from, l, c + 1, push, l, c + 2, to));
        }
        moved
    }

    pub fn move_left(&mut self) -> bool {
        let (l, c) = (self.player.0, self.player.1);
        let to = self.map[l].remove(c - 2);
        let push = self.map[l].remove(c - 2);
        let from = self.map[l].remove(c - 2);
        let (nfrom, npush, nto, moved) = Cell::mv(&from, &push, &to);
        self.map[l].insert(c - 2, nto);
        self.map[l].insert(c - 1, npush);
        self.map[l].insert(c, nfrom);
        if moved {
            self.player = (l, c - 1);
            self.record.push((l, c, from, l, c - 1, push, l, c - 2, to));
        }
        moved
    }

    pub fn move_upward(&mut self) -> bool {
        let (l, c) = (self.player.0, self.player.1);
        let from = self.map[l].remove(c);
        let push = self.map[l - 1].remove(c);
        let to = self.map[l - 2].remove(c);
        let (nfrom, npush, nto, moved) = Cell::mv(&from, &push, &to);
        self.map[l].insert(c, nfrom);
        self.map[l - 1].insert(c, npush);
        self.map[l - 2].insert(c, nto);
        if moved {
            self.player = (l - 1, c);
            self.record.push((l, c, from, l - 1, c, push, l - 2, c, to));
        }
        moved
    }

    pub fn move_down(&mut self) -> bool {
        let (l, c) = (self.player.0, self.player.1);
        let from = self.map[l].remove(c);
        let push = self.map[l + 1].remove(c);
        let to = self.map[l + 2].remove(c);
        let (nfrom, npush, nto, moved) = Cell::mv(&from, &push, &to);
        self.map[l].insert(c, nfrom);
        self.map[l + 1].insert(c, npush);
        self.map[l + 2].insert(c, nto);
        if moved {
            self.player = (l + 1, c);
            self.record.push((l, c, from, l + 1, c, push, l + 2, c, to));
        }
        moved
    }

    pub fn revert(&mut self) {
        match self.record.pop() {
            Some((fl, fc, from, pl, pc, push, tl, tc, to)) => {
                self.map[fl].remove(fc);
                self.map[fl].insert(fc, from);
                self.map[pl].remove(pc);
                self.map[pl].insert(pc, push);
                self.map[tl].remove(tc);
                self.map[tl].insert(tc, to);
                self.player = (fl, fc);
            }
            None => {}
        }
    }

    fn can_move_upward(&mut self, stats: &mut HashMap<String, (usize, usize)>) -> bool {
        let mut b = self.move_upward();
        if b {
            if stats.contains_key(&self.sig()) {
                b = false;
            }
            self.revert();
        }
        b
    }

    fn can_move_down(&mut self, stats: &mut HashMap<String, (usize, usize)>) -> bool {
        let mut b = self.move_down();
        if b {
            if stats.contains_key(&self.sig()) {
                b = false;
            }
            self.revert();
        }
        b
    }

    fn can_move_right(&mut self, stats: &mut HashMap<String, (usize, usize)>) -> bool {
        let mut b = self.move_right();
        if b {
            if stats.contains_key(&self.sig()) {
                b = false;
            }
            self.revert();
        }
        b
    }

    fn can_move_left(&mut self, stats: &mut HashMap<String, (usize, usize)>) -> bool {
        let mut b = self.move_left();
        if b {
            if stats.contains_key(&self.sig()) {
                b = false;
            }
            self.revert();
        }
        b
    }

    //TODO
    fn automove(&mut self) {
        let mut stats: HashMap<String, (usize, usize)> = HashMap::new();
        self.dfs(&mut stats);
    }

    fn sig(&self) -> String {
        let mut s = String::new();
        for l in self.map.as_slice() {
            for c in l {
                s.push(match c {
                    &Cell::BOX(_, _) => 'o',
                    &Cell::BOX_ON_TARGET(_, _) => 'O',
                    &Cell::WALL(_, _) => '#',
                    &Cell::PLAYER(_, _) => 'i',
                    &Cell::PLAYER_ON_TARGET(_, _) => 'I',
                    &Cell::EMPTY(_, _) => ' ',
                    &Cell::TARGET(_, _) => 'x',
                });
            }
        }
        s
    }

    // fn paint(&self) {
    //     let starty = (LINES() - self.height as i32) / 2;
    //     let startx = (COLS() - self.width as i32) / 2;
    //     for l in self.map.as_slice() {
    //         for c in l {
    //             match *c {
    //                 Cell::PLAYER(i, j) => {
    //                     mvprintw(starty + i as i32, startx + j as i32, "i");
    //                 }
    //                 Cell::WALL(i, j) => {
    //                     mvprintw(starty + i as i32, startx + j as i32, "#");
    //                 }
    //                 Cell::BOX(i, j) => {
    //                     mvprintw(starty + i as i32, startx + j as i32, "o");
    //                 }
    //                 Cell::TARGET(i, j) => {
    //                     mvprintw(starty + i as i32, startx + j as i32, "x");
    //                 }
    //                 Cell::EMPTY(i, j) => {
    //                     mvprintw(starty + i as i32, startx + j as i32, " ");
    //                 }
    //                 Cell::PLAYER_ON_TARGET(i, j) => {
    //                     mvprintw(starty + i as i32, startx + j as i32, "I");
    //                 }
    //                 Cell::BOX_ON_TARGET(i, j) => {
    //                     mvprintw(starty + i as i32, startx + j as i32, "O");
    //                 }
    //             };
    //         }
    //     }
    //     refresh();
    // }

    //TODO
    fn dfs(&mut self, stats: &mut HashMap<String, (usize, usize)>) {
        if self.is_pass() {
            return;
        }
        if self.is_dead() {
            self.revert();
        }
        // stats.push(self.sig(), self.player);
        if self.can_move_upward(stats) {
            self.move_upward();
            let dump = self.sig();
            stats.insert(dump, self.player);
            self.dfs(stats);
        } else if self.can_move_down(stats) {
            self.move_down();
            let dump = self.sig();
            stats.insert(dump, self.player);
            self.dfs(stats);
        } else if self.can_move_left(stats) {
            self.move_left();
            let dump = self.sig();
            stats.insert(dump, self.player);
            self.dfs(stats);
        } else if self.can_move_right(stats) {
            self.move_right();
            let dump = self.sig();
            stats.insert(dump, self.player);
            self.dfs(stats);
        } else {
            self.revert();
            self.dfs(stats);
        }
    }
}
