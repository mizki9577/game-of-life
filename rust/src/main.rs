use pancurses::{Input, Window};
use rand::Rng as _;
use std::convert::TryFrom;

fn main() {
    let window = pancurses::initscr();
    window.nodelay(true);
    pancurses::curs_set(0);

    let (mut width, mut height, mut matrix) = init_matrix(&window);

    loop {
        match window.getch() {
            Some(Input::KeyResize) => {
                pancurses::resize_term(0, 0);
                let t = init_matrix(&window);
                width = t.0;
                height = t.1;
                matrix = t.2;
            }
            Some(Input::Character(..)) => break,
            Some(..) => (),
            None => (),
        }

        for y in (1..height - 1).step_by(4) {
            let mut line = String::new();
            for x in (1..width - 1).step_by(2) {
                let mut ch = 0x2800u32;
                if matrix[y][x] {
                    ch |= 0x01;
                }
                if matrix[y + 1][x] {
                    ch |= 0x02;
                }
                if matrix[y + 2][x] {
                    ch |= 0x04;
                }
                if matrix[y][x + 1] {
                    ch |= 0x08;
                }
                if matrix[y + 1][x + 1] {
                    ch |= 0x10;
                }
                if matrix[y + 2][x + 1] {
                    ch |= 0x20;
                }
                if matrix[y + 3][x] {
                    ch |= 0x40;
                }
                if matrix[y + 3][x + 1] {
                    ch |= 0x80;
                }

                line.push(char::try_from(ch).unwrap());
            }
            window.mvaddstr((y as i32 - 1) / 4, 0, &line);
        }
        window.refresh();

        let mut next = matrix.clone();
        for y in 1..height - 1 {
            for x in 1..width - 1 {
                let mut livings = 0;
                if matrix[y - 1][x - 1] {
                    livings += 1;
                }
                if matrix[y - 1][x] {
                    livings += 1;
                }
                if matrix[y - 1][x + 1] {
                    livings += 1;
                }
                if matrix[y][x - 1] {
                    livings += 1;
                }
                if matrix[y][x + 1] {
                    livings += 1;
                }
                if matrix[y + 1][x - 1] {
                    livings += 1;
                }
                if matrix[y + 1][x] {
                    livings += 1;
                }
                if matrix[y + 1][x + 1] {
                    livings += 1;
                }

                if matrix[y][x] && livings != 2 && livings != 3 {
                    next[y][x] = false;
                } else if !matrix[y][x] && livings == 3 {
                    next[y][x] = true;
                }
            }
        }

        matrix = next;
    }

    pancurses::endwin();
}

fn init_matrix(window: &Window) -> (usize, usize, Vec<Vec<bool>>) {
    let width = window.get_max_x() as usize * 2 + 2;
    let height = window.get_max_y() as usize * 4 + 2;
    let mut matrix = vec![vec![false; width]; height];
    let mut rng = rand::thread_rng();
    for row in matrix.iter_mut().skip(1).take(height - 2) {
        for cell in row.iter_mut().skip(1).take(width - 2) {
            *cell = rng.gen();
        }
    }
    (width, height, matrix)
}
