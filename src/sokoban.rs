
pub struct Level {
    pub index: i32,
    pub height: usize,
    pub width: usize,
    pub layout: Box<[&'static str]>,
    pub map: Vec<Vec<Cell>>,
    pub player: (usize, usize),
}

pub enum Cell {
    PLAYER(usize, usize),
    WALL(usize, usize),
    BOX(usize, usize),
    EMPTY(usize, usize),
    TARGET(usize, usize),
    PLAYER_ON_TARGET(usize, usize),
    BOX_ON_TARGET(usize, usize),
}

impl Cell {
    fn mv(from: Cell, push: Cell, to: Cell) -> (Cell, Cell, Cell, bool) {
        match from {
            Cell::PLAYER_ON_TARGET(pl, pc) => {
                match push {
                    Cell::WALL(wl, wc) => (from, push, to, false),
                    Cell::EMPTY(el, ec) => (Cell::TARGET(pl, pc), Cell::PLAYER(el, ec), to, true),
                    Cell::TARGET(tl, tc) => {
                        (Cell::TARGET(pl, pc), Cell::PLAYER_ON_TARGET(tl, tc), to, true)
                    }
                    Cell::BOX(bl, bc) => {
                        match to {
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
                            _ => (from, push, to, false),
                        }
                    }
                    Cell::BOX_ON_TARGET(bl, bc) => {
                        match to {
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
                            _ => (from, push, to, false),
                        }
                    }
                    _ => (from, push, to, false),
                }
            }
            Cell::PLAYER(pl, pc) => {
                match push {
                    Cell::WALL(wl, wc) => (from, push, to, false),
                    Cell::EMPTY(el, ec) => (Cell::EMPTY(pl, pc), Cell::PLAYER(el, ec), to, true),
                    Cell::TARGET(tl, tc) => {
                        (Cell::EMPTY(pl, pc), Cell::PLAYER_ON_TARGET(tl, tc), to, true)
                    }
                    Cell::BOX(bl, bc) => {
                        match to {
                            Cell::EMPTY(el, ec) => {
                                (Cell::EMPTY(pl, pc), Cell::PLAYER(bl, bc), Cell::BOX(el, ec), true)
                            }
                            Cell::TARGET(tl, tc) => {
                                (Cell::EMPTY(pl, pc),
                                 Cell::PLAYER(bl, bc),
                                 Cell::BOX_ON_TARGET(tl, tc),
                                 true)
                            }
                            _ => (from, push, to, false),
                        }
                    }
                    Cell::BOX_ON_TARGET(bl, bc) => {
                        match to {
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
                            _ => (from, push, to, false),
                        }
                    }
                    _ => (from, push, to, false),
                }
            }
            _ => (from, push, to, false),
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
        }
    }

    pub fn is_pass(&self) -> bool {
        let mut l = 0;
        while l < self.map.len() {
            let mut c = 0;
            while c < self.map[l as usize].len() {
                match self.map[l][c] {
                    Cell::BOX(i, j) => return false,
                    _ => {}
                }
                c += 1;
            }
            l += 1;
        }
        true
    }

    pub fn reset(&mut self) {
        self.map.clear();
        let (h, w, mut m, p) = Level::build_map(&self.layout);
        self.player = p;
        self.map.append(&mut m);
        self.width = w;
        self.height = h;
        // self.paint();
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

    pub fn rmove(&mut self) {
        let (l, c) = (self.player.0 as usize, self.player.1 as usize);
        let from = self.map[l].remove(c);
        let push = self.map[l].remove(c);
        let to = self.map[l].remove(c);
        let (from, push, to, moved) = Cell::mv(from, push, to);
        self.map[l].insert(c, from);
        self.map[l].insert(c + 1, push);
        self.map[l].insert(c + 2, to);
        if moved {
            self.player = (l, c + 1);
        }
        // self.paint();
    }

    pub fn lmove(&mut self) {
        let (l, c) = (self.player.0 as usize, self.player.1 as usize);
        let to = self.map[l].remove(c - 2);
        let push = self.map[l].remove(c - 2);
        let from = self.map[l].remove(c - 2);
        let (from, push, to, moved) = Cell::mv(from, push, to);
        self.map[l].insert(c - 2, to);
        self.map[l].insert(c - 1, push);
        self.map[l].insert(c, from);
        if moved {
            self.player = (l, c - 1);
        }
        // self.paint();
    }

    pub fn umove(&mut self) {
        let (l, c) = (self.player.0 as usize, self.player.1 as usize);
        let from = self.map[l].remove(c);
        let push = self.map[l - 1].remove(c);
        let to = self.map[l - 2].remove(c);
        let (from, push, to, moved) = Cell::mv(from, push, to);
        self.map[l].insert(c, from);
        self.map[l - 1].insert(c, push);
        self.map[l - 2].insert(c, to);
        if moved {
            self.player = (l - 1, c);
        }
        // self.paint();
    }

    pub fn bmove(&mut self) {
        let (l, c) = (self.player.0 as usize, self.player.1 as usize);
        let from = self.map[l].remove(c);
        let push = self.map[l + 1].remove(c);
        let to = self.map[l + 2].remove(c);
        let (from, push, to, moved) = Cell::mv(from, push, to);
        self.map[l].insert(c, from);
        self.map[l + 1].insert(c, push);
        self.map[l + 2].insert(c, to);
        if moved {
            self.player = (l + 1, c);
        }
        // self.paint();
    }
}
