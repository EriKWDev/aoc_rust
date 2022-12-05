#![allow(dead_code, unreachable_code, unused)]

use utils::*;

pub type Buffer = Vec<Vec<char>>;
pub type Commands = Vec<(usize, usize, usize)>;
pub type Data = (Buffer, Commands);

pub const DATE: utils::Date = (2022, 05);

pub fn parse_data(input: utils::Input) -> Data {
    let mut max_len_first = 0;
    let mut is_parsing_first_part = true;

    let (lines_piles, lines_commands): (Vec<_>, Vec<_>) = input.lines().partition(|line| {
        if line.is_empty() {
            is_parsing_first_part = false;
        }

        if is_parsing_first_part {
            max_len_first = max_len_first.max(line.len());
        }

        is_parsing_first_part
    });

    let mut buffer = vec![vec![]; max_len_first / 3];
    lines_piles.iter().rev().skip(1).for_each(|line| {
        line.chars()
            .enumerate()
            .filter(|(_, char)| char.is_ascii_alphabetic())
            .for_each(|(j, char)| {
                buffer[(j - 1) / 4].push(char);
            });
    });

    let commands = lines_commands
        .iter()
        .skip(1)
        .map(|line| {
            let mut it = line.split_ascii_whitespace().skip(1).step_by(2);

            (
                it.next().unwrap().parse().unwrap(),
                it.next().unwrap().parse().unwrap(),
                it.next().unwrap().parse().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    (buffer, commands)
}

pub fn part_1(input: utils::Input) -> String {
    let (mut buffer, mut commands) = parse_data(input);
    let mut buf = vec![];

    commands.drain(..).for_each(|(n, from, to)| {
        let items = (0..n).into_iter().filter_map(|_| buffer[from - 1].pop());
        buf.extend(items);

        buffer[to - 1].extend(buf.drain(..));
    });

    let top_crates = buffer
        .iter_mut()
        .filter_map(|pile| pile.pop())
        .collect::<String>();

    top_crates
}

pub fn part_2(input: utils::Input) -> String {
    let (mut buffer, mut commands) = parse_data(input);
    let mut buf = vec![];

    commands.drain(..).for_each(|(n, from, to)| {
        let items = (0..n).into_iter().filter_map(|_| buffer[from - 1].pop());
        buf.extend(items);

        buffer[to - 1].extend(buf.drain(..).rev());
    });

    let top_crates = buffer
        .iter_mut()
        .filter_map(|pile| pile.pop())
        .collect::<String>();

    top_crates
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
    run_1();
    run_2();
}
