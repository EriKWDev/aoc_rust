#![allow(dead_code, unreachable_code, unused)]

use utils::*;

pub type Data = Vec<(usize, usize)>;
pub const DATE: utils::Date = (2023, 6);

pub fn parse_data(input: utils::Input) -> Data {
    let mut lines = input.lines();

    let a = lines.next().unwrap();
    let b = lines.next().unwrap();

    a.split_whitespace()
        .skip(1)
        .map(|n| n.parse().unwrap())
        .zip(b.split_whitespace().skip(1).map(|n| n.parse().unwrap()))
        .collect()
}

pub fn part_1(input: utils::Input) -> String {
    let data = parse_data(input);
    let mut result = 1;

    for (time, record_distance) in data {
        let mut wins = 0;

        for speed in 0..=time {
            let remaining_t = time - speed;
            let start_t = time - remaining_t;
            let distance_travelled = remaining_t * speed;

            if distance_travelled > record_distance {
                wins += 1;
            }
        }

        result *= wins;
    }

    format!("{}", result)
}

pub fn part_2(input: utils::Input) -> String {
    let mut lines = input.lines();

    #[rustfmt::skip]
    let time: u128 = lines.next().unwrap()
        .split_whitespace().skip(1)
        .collect::<String>()
        .parse().unwrap();

    #[rustfmt::skip]
    let record_distance: u128 = lines.next().unwrap()
        .split_whitespace().skip(1)
        .collect::<String>()
        .parse().unwrap();

    let mut result = 0;

    for speed in 0..=time {
        let remaining_t = time - speed;
        let start_t = time - remaining_t;
        let distance_travelled = remaining_t * speed;

        if distance_travelled > record_distance {
            result += 1;
        }
    }

    format!("{}", result)
}

fn run_1() {
    #[rustfmt::skip]
    let tests_1 = [
        (1, Some("288")),
    ];

    let all_correct_1 = utils::test(part_1, DATE, 1, &tests_1);
    if all_correct_1 {
        let answer = utils::run(part_1, 1, DATE);
    }
}

fn run_2() {
    #[rustfmt::skip]
    let tests_2 = [
        (1, Some("71503")),
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
