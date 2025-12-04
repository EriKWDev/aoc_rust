#![allow(dead_code, unreachable_code, unused)]

use utils::*;

pub type Item = (usize, usize);
pub type Data = Vec<Item>;

pub fn parse_data(input: utils::Input) -> Data {
    input
        .lines()
        .map(|it| it.trim())
        .collect::<String>()
        .split(',')
        .map(|part| {
            let (a, b) = part.split_once("-").unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect()
}

pub fn part_1(input: utils::Input) -> String {
    let data = parse_data(input);
    let mut result = 0;

    let mut digits: Vec<usize> = vec![];

    for (start_id, end_id) in data {
        for id in start_id..=end_id {
            digits.clear();

            let mut it = id;
            while it > 0 {
                let d = it % 10;
                digits.push(d);
                it = it / 10;
            }
            digits.reverse();

            if digits.len() % 2 == 0 {
                let (a, b) = digits.as_slice().split_at(digits.len() / 2);
                if a == b {
                    result += id;
                }
            }
        }
    }

    format!("{}", result)
}

pub fn part_2(input: utils::Input) -> String {
    let data = parse_data(input);
    let mut result = 0;

    let mut digits: Vec<usize> = vec![];

    for (start_id, end_id) in data {
        'next_id: for id in start_id..=end_id {
            digits.clear();
            let mut it = id;
            while it > 0 {
                let d = it % 10;
                digits.push(d);
                it = it / 10;
            }
            digits.reverse();

            for i in 2..10 {
                if digits.len() % i == 0 {
                    let n = digits.len() / i;
                    let a = &digits[0..n];
                    let mut all = true;
                    let mut any = false;
                    for j in 1..i {
                        any = true;
                        let b = &digits[j * n..(j + 1) * n];
                        if a != b {
                            all = false;
                            break;
                        }
                    }
                    if all && any {
                        result += id;
                        continue 'next_id;
                    }
                }
            }
        }
    }

    format!("{}", result)
}

pub fn run_1(date: utils::Date) {
    #[rustfmt::skip]
    let tests_1 = [
        (1, Some("1227775554")),
    ];

    let all_correct_1 = utils::test(part_1, date, 1, &tests_1);
    if all_correct_1 {
        let answer = utils::run(part_1, 1, date);
    }
}

pub fn run_2(date: utils::Date) {
    #[rustfmt::skip]
    let tests_2 = [
        (1, Some("4174379265")),
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
