#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum Cell {
    Case,
    Wall,
    Ground,
    Player,
    Target,
    PlayerOnTarget,
    CaseOnTarget,
}

type Triple = (Cell, Cell, Cell);

impl Cell {
    pub fn shift(triple: &Triple) -> (Triple, bool) {
        let (l, m, r) = triple;
        let (l_origin, l_move) = l.seprate();
        let (m_origin, m_move) = m.seprate();
        let m_overlay = m_origin.overlay(&l_move);
        let r_overlay = r.overlay(&m_move);
        let valid_vol = l.vol() + m.vol() + r.vol();
        let new_vol = l_origin.vol() + m_overlay.vol() + r_overlay.vol();
        if valid_vol == new_vol {
            ((l_origin, m_overlay, r_overlay), true)
        } else {
            ((l.clone(), m.clone(), r.clone()), false)
        }
    }

    pub fn reachable(&self) -> bool {
        match self {
            &Cell::Ground => true,
            &Cell::Target => true,
            &Cell::Player => true,
            &Cell::PlayerOnTarget => true,
            _ => false,
        }
    }

    fn vol(&self) -> u8 {
        match self {
            &Cell::Ground => 0,
            &Cell::Target => 0,
            _ => 1,
        }
    }

    fn overlay(&self, unit: &Self) -> Self {
        match self {
            &Cell::Ground => match unit {
                &Cell::Case => Cell::Case,
                &Cell::Player => Cell::Player,
                _ => Cell::Ground,
            },
            &Cell::Target => match unit {
                &Cell::Case => Cell::CaseOnTarget,
                &Cell::Player => Cell::PlayerOnTarget,
                _ => Cell::Target,
            },
            _ => self.clone(),
        }
    }

    fn seprate(&self) -> (Self, Self) {
        match self {
            &Cell::Case => (Cell::Ground, Cell::Case),
            &Cell::CaseOnTarget => (Cell::Target, Cell::Case),
            &Cell::Player => (Cell::Ground, Cell::Player),
            &Cell::PlayerOnTarget => (Cell::Target, Cell::Player),
            _ => (self.clone(), Cell::Ground),
        }
    }
}

#[test]
pub fn test_shift() {
    assert_eq!(
        Cell::shift((Cell::Player, Cell::Wall, Cell::Ground)),
        ((Cell::Player, Cell::Wall, Cell::Ground), false)
    );
    assert_eq!(
        Cell::shift((Cell::Player, Cell::Ground, Cell::Ground)),
        ((Cell::Ground, Cell::Player, Cell::Ground), true)
    );
    assert_eq!(
        Cell::shift((Cell::Player, Cell::Case, Cell::Wall)),
        ((Cell::Player, Cell::Case, Cell::Wall), false)
    );
}

pub type Map = Vec<Vec<Cell>>;

pub type Coordinate = (usize, usize);

pub fn update(map: &mut Map, triple: &(Coordinate, Coordinate, Coordinate)) -> (Triple, bool) {
    let origin = (
        map[(triple.0).0][(triple.0).1].clone(),
        map[(triple.1).0][(triple.1).1].clone(),
        map[(triple.2).0][(triple.2).1].clone(),
    );
    let (transform, moved) = Cell::shift(&origin);
    if moved {
        map[(triple.0).0][(triple.0).1] = transform.0;
        map[(triple.1).0][(triple.1).1] = transform.1;
        map[(triple.2).0][(triple.2).1] = transform.2;
    }
    (origin, moved)
}

pub struct Scene {
    pub player: Coordinate,
    pub map: Map,
    history: Vec<(Coordinate, Coordinate, Coordinate, Triple)>,
}

impl Scene {
    pub fn init() -> Scene {
        Scene {
            player: (0, 0),
            map: vec![],
            history: vec![],
        }
    }

    pub fn load(&mut self, layout: &[&str]) {
        self.map.clear();
        self.history.clear();
        for (r, row) in layout.iter().enumerate() {
            let mut columns: Vec<Cell> = Vec::with_capacity(row.len());
            for (c, column) in row.chars().enumerate() {
                columns.push(match column {
                    ' ' => Cell::Ground,
                    '#' => Cell::Wall,
                    'o' => Cell::Case,
                    'O' => Cell::CaseOnTarget,
                    'x' => Cell::Target,
                    'i' => {
                        self.player = (r, c);
                        Cell::Player
                    }
                    'I' => {
                        self.player = (r, c);
                        Cell::PlayerOnTarget
                    }
                    _ => panic!("Err: Illegal char in map."),
                });
            }
            self.map.push(columns);
        }
    }

    pub fn get_size(&self) -> (usize, usize) {
        (self.map.len(), self.map[0].len())
    }

    pub fn is_pass(&self) -> bool {
        self.map
            .iter()
            .map(|ref r| r.iter().filter(|x| **x == Cell::Case).count())
            .fold(0, |sum, val| sum + val)
            == 0
    }

    pub fn get_boxes(&self) -> Vec<Coordinate> {
        vec![]
    }

    pub fn move_right(&mut self) {
        let (r, c) = (self.player.0, self.player.1);
        let (origin, updated) = update(&mut self.map, &((r, c), (r, c + 1), (r, c + 2)));
        if updated {
            self.player = (r, c + 1);
            self.history.push(((r, c), (r, c + 1), (r, c + 2), origin));
        }
    }

    pub fn move_left(&mut self) {
        let (r, c) = (self.player.0, self.player.1);
        let (origin, updated) = update(&mut self.map, &((r, c), (r, c - 1), (r, c - 2)));
        if updated {
            self.player = (r, c - 1);
            self.history.push(((r, c), (r, c - 1), (r, c - 2), origin));
        }
    }

    pub fn move_upward(&mut self) {
        let (r, c) = (self.player.0, self.player.1);
        let (origin, updated) = update(&mut self.map, &((r, c), (r - 1, c), (r - 2, c)));
        if updated {
            self.player = (r - 1, c);
            self.history.push(((r, c), (r - 1, c), (r - 2, c), origin));
        }
    }

    pub fn move_down(&mut self) {
        let (r, c) = (self.player.0, self.player.1);
        let (origin, updated) = update(&mut self.map, &((r, c), (r + 1, c), (r + 2, c)));
        if updated {
            self.player = (r + 1, c);
            self.history.push(((r, c), (r + 1, c), (r + 2, c), origin));
        }
    }

    pub fn undo(&mut self) {
        match self.history.pop() {
            Some((f, p, t, triple)) => {
                self.map[f.0][f.1] = triple.0;
                self.map[p.0][p.1] = triple.1;
                self.map[t.0][t.1] = triple.2;
                self.player = (f.0, f.1);
            }
            None => {}
        }
    }
}
