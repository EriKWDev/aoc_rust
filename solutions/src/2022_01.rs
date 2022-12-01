#![allow(dead_code, unreachable_code, unused)]

use utils::*;

pub type Data = Vec<Vec<usize>>;
const DATE: utils::Date = (2022, 01);

pub fn parse_data(input: utils::Input) -> Data {
    let mut current_elf = vec![];

    let mut result = input
        .lines()
        .map(|line| line.unwrap())
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() {
                Some(current_elf.drain(..).collect())
            } else {
                let value = line.parse::<usize>().unwrap();
                current_elf.push(value);

                None
            }
        })
        .collect::<Vec<_>>();

    if !current_elf.is_empty() {
        result.push(current_elf);
    }

    result
}

pub fn part_1(input: utils::Input) -> String {
    let data = parse_data(input);

    let max = data
        .iter()
        .map(|elf| elf.iter().sum::<usize>())
        .max()
        .unwrap_or(0);

    format!("{}", max)
}

pub fn part_2(input: utils::Input) -> String {
    let data = parse_data(input);

    let mut new_vals = data
        .iter()
        .map(|elf| elf.iter().sum::<usize>())
        .collect::<Vec<_>>();

    new_vals.sort();

    let answer = new_vals.iter().rev().take(3).sum::<usize>();

    format!("{}", answer)
}

fn run_1() {
    #[rustfmt::skip]
    let tests_1 = [
        (1, Some("24000")),
    ];

    let all_correct_1 = utils::test(part_1, DATE, 1, &tests_1);
    if all_correct_1 {
        let answer = utils::run(part_1, 1, DATE);
    }
}

fn run_2() {
    #[rustfmt::skip]
    let tests_2 = [
        (2, Some("45000")),
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
