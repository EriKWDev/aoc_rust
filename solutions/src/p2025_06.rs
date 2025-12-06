#![allow(dead_code, unreachable_code, unused)]

use utils::*;

pub type Data = (Vec<Vec<usize>>, Vec<char>);

pub fn parse_data(input: utils::Input) -> Data {
    let lines = input.lines().collect::<Vec<_>>();

    let mut result = vec![];

    for i in 0..lines.len().saturating_sub(1) {
        let line = &lines[i];

        result.push(
            line.split_whitespace()
                .map(|it| it.parse::<usize>().unwrap())
                .collect::<Vec<_>>(),
        );
    }

    let ops = lines
        .last()
        .unwrap()
        .split_whitespace()
        .map(|it| it.chars().next().unwrap())
        .collect();

    (result, ops)
}

pub fn parse_data_2(input: utils::Input) -> Data {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut result = vec![];
    let mut ops = vec![];

    let h = grid.len().saturating_sub(1);
    let last = &grid[h];

    let mut x = last.len();
    let mut current = vec![];
    loop {
        x -= 1;

        let mut n = 0;
        let mut exp = 0;
        for y in (0..h).rev() {
            if let Some(m) = grid[y][x].to_digit(10) {
                n += usize::pow(10, exp) * m as usize;
                exp += 1;
            }
        }
        current.push(n);

        let op = last[x];
        if !op.is_ascii_whitespace() {
            ops.push(op);
            result.push(core::mem::take(&mut current));
            if x != 0 {
                x -= 1;
            }
        }

        if x == 0 {
            break;
        }
    }

    result.reverse();
    ops.reverse();

    (result, ops)
}

pub fn part_1(input: utils::Input) -> String {
    let data = parse_data(input);
    let (numbers, ops) = data;

    if utils::is_test() {
        println!("{numbers:?}");
        println!("{ops:?}");
    }

    let mut result = 0;

    for x in 0..ops.len() {
        let op = ops[x];
        let mut accum = if op == '*' { 1 } else { 0 };

        for y in 0..numbers.len() {
            let row = &numbers[y];
            let value = row[x];

            accum = if op == '*' {
                accum * value
            } else {
                accum + value
            };
        }

        result += accum;
    }

    format!("{}", result)
}

pub fn part_2(input: utils::Input) -> String {
    let data = parse_data_2(input);
    let (numbers, ops) = data;

    if utils::is_test() {
        println!("{numbers:?}");
        println!("{ops:?}");
    }

    let mut result = 0;

    for x in 0..ops.len() {
        let op = ops[x];
        let mut accum = if op == '*' { 1 } else { 0 };
        for &value in &numbers[x] {
            accum = if op == '*' {
                accum * value
            } else {
                accum + value
            };
        }
        result += accum;
    }

    format!("{}", result)
}

pub fn run_1(date: utils::Date) {
    #[rustfmt::skip]
    let tests_1 = [
        (1, Some("4277556")),
    ];

    let all_correct_1 = utils::test(part_1, date, 1, &tests_1);
    if all_correct_1 {
        let answer = utils::run(part_1, 1, date);
    }
}

pub fn run_2(date: utils::Date) {
    #[rustfmt::skip]
    let tests_2 = [
        (1, Some("3263827")),
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
