#![allow(dead_code, unreachable_code, unused)]

use utils::*;

pub type Data = ();
const DATE: utils::Date = (2022, 02);

pub fn parse_data(input: utils::Input) -> Data {
    ()
}

pub fn part_1(input: utils::Input) -> String {
    let data = parse_data(input);

    format!("{}", 0)
}

pub fn part_2(input: utils::Input) -> String {
    let data = parse_data(input);

    format!("{}", 0)
}

fn run_1() {
    #[rustfmt::skip]
    let tests_1 = [
        (1, Some("1")),
    ];

    let all_correct_1 = utils::test(part_1, DATE, 1, &tests_1);
    if all_correct_1 {
        let answer = utils::run(part_1, 1, DATE);
    }
}

fn run_2() {
    #[rustfmt::skip]
    let tests_2 = [
        (1, Some("1")),
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
