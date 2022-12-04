#![allow(dead_code, unreachable_code, unused)]

use utils::*;

const DATE: (usize, usize) = (2022, 04);
pub type Data = Vec<((usize, usize), (usize, usize))>;

pub fn parse_data(input: utils::Input) -> Data {
    input
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let mut split = line.split(",");

            let (a, b) = (split.next().unwrap(), split.next().unwrap());

            let mut a = a.split("-");
            let mut b = b.split("-");

            let a = (
                a.next().unwrap().parse().unwrap(),
                a.next().unwrap().parse().unwrap(),
            );

            let b = (
                b.next().unwrap().parse().unwrap(),
                b.next().unwrap().parse().unwrap(),
            );

            (a, b)
        })
        .collect::<Vec<_>>()
}

#[inline]
pub fn fully_contains(a: (usize, usize), b: (usize, usize)) -> bool {
    let (a1, a2) = a;
    let (b1, b2) = b;

    b1 >= a1 && b2 <= a2
}

#[inline]
pub fn overlaps(a: (usize, usize), b: (usize, usize)) -> bool {
    let (a1, a2) = a;
    let (b1, b2) = b;

    a1 <= b2 && b1 <= a2
}

pub fn part_1(input: utils::Input) -> String {
    let data = parse_data(input);

    let result = data
        .iter()
        .filter(|(a, b)| {
            let (a, b) = (*a, *b);
            fully_contains(a, b) || fully_contains(b, a)
        })
        .count();

    format!("{}", result)
}

pub fn part_2(input: utils::Input) -> String {
    let data = parse_data(input);

    let result = data.iter().filter(|(a, b)| overlaps(*a, *b)).count();

    format!("{}", result)
}

fn run_1() {
    #[rustfmt::skip]
    let tests_1 = [
        (1, Some("2")),
    ];

    let all_correct_1 = utils::test(part_1, DATE, 1, &tests_1);
    if all_correct_1 {
        let answer = utils::run(part_1, 1, DATE);
    }
}

fn run_2() {
    #[rustfmt::skip]
    let tests_2 = [
        (1, Some("4")),
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
