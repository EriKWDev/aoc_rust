#![allow(dead_code, unreachable_code, unused)]

use utils::*;

pub type Key = u64;
pub type Val = u32;
pub type Data = Vec<(Key, Vec<Val>)>;

pub fn parse_data(input: utils::Input) -> Data {
    input
        .lines()
        .map(|line| {
            let line = line.trim();

            let Some((a, b)) = line.split_once(": ") else {
                panic!()
            };
            let k = a.parse().unwrap();
            let mut vals = b
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect::<Vec<_>>();

            (k, vals)
        })
        .collect::<_>()
}

pub fn part_1(input: utils::Input) -> String {
    let data = parse_data(input);
    let mut result = 0;

    for (key, vals) in data {
        for mut i in 0..Key::pow(2, (vals.len() - 1) as _) {
            let mut it = vals.iter();
            let mut tot = *it.next().unwrap() as Key;
            for n in it {
                let n = *n as Key;

                if i % 2 == 0 {
                    tot += n;
                } else {
                    tot *= n;
                }
                i /= 2;
            }

            if tot == key {
                result += key;
                break;
            }
        }
    }

    format!("{}", result)
}

pub fn part_2(input: utils::Input) -> String {
    let data = parse_data(input);
    let mut result = 0;

    for (key, vals) in data {
        for mut i in 0..Key::pow(3, (vals.len() - 1) as _) {
            let mut it = vals.iter();
            let mut tot = *it.next().unwrap() as Key;
            for n in it {
                let n = *n as Key;

                if i % 3 == 0 {
                    tot += n;
                } else if i % 3 == 1 {
                    tot *= n;
                } else {
                    tot *= Key::pow(10, n.ilog10() + 1);
                    tot += n;
                }
                i /= 3;
            }

            if tot == key {
                result += key;
                break;
            }
        }
    }

    format!("{}", result)
}

pub fn run_1(date: utils::Date) {
    #[rustfmt::skip]
    let tests_1 = [
        (1, Some("3749")),
    ];

    let incorrect = ["850435815844"];

    let all_correct_1 = utils::test(part_1, date, 1, &tests_1);
    if all_correct_1 {
        let answer = utils::run(part_1, 1, date);
        if incorrect.contains(&answer.as_str()) {
            println!("\tCorrect: NO !!!!!!!!!\n\n");
        }
    }
}

pub fn run_2(date: utils::Date) {
    #[rustfmt::skip]
    let tests_2 = [
        (1, Some("11387")),
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
