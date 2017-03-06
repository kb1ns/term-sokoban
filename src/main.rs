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
                Cell::PLAYER(i, j) => {
                    mvprintw(starty + i as i32, startx + j as i32, "i");
                }
                Cell::WALL(i, j) => {
                    mvprintw(starty + i as i32, startx + j as i32, "#");
                }
                Cell::BOX(i, j) => {
                    mvprintw(starty + i as i32, startx + j as i32, "o");
                }
                Cell::TARGET(i, j) => {
                    mvprintw(starty + i as i32, startx + j as i32, "x");
                }
                Cell::EMPTY(i, j) => {
                    mvprintw(starty + i as i32, startx + j as i32, " ");
                }
                Cell::PLAYER_ON_TARGET(i, j) => {
                    mvprintw(starty + i as i32, startx + j as i32, "I");
                }
                Cell::BOX_ON_TARGET(i, j) => {
                    mvprintw(starty + i as i32, startx + j as i32, "O");
                }
            };
        }
    }
    refresh();
}

fn main() {
    initscr();
    game_mode();
    let mut level = Level::new(1, Box::new(map::L2));
    level.reset();
    paint(&level);
    loop {
        if level.is_pass() {
            break;
        }
        let mut ch = getch();
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
                    "n" => {}
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
