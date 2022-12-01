#![allow(dead_code, unreachable_code, unused)]

use utils::*;

pub type Data = Vec<Vec<usize>>;
const DATE: utils::Date = (2022, 01);

pub fn parse_data(input: utils::Input) -> Data {
    let mut current_elf = vec![];

    let mut result = vec![];

    for line in input.lines().map(|line| line.unwrap()) {
        if line == "" {
            result.push(current_elf.clone());
            current_elf.clear();
        } else {
            let value = line.trim().parse::<usize>().unwrap();
            current_elf.push(value);
        }
    }

    result.push(current_elf.clone());

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
    new_vals.reverse();

    let (a, b, c) = (new_vals[0], new_vals[1], new_vals[2]);

    format!("{}", a + b + c)
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
