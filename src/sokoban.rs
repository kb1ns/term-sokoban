
#[derive(Clone, Eq, PartialEq)]
pub enum Cell {
    Player(usize, usize),
    Wall(usize, usize),
    Box(usize, usize),
    Empty(usize, usize),
    Target(usize, usize),
    PlayerOnTarget(usize, usize),
    BoxOnTarget(usize, usize),
}

pub struct Level {
    pub index: usize,
    height: usize,
    width: usize,
    layout: Box<[&'static str]>,
    pub map: Vec<Vec<Cell>>,
    player: (usize, usize),
    record: Vec<(usize, usize, Cell, usize, usize, Cell, usize, usize, Cell)>,
}


impl Cell {
    fn volume(&self) -> u32 {
        match *self {
            Cell::BoxOnTarget(_, _) => 1,
            Cell::Box(_, _) => 1,
            Cell::Wall(_, _) => 1,
            Cell::PlayerOnTarget(_, _) => 0,
            Cell::Player(_, _) => 0,
            Cell::Empty(_, _) => 0,
            Cell::Target(_, _) => 0,
        }
    }

    fn mv(from: &Cell, push: &Cell, to: &Cell) -> (Cell, Cell, Cell, bool) {
        match *from {
            Cell::PlayerOnTarget(pl, pc) => {
                match *push {
                    Cell::Wall(wl, wc) => {
                        (Cell::PlayerOnTarget(pl, pc), Cell::Wall(wl, wc), to.clone(), false)
                    }
                    Cell::Empty(el, ec) => {
                        (Cell::Target(pl, pc), Cell::Player(el, ec), to.clone(), true)
                    }
                    Cell::Target(tl, tc) => {
                        (Cell::Target(pl, pc), Cell::PlayerOnTarget(tl, tc), to.clone(), true)
                    }
                    Cell::Box(bl, bc) => {
                        match *to {
                            Cell::Empty(el, ec) => {
                                (Cell::Target(pl, pc),
                                 Cell::Player(bl, bc),
                                 Cell::Box(el, ec),
                                 true)
                            }
                            Cell::Target(tl, tc) => {
                                (Cell::Target(pl, pc),
                                 Cell::Player(bl, bc),
                                 Cell::BoxOnTarget(tl, tc),
                                 true)
                            }
                            _ => (from.clone(), push.clone(), to.clone(), false),
                        }
                    }
                    Cell::BoxOnTarget(bl, bc) => {
                        match *to {
                            Cell::Empty(el, ec) => {
                                (Cell::Target(pl, pc),
                                 Cell::PlayerOnTarget(bl, bc),
                                 Cell::Box(el, ec),
                                 true)
                            }
                            Cell::Target(tl, tc) => {
                                (Cell::Target(pl, pc),
                                 Cell::PlayerOnTarget(bl, bc),
                                 Cell::BoxOnTarget(tl, tc),
                                 true)
                            }
                            _ => (from.clone(), push.clone(), to.clone(), false),
                        }
                    }
                    _ => (from.clone(), push.clone(), to.clone(), false),
                }
            }
            Cell::Player(pl, pc) => {
                match *push {
                    Cell::Wall(_, _) => (from.clone(), push.clone(), to.clone(), false),
                    Cell::Empty(el, ec) => {
                        (Cell::Empty(pl, pc), Cell::Player(el, ec), to.clone(), true)
                    }
                    Cell::Target(tl, tc) => {
                        (Cell::Empty(pl, pc), Cell::PlayerOnTarget(tl, tc), to.clone(), true)
                    }
                    Cell::Box(bl, bc) => {
                        match *to {
                            Cell::Empty(el, ec) => {
                                (Cell::Empty(pl, pc), Cell::Player(bl, bc), Cell::Box(el, ec), true)
                            }
                            Cell::Target(tl, tc) => {
                                (Cell::Empty(pl, pc),
                                 Cell::Player(bl, bc),
                                 Cell::BoxOnTarget(tl, tc),
                                 true)
                            }
                            _ => (from.clone(), push.clone(), to.clone(), false),
                        }
                    }
                    Cell::BoxOnTarget(bl, bc) => {
                        match *to {
                            Cell::Empty(el, ec) => {
                                (Cell::Empty(pl, pc),
                                 Cell::PlayerOnTarget(bl, bc),
                                 Cell::Box(el, ec),
                                 true)
                            }
                            Cell::Target(tl, tc) => {
                                (Cell::Empty(pl, pc),
                                 Cell::PlayerOnTarget(bl, bc),
                                 Cell::BoxOnTarget(tl, tc),
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
    pub fn new(i: usize, cellstr: Box<[&'static str]>) -> Self {
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

    pub fn is_pass(&self) -> bool {
        for l in self.map.as_slice() {
            for c in l {
                match c {
                    &Cell::Box(_, _) => return false,
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
        let h: usize = cellstr.as_ref().len();
        let mut w: usize = 0;
        let mut p: (usize, usize) = (0, 0);
        for (l, line) in cellstr.as_ref().iter().enumerate() {
            w = line.len();
            let mut mm: Vec<Cell> = Vec::new();
            for (c, col) in line.chars().enumerate() {
                match col {
                    ' ' => {
                        mm.push(Cell::Empty(l, c));
                    }
                    '#' => {
                        mm.push(Cell::Wall(l, c));
                    }
                    'o' => {
                        mm.push(Cell::Box(l, c));
                    }
                    'x' => {
                        mm.push(Cell::Target(l, c));
                    }
                    'i' => {
                        mm.push(Cell::Player(l, c));
                        p = (l, c);
                    }
                    'I' => {
                        mm.push(Cell::PlayerOnTarget(l, c));
                        p = (l, c);
                    }
                    'O' => {
                        mm.push(Cell::BoxOnTarget(l, c));
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
}
