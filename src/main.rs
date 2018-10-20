extern crate ncurses;

mod engine;
mod map;
mod resolver;

use engine::*;
use ncurses::*;
use resolver::*;

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

fn paint(level: usize, scene: &Scene) {
    let starty = (LINES() - scene.get_size().0 as i32) / 2;
    let startx = (COLS() - scene.get_size().1 as i32) / 2;
    for (i, row) in scene.map.iter().enumerate() {
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
    let title = format!("Level {}", level % map::MAPS.len());
    mvprintw(1, (COLS() - 8) / 2 as i32, &title);
    refresh();
}


fn main() {
    initscr();
    game_mode();
    let mut level = 0;
    let mut scene = Scene::init();
    scene.load(&map::MAPS[level % map::MAPS.len()]);
    paint(level, &scene);
    loop {
        if scene.is_pass() {
            level = level + 1;
            scene.load(&map::MAPS[level % map::MAPS.len()]);
            paint(level, &scene);
        }
        let ch = getch();
        match ch {
            KEY_LEFT | 0x68 => {
                scene.move_left();
                paint(level, &scene);
            }
            KEY_RIGHT | 0x6c => {
                scene.move_right();
                paint(level, &scene);
            }
            KEY_UP | 0x6b => {
                scene.move_upward();
                paint(level, &scene);
            }
            KEY_DOWN | 0x6a => {
                scene.move_down();
                paint(level, &scene);
            }
            // u
            0x75 => {
                clear();
                scene.undo();
                paint(level, &scene);
            }
            // r
            0x72 => {
                clear();
                scene.load(&map::MAPS[level % map::MAPS.len()]);
                paint(level, &scene);
            }
            // n
            0x6e => {
                clear();
                level = level + 1;
                scene.load(&map::MAPS[level % map::MAPS.len()]);
                paint(level, &scene);
            }
            // q
            0x71 => break,
            _ => {}
        }
    }
    endwin();
}
