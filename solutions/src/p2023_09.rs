#![allow(dead_code, unreachable_code, unused)]

use utils::*;

pub type Data = Vec<Vec<isize>>;
pub const DATE: utils::Date = (2023, 09);

pub fn parse_data(input: utils::Input) -> Data {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect::<Vec<_>>()
}

pub fn part_1(input: utils::Input) -> String {
    let data = parse_data(input);
    let mut result = 0;

    for round in data {
        let mut them = vec![round.clone()];
        let mut q = Some(round);

        while let Some(current) = q.take() {
            let mut any_not_z = false;

            let new = current
                .windows(2)
                .map(|ab| {
                    let &[a, b] = ab else { panic!() };
                    let d = b - a;

                    if d != 0 {
                        any_not_z = true;
                    }

                    d
                })
                .collect::<Vec<_>>();

            them.push(new.clone());
            if any_not_z {
                q.replace(new);
            }
        }

        let mut n = 0;
        for i in (1..them.len()).rev() {
            let r = &mut them[i];
            r.push(0); // yuck
            let m = r[r.len() - 2];
            n += m;
        }

        n += them[0].last().unwrap();

        result += n;
    }

    format!("{}", result)
}

pub fn part_2(input: utils::Input) -> String {
    let data = parse_data(input);
    let mut result = 0;

    for round in data {
        let mut them = vec![round.clone()];
        let mut q = Some(round);

        while let Some(mut current) = q.take() {
            let mut any_not_z = false;

            let mut new = current
                .windows(2)
                .map(|ab| {
                    let &[a, b] = ab else { panic!() };
                    let d = a - b;

                    if d != 0 {
                        any_not_z = true;
                    }

                    d
                })
                .collect::<Vec<_>>();

            them.push(new.clone());
            if any_not_z {
                q.replace(new);
            }
        }

        let mut n = 0;
        for i in (1..them.len()).rev() {
            let r = &mut them[i];
            let m = r[0];
            n += m;
        }

        n += them[0][0];

        result += n;
    }

    format!("{}", result)
}

fn run_1() {
    #[rustfmt::skip]
    let tests_1 = [
        (1, Some("114")),
    ];

    let all_correct_1 = utils::test(part_1, DATE, 1, &tests_1);
    if all_correct_1 {
        let answer = utils::run(part_1, 1, DATE);
    }
}

fn run_2() {
    #[rustfmt::skip]
    let tests_2 = [
        (1, Some("2")),
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
