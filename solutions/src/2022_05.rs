#![allow(dead_code, unreachable_code, unused)]

use utils::*;

pub type Buffer = Vec<Vec<char>>;
pub type Commands = Vec<(usize, usize, usize)>;
pub type Data = (Buffer, Commands);

const DATE: utils::Date = (2022, 05);

pub fn parse_data(input: utils::Input) -> Data {
    // input.lines().map(|line| line.unwrap()).collect::<Vec<_>>()

    let mut flag = false;

    let mut lines_first = vec![];
    let mut lines_last = vec![];

    let mut max_len_first = 0;

    for line in input.lines().map(|line| line.unwrap()) {
        if line.is_empty() {
            flag = true;
            continue;
        }

        if flag {
            lines_last.push(line);
        } else {
            if line.contains(" 1 ") {
                continue;
            }

            max_len_first = max_len_first.max(line.len());
            lines_first.push(line);
        }
    }

    let height = lines_first.len();
    let mut buffer = vec![vec![]; max_len_first / 3];

    for (i, line) in lines_first.iter().rev().enumerate() {
        for (j, char) in line.chars().enumerate() {
            if char.is_ascii_alphabetic() {
                buffer[(j - 1) / 4].push(char);
            }
        }
    }

    let mut commands = vec![];

    for line in lines_last.iter() {
        let it = line.trim().split_ascii_whitespace().collect::<Vec<_>>();

        let a = it[1];
        let b = it[3];
        let c = it[5];

        let (a, b, c) = (
            a.trim().parse().unwrap(),
            b.trim().parse().unwrap(),
            c.trim().parse().unwrap(),
        );

        commands.push((a, b, c));
    }

    (buffer, commands)
}

pub fn part_1(input: utils::Input) -> String {
    let (mut buffer, commands) = parse_data(input);

    for (n, from, to) in commands {
        for _ in 0..n {
            let item = buffer[from - 1].pop().unwrap();
            buffer[to - 1].push(item);
        }
    }

    let mut result = String::new();
    for pile in &mut buffer {
        if let Some(it) = pile.pop() {
            result.push(it);
        }
    }

    result
}

pub fn part_2(input: utils::Input) -> String {
    let (mut buffer, commands) = parse_data(input);

    let mut buf = vec![];

    for (n, from, to) in commands {
        for _ in 0..n {
            let item = buffer[from - 1].pop().unwrap();
            buf.push(item);
        }

        for item in buf.drain(..).rev() {
            buffer[to - 1].push(item);
        }
    }

    let mut result = String::new();
    for pile in &mut buffer {
        if let Some(it) = pile.pop() {
            result.push(it);
        }
    }

    result
}

fn run_1() {
    #[rustfmt::skip]
    let tests_1 = [
        (1, Some("CMZ")),
    ];

    let all_correct_1 = utils::test(part_1, DATE, 1, &tests_1);
    if all_correct_1 {
        let answer = utils::run(part_1, 1, DATE);
    }
}

fn run_2() {
    #[rustfmt::skip]
    let tests_2 = [
        (1, Some("MCD")),
    ];

    let all_correct_2 = utils::test(part_2, DATE, 2, &tests_2);
    if all_correct_2 {
        let answer = utils::run(part_2, 2, DATE);
    }
}

fn main() {
    // run_1();
    run_2();
}
