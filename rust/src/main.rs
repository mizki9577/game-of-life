#![feature(step_by)]
#![feature(try_from)]

extern crate rand;
extern crate pancurses;

use std::convert::TryFrom;
use rand::Rng;
use pancurses::Window;
use pancurses::Input::Character;

type Matrix = Vec<Vec<bool>>;

trait State {
    fn new(width: usize, height: usize) -> Self;
    fn randomize(&mut self) -> &Self;
    fn next(&self) -> Self;
    fn print(&self, window: &Window);
}

impl State for Matrix {
    fn new(width: usize, height: usize) -> Self {
        let width = width * 2 + 2;
        let height = height * 4 + 2;
        vec![vec![false; width]; height]
    }

    fn randomize(&mut self) -> &Self {
        let mut rng = rand::thread_rng();
        let width = self[0].len();
        let height = self.len();

        for y in 1..height-1 {
            for x in 1..width-1 {
                self[y][x] = rng.gen();
            }
        }

        self
    }

    fn next(&self) -> Self {
        let width = self[0].len();
        let height = self.len();

        let mut next_state = self.clone();
        for y in 1..height-1 {
            for x in 1..width-1 {
                let mut livings = 0;
                    if self[y-1][x-1] { livings += 1; }
                    if self[y-1][x  ] { livings += 1; }
                    if self[y-1][x+1] { livings += 1; }
                    if self[y  ][x-1] { livings += 1; }
                    if self[y  ][x+1] { livings += 1; }
                    if self[y+1][x-1] { livings += 1; }
                    if self[y+1][x  ] { livings += 1; }
                    if self[y+1][x+1] { livings += 1; }

                if self[y][x] && livings != 2 && livings != 3 {
                    next_state[y][x] = false;
                } else if !self[y][x] && livings == 3 {
                    next_state[y][x] = true;
                }
            }
        }

        next_state
    }

    fn print(&self, window: &Window) {
        let width = self[0].len();
        let height = self.len();

        window.mv(0, 0);
        for y in (1..height-1).step_by(4) {
            for x in (1..width-1).step_by(2) {
                let mut ch = 0x2800;
                if self[y  ][x  ] { ch |= 0x01; }
                if self[y+1][x  ] { ch |= 0x02; }
                if self[y+2][x  ] { ch |= 0x04; }
                if self[y  ][x+1] { ch |= 0x08; }
                if self[y+1][x+1] { ch |= 0x10; }
                if self[y+2][x+1] { ch |= 0x20; }
                if self[y+3][x  ] { ch |= 0x40; }
                if self[y+3][x+1] { ch |= 0x80; }

                if let Ok(ch) = char::try_from(ch) {
                    window.addstr(ch.to_string().as_str());
                }
            }
        }
        window.refresh();
    }
}

fn main() {
    let window = pancurses::initscr();
    window.nodelay(true);

    let width  = window.get_max_x() as usize;
    let height = window.get_max_y() as usize;

    let mut prev_state: Matrix = State::new(width, height);
    prev_state.randomize();

    loop {
        let state = prev_state.next();
        state.print(&window);
        prev_state = state;

        match window.getch() {
            Some(Character('q')) => break,
            _                    => (),
        }
    }

    pancurses::endwin();
}
