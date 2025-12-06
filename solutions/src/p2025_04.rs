#![allow(dead_code, unreachable_code, unused)]

use utils::*;

pub type Data = Vec<Vec<char>>;

pub fn parse_data(input: utils::Input) -> Data {
    input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<_>>()
}

pub fn part_1(input: utils::Input) -> String {
    let mut data = parse_data(input);
    data.reverse();

    let mut result = 0;

    for y in (0..data.len() as i32).rev() {
        let it = &data[y as usize];
        for x in 0..it.len() as i32 {
            let v = it[x as usize];
            if v == '@' {
                let mut n = 0;

                for dx in -1..=1 {
                    for dy in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }

                        let x = x + dx;
                        let y = y + dy;

                        if x < 0 || y < 0 || y >= data.len() as i32 || x >= it.len() as i32 {
                            continue;
                        }

                        let x = x as usize;
                        let y = y as usize;

                        if let Some(&v) = data.get(y).and_then(|it| it.get(x)) {
                            if v == '@' {
                                n += 1;
                            }
                        }
                    }
                }

                if n < 4 {
                    result += 1;
                }
            }
        }
    }

    format!("{}", result)
}

pub fn part_2(input: utils::Input) -> String {
    let mut data = parse_data(input);
    data.reverse();

    let mut result = 0;

    let mut to_remove = vec![];

    loop {
        to_remove.clear();

        for y in (0..data.len() as i32).rev() {
            let it = &data[y as usize];
            for x in 0..it.len() as i32 {
                let v = it[x as usize];
                if v == '@' {
                    let mut n = 0;

                    for dx in -1..=1 {
                        for dy in -1..=1 {
                            if dx == 0 && dy == 0 {
                                continue;
                            }

                            let x = x + dx;
                            let y = y + dy;

                            if x < 0 || y < 0 || y >= data.len() as i32 || x >= it.len() as i32 {
                                continue;
                            }

                            let x = x as usize;
                            let y = y as usize;

                            if let Some(&v) = data.get(y).and_then(|it| it.get(x)) {
                                if v == '@' {
                                    n += 1;
                                }
                            }
                        }
                    }

                    if n < 4 {
                        to_remove.push((x as usize, y as usize));
                    }
                }
            }
        }

        let round = to_remove.len();
        result += round;

        for (x, y) in to_remove.drain(..) {
            data[y][x] = '.';
        }

        if round == 0 {
            break;
        }
    }

    format!("{}", result)
}

pub fn run_1(date: utils::Date) {
    #[rustfmt::skip]
    let tests_1 = [
        (1, Some("13")),
    ];

    let all_correct_1 = utils::test(part_1, date, 1, &tests_1);
    if all_correct_1 {
        let answer = utils::run(part_1, 1, date);
        let n = answer.parse::<usize>().unwrap();
        assert!(n < 1498);
    }
}

pub fn run_2(date: utils::Date) {
    #[rustfmt::skip]
    let tests_2 = [
        (1, Some("43")),
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
