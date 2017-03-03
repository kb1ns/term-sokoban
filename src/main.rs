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
    let starty = (LINES() - level.height as i32) / 2;
    let startx = (COLS() - level.width as i32) / 2;
    let mut l = 0;
    while l < level.map.len() {
        let mut c = 0;
        while c < level.map[l as usize].len() {
            let m: String = match *&level.map[l as usize][c as usize] {
                Cell::PLAYER(l, c) => 'i'.to_string(),
                Cell::WALL(l, c) => '#'.to_string(),
                Cell::BOX(l, c) => 'o'.to_string(),
                Cell::TARGET(l, c) => 'x'.to_string(),
                Cell::EMPTY(l, c) => ' '.to_string(),
                Cell::PLAYER_ON_TARGET(l, c) => 'I'.to_string(),
                Cell::BOX_ON_TARGET(l, c) => 'O'.to_string(),
            };
            c += 1;
            mvprintw(starty + l as i32, startx + c as i32, &m);
        }
        l += 1;
    }
    // wmove(stdscr(), 0, 0);
    refresh();
}

fn main() {
    initscr();
    game_mode();
    let mut level = Level::new(1, Box::new(map::l2));
    level.reset();
    paint(&level);
    loop {
        if level.is_pass() {
            break;
        }
        let mut ch = getch();
        match ch {
            KEY_LEFT => {
                level.lmove();
                paint(&level);
            }
            KEY_RIGHT => {
                level.rmove();
                paint(&level);
            }
            KEY_UP => {
                level.umove();
                paint(&level);
            }
            KEY_DOWN => {
                level.bmove();
                paint(&level);
            }
            0x3a => {
                ctl_mode();
                let mut input = "".to_string();
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
