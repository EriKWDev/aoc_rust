#![allow(dead_code, unreachable_code, unused)]

use utils::*;

pub type Data = Vec<(i32, i32)>;

pub fn parse_data(input: utils::Input) -> Data {
    input
        .lines()
        .map(|line| {
            let mut c = line.chars();
            let dir = c.next().unwrap();
            let dir: i32 = if dir == 'L' { -1 } else { 1 };

            let num = c.collect::<String>().parse::<i32>().unwrap();

            (dir, num)
        })
        .collect::<Vec<_>>()
}

pub fn part_1(input: utils::Input) -> String {
    let data = parse_data(input);

    let start: i32 = 50;

    let mut dial: i32 = start;
    let mut result = 0;
    for (dir, num) in data {
        let a = dial;
        dial += dir * num;
        dial = dial.rem_euclid(100);
        if dial == 0 {
            result += 1;
        }
    }

    format!("{}", result)
}

pub fn part_2(input: utils::Input) -> String {
    let data = parse_data(input);

    let start: i32 = 50;

    let mut dial: i32 = start;
    let mut result = 0;
    for (dir, num) in data {
        let a = dial;
        for _ in 0..num {
            dial += dir;
            dial = dial.rem_euclid(100);

            if dial == 0 {
                result += 1;
            }
        }
    }

    format!("{}", result)
}

pub fn run_1(date: utils::Date) {
    #[rustfmt::skip]
    let tests_1 = [
        (1, Some("3")),
    ];

    let all_correct_1 = utils::test(part_1, date, 1, &tests_1);
    if all_correct_1 {
        let answer = utils::run(part_1, 1, date);
    }
}

pub fn run_2(date: utils::Date) {
    #[rustfmt::skip]
    let tests_2 = [
        (1, Some("6")),
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
