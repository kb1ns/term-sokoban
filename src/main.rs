extern crate ncurses;

mod sokoban;
mod map;

use ncurses::*;
use sokoban::*;


fn game_mode() {
    raw();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    keypad(stdscr(), true);
    noecho();
}

fn ctl_mode() {
    cbreak();
    wmove(stdscr(), LINES(), 0);
    mvprintw(LINES() - 1, 0, ":");
    curs_set(CURSOR_VISIBILITY::CURSOR_VISIBLE);
    echo();
}

fn paint(level: &Level) {
    let starty = (LINES() - level.height() as i32) / 2;
    let startx = (COLS() - level.width() as i32) / 2;
    for l in level.map.as_slice() {
        for c in l {
            match *c {
                Cell::Player(i, j) => {
                    mvaddch(starty + i as i32, startx + j as i32, 'i' as u32);
                }
                Cell::Wall(i, j) => {
                    attron(A_DIM());
                    mvaddch(starty + i as i32, startx + j as i32, ACS_CKBOARD());
                    attroff(A_DIM());
                }
                Cell::Box(i, j) => {
                    mvaddch(starty + i as i32, startx + j as i32, 'o' as u32);
                }
                Cell::Target(i, j) => {
                    attron(A_DIM());
                    mvaddch(starty + i as i32, startx + j as i32, 'x' as u32);
                    attroff(A_DIM());
                }
                Cell::Empty(i, j) => {
                    mvaddch(starty + i as i32, startx + j as i32, ' ' as u32);
                }
                Cell::PlayerOnTarget(i, j) => {
                    mvaddch(starty + i as i32, startx + j as i32, 'I' as u32);
                }
                Cell::BoxOnTarget(i, j) => {
                    color_set(COLOR_RED);
                    attron(A_BOLD() | A_BLINK() | COLOR_PAIR(COLOR_RED));
                    mvaddch(starty + i as i32, startx + j as i32, 'O' as u32);
                    attroff(A_BOLD() | A_BLINK() | COLOR_PAIR(COLOR_RED));
                }
            };
        }
    }
    let mut s = "Level ".to_string();
    s.push_str(&level.index.to_string());
    mvprintw(1, (COLS() - 8) / 2 as i32, &s);
    refresh();
}

fn next(i: usize) -> (usize, Level) {
    let level_count = map::MAPS.len();
    let mut index = i + 1;
    if index >= level_count {
        index = 0;
    }
    (index, Level::new(index + 1, Box::new(map::MAPS[index])))
}


fn main() {
    initscr();
    game_mode();
    let mut index: usize = 0;
    let mut level = Level::new(1, Box::new(map::MAPS[0]));
    level.reset();
    paint(&level);
    loop {
        if level.is_pass() {
            let t = next(index);
            index = t.0;
            level = t.1;
            level.reset();
            paint(&level);
        }
        let ch = getch();
        match ch {
            KEY_LEFT => {
                level.move_left();
                paint(&level);
            }
            KEY_RIGHT => {
                level.move_right();
                paint(&level);
            }
            KEY_UP => {
                level.move_upward();
                paint(&level);
            }
            KEY_DOWN => {
                level.move_down();
                paint(&level);
            }
            KEY_F1 => {
                level.revert();
                paint(&level);
            }
            0x3a => {
                ctl_mode();
                let mut input = String::new();
                getstr(&mut input);
                match &*input {
                    "q" => break,
                    "n" => {
                        let t = next(index);
                        index = t.0;
                        level = t.1;
                        level.reset();
                        paint(&level);
                    }
                    "r" => {
                        clear();
                        level.reset();
                        paint(&level);
                    }
                    _ => {}
                }
                let mut c = 0;
                while c < COLS() {
                    mvprintw(LINES() - 1, c, " ");
                    c += 1;
                }
                game_mode();
            }
            _ => {}
        }
    }
    endwin();
}
