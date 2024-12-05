#![allow(dead_code, unreachable_code, unused)]

use utils::*;

pub type Data = Vec<String>;

pub fn parse_data(input: utils::Input) -> Data {
    input
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<_>>()
}

pub fn part_1(input: utils::Input) -> String {
    let data = parse_data(input);
    let mut result = 0;

    format!("{}", result)
}

pub fn part_2(input: utils::Input) -> String {
    let data = parse_data(input);
    let mut result = 0;

    format!("{}", result)
}

fn run_1(date: utils::Date) {
    #[rustfmt::skip]
    let tests_1 = [
        (1, Some("ANWER_PART_1")),
    ];

    let all_correct_1 = utils::test(part_1, date, 1, &tests_1);
    if all_correct_1 {
        let answer = utils::run(part_1, 1, date);
    }
}

fn run_2(date: utils::Date) {
    #[rustfmt::skip]
    let tests_2 = [
        (1, Some("ANSWER_PART_2")),
    ];

    let all_correct_2 = utils::test(part_2, date, 2, &tests_2);
    if all_correct_2 {
        let answer = utils::run(part_2, 2, date);
    }
}

fn main() {
    let date = utils::date_from_file_name(file!());
    run_1(date);
    // run_2(date);
}
