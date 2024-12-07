#![allow(dead_code, unreachable_code, unused)]

use utils::*;

pub type Data = Vec<Vec<char>>;

pub fn parse_data(input: utils::Input) -> Data {
    input
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect()
}

pub fn part_1(input: utils::Input) -> String {
    let data = parse_data(input);

    let them = ['M', 'A', 'S'];

    let (w, h) = (data[0].len(), data.len());
    let deltas = [
        (1, 1),
        (1, 0),
        (1, -1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, 1),
        (0, -1),
    ];

    let mut result = 0;
    for y in 0..h {
        for x in 0..w {
            if data[y][x] != 'X' {
                continue;
            }

            'delta: for (dx, dy) in deltas {
                for i in 1..=3 {
                    let (nx, ny) = (x as i32 + dx * i, y as i32 + dy * i);
                    if nx < 0 || ny < 0 || ny >= h as i32 || nx >= w as i32 {
                        continue 'delta;
                    }
                    let (nx, ny) = (nx as usize, ny as usize);
                    if data[ny][nx] != them[(i - 1) as usize] {
                        continue 'delta;
                    }
                }

                result += 1;
            }
        }
    }

    format!("{}", result)
}

pub fn part_2(input: utils::Input) -> String {
    let data = parse_data(input);

    let mut result = 0;

    for y in 1..data.len() - 1 {
        for x in 1..data[y].len() - 1 {
            if data[y][x] != 'A' {
                continue;
            }

            let mut num = 0;
            for (dx, dy) in [(-1, -1), (1, -1)] {
                let (nx, ny) = ((x as i32 + dx) as usize, (y as i32 + dy) as usize);
                let (nxx, nyy) = ((x as i32 - dx) as usize, (y as i32 - dy) as usize);
                let (a, b) = (data[ny][nx], data[nyy][nxx]);
                if (a == 'M' && b == 'S') || (a == 'S' && b == 'M') {
                    num += 1;
                }
            }

            if num == 2 {
                result += 1;
            }
        }
    }

    format!("{}", result)
}

pub fn run_1(date: utils::Date) {
    #[rustfmt::skip]
    let tests_1 = [
        (1, Some("18")),
    ];

    let all_correct_1 = utils::test(part_1, date, 1, &tests_1);
    if all_correct_1 {
        let answer = utils::run(part_1, 1, date);
    }
}

pub fn run_2(date: utils::Date) {
    #[rustfmt::skip]
    let tests_2 = [
        (1, Some("9")),
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
