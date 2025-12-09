#![allow(dead_code, unreachable_code, unused)]

use utils::*;

pub type Data = Vec<Vec<char>>;

pub fn parse_data(input: utils::Input) -> Data {
    input.lines().map(|line| line.chars().collect()).collect()
}

pub fn part_1(input: utils::Input) -> String {
    let data = parse_data(input);

    let mut s = 0;
    for i in 0..data[0].len() {
        if data[0][i] == 'S' {
            s = i;
            break;
        }
    }

    let mut result = 0;
    let mut visited = HashSet::<(usize, usize)>::default();

    let mut todo = vec![(0, s)];
    while let Some((y, x)) = todo.pop() {
        if !visited.insert((x, y)) {
            continue;
        }
        if let Some(c) = data.get(y).and_then(|row| row.get(x)) {
            match c {
                '.' | 'S' => {
                    todo.push(((y + 1, x)));
                }
                '^' => {
                    result += 1;
                    todo.push(((y, x + 1)));
                    todo.push(((y, x - 1)));
                }
                _ => unreachable!(),
            }
        }
    }

    format!("{}", result)
}

pub fn part_2(input: utils::Input) -> String {
    let data = parse_data(input);

    let mut s = 0;
    for i in 0..data[0].len() {
        if data[0][i] == 'S' {
            s = i;
            break;
        }
    }

    let mut result = 0;
    let mut visited = HashSet::<(usize, usize)>::default();

    let mut todo = vec![(0, s)];
    while let Some((y, x)) = todo.pop() {
        if !visited.insert((x, y)) {
            continue;
        }

        if let Some(c) = data.get(y).and_then(|row| row.get(x)) {
            match c {
                '.' | 'S' => {
                    todo.push(((y + 1, x)));
                }
                '^' => {
                    result += 2;
                    todo.push(((y, x + 1)));
                    todo.push(((y, x - 1)));
                }
                _ => unreachable!(),
            }
        }
    }

    format!("{}", result)
}

pub fn run_1(date: utils::Date) {
    #[rustfmt::skip]
    let tests_1 = [
        (1, Some("21")),
    ];

    let all_correct_1 = utils::test(part_1, date, 1, &tests_1);
    if all_correct_1 {
        let answer = utils::run(part_1, 1, date);
    }
}

pub fn run_2(date: utils::Date) {
    #[rustfmt::skip]
    let tests_2 = [
        (1, Some("40")),
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
