#![allow(dead_code, unreachable_code, unused)]

use utils::*;

pub type Data = ();
pub const DATE: utils::Date = (2022, 06);

pub fn part_1(input: utils::Input) -> String {
    let mut buf = HashSet::<char>::new();
    const N: usize = 4;

    let chars = input.lines().next().unwrap().chars().collect::<Vec<_>>();
    let (i, _window) = chars
        .windows(N)
        .enumerate()
        .find(|(i, window)| {
            buf.extend(*window);
            let result = buf.len() == N;
            buf.clear();

            result
        })
        .unwrap();

    format!("{}", i + N)
}

pub fn part_2(input: utils::Input) -> String {
    let mut buf = HashSet::<char>::new();
    const N: usize = 14;

    let chars = input.lines().next().unwrap().chars().collect::<Vec<_>>();
    let (i, _window) = chars
        .windows(N)
        .enumerate()
        .find(|(i, window)| {
            buf.extend(*window);
            let result = buf.len() == N;
            buf.clear();

            result
        })
        .unwrap();

    format!("{}", i + N)
}

fn run_1() {
    #[rustfmt::skip]
    let tests_1 = [
        (1, Some("7")),
        (2, Some("5")),
        (3, Some("6")),
        (4, Some("10")),
        (5, Some("11")),
    ];

    let all_correct_1 = utils::test(part_1, DATE, 1, &tests_1);
    if all_correct_1 {
        let answer = utils::run(part_1, 1, DATE);
    }
}

fn run_2() {
    #[rustfmt::skip]
    let tests_2 = [
        (1, Some("19")),
        (2, Some("23")),
        (3, Some("23")),
        (4, Some("29")),
        (5, Some("26")),
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
