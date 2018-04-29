#![feature(try_from)]

extern crate pancurses;
extern crate rand;

use pancurses::Input::Character;
use pancurses::Window;
use rand::Rng;
use std::convert::TryFrom;

#[derive(Clone)]
struct Matrix(Vec<Vec<bool>>);

impl Matrix {
    fn new(width: usize, height: usize) -> Matrix {
        let width = width * 2 + 2;
        let height = height * 4 + 2;
        Matrix(vec![vec![false; width]; height])
    }

    fn randomize(&mut self) -> &mut Matrix {
        let mut rng = rand::thread_rng();
        let width = self.0[0].len();
        let height = self.0.len();

        for y in 1..height - 1 {
            for x in 1..width - 1 {
                self.0[y][x] = rng.gen();
            }
        }

        self
    }

    fn next(&self) -> Matrix {
        let width = self.0[0].len();
        let height = self.0.len();

        let mut next_state = self.clone();
        for y in 1..height - 1 {
            for x in 1..width - 1 {
                let mut livings = 0;
                if self.0[y - 1][x - 1] {
                    livings += 1;
                }
                if self.0[y - 1][x] {
                    livings += 1;
                }
                if self.0[y - 1][x + 1] {
                    livings += 1;
                }
                if self.0[y][x - 1] {
                    livings += 1;
                }
                if self.0[y][x + 1] {
                    livings += 1;
                }
                if self.0[y + 1][x - 1] {
                    livings += 1;
                }
                if self.0[y + 1][x] {
                    livings += 1;
                }
                if self.0[y + 1][x + 1] {
                    livings += 1;
                }

                if self.0[y][x] && livings != 2 && livings != 3 {
                    next_state.0[y][x] = false;
                } else if !self.0[y][x] && livings == 3 {
                    next_state.0[y][x] = true;
                }
            }
        }

        next_state
    }

    fn print(&self, window: &Window) {
        let width = self.0[0].len();
        let height = self.0.len();

        window.mv(0, 0);
        for y in (1..height - 1).step_by(4) {
            for x in (1..width - 1).step_by(2) {
                let mut ch = 0x2800u32;
                if self.0[y][x] {
                    ch |= 0x01;
                }
                if self.0[y + 1][x] {
                    ch |= 0x02;
                }
                if self.0[y + 2][x] {
                    ch |= 0x04;
                }
                if self.0[y][x + 1] {
                    ch |= 0x08;
                }
                if self.0[y + 1][x + 1] {
                    ch |= 0x10;
                }
                if self.0[y + 2][x + 1] {
                    ch |= 0x20;
                }
                if self.0[y + 3][x] {
                    ch |= 0x40;
                }
                if self.0[y + 3][x + 1] {
                    ch |= 0x80;
                }

                window.addstr(&char::try_from(ch).unwrap().to_string());
            }
        }
        window.refresh();
    }
}

fn main() {
    let window = pancurses::initscr();
    window.nodelay(true);

    let width = window.get_max_x() as usize;
    let height = window.get_max_y() as usize;

    let mut state = Matrix::new(width, height);
    state.randomize();

    loop {
        state = state.next();
        state.print(&window);

        match window.getch() {
            Some(Character('q')) => break,
            Some(Character('r')) => {
                state.randomize();
            }
            _ => (),
        }
    }

    pancurses::endwin();
}
