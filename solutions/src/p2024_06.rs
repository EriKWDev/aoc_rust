#![allow(dead_code, unreachable_code, unused)]

use utils::*;

pub type Data = Vec<Vec<u8>>;

const FREE: u8 = 0;
const WALL: u8 = 1;

const DOWN: u8 = 2;
const UP: u8 = 3;
const RIGHT: u8 = 4;
const LEFT: u8 = 5;

pub const fn rotate_right(d: u8) -> u8 {
    match d {
        UP => RIGHT,
        RIGHT => DOWN,
        DOWN => LEFT,
        LEFT => UP,
        _ => unreachable!(),
    }
}

pub const fn move_dir(d: u8) -> (i32, i32) {
    match d {
        UP => (0, -1),
        RIGHT => (1, 0),
        DOWN => (0, 1),
        LEFT => (-1, 0),
        _ => unreachable!(),
    }
}

#[inline]
pub const fn is_guard(d: u8) -> bool {
    d != WALL && d != FREE
}

pub fn parse_data(input: utils::Input) -> Data {
    input
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| match c {
                    '.' => 0,
                    '#' => 1,

                    'v' => DOWN,
                    '^' => UP,
                    '>' => RIGHT,
                    '<' => LEFT,

                    _ => panic!(),
                })
                .collect()
        })
        .collect::<Vec<_>>()
}

pub fn part_1(input: utils::Input) -> String {
    let mut data = parse_data(input);

    let (h, w) = (data.len(), data[0].len());
    let mut guard = (0, 0);
    for y in 0..h {
        for x in 0..w {
            if is_guard(data[y][x]) {
                guard = (x, y);
                break;
            }
        }
    }

    let mut visited = HashSet::new();

    loop {
        visited.insert(guard);

        let (x, y) = guard;
        let c = data[y][x];
        let (dx, dy) = move_dir(c);
        let (nx, ny) = (x as i32 + dx, y as i32 + dy);
        if nx < 0 || ny < 0 || nx >= w as i32 || ny >= h as i32 {
            break;
        }
        let (nx, ny) = (nx as usize, ny as usize);

        match data[ny][nx] {
            FREE => {
                data[ny][nx] = c;
                data[y][x] = FREE;
                guard = (nx, ny)
            }

            WALL => {
                data[y][x] = rotate_right(c);
            }

            _ => unreachable!(),
        }
    }

    format!("{}", visited.len())
}

pub fn part_2(input: utils::Input) -> String {
    let data = parse_data(input);
    let mut result = 0;

    format!("{}", result)
}

pub fn run_1(date: utils::Date) {
    #[rustfmt::skip]
    let tests_1 = [
        (1, Some("41")),
    ];

    let all_correct_1 = utils::test(part_1, date, 1, &tests_1);
    if all_correct_1 {
        let answer = utils::run(part_1, 1, date);
    }
}

pub fn run_2(date: utils::Date) {
    #[rustfmt::skip]
    let tests_2 = [
        (1, Some("6")),
    ];

    let all_correct_2 = utils::test(part_2, date, 2, &tests_2);
    if all_correct_2 {
        let answer = utils::run(part_2, 2, date);
    }
}

pub fn date() -> utils::Date {
    utils::date_from_file_name(file!())
}

fn main() {
    let date = date();
    run_1(date);
    run_2(date);
}
