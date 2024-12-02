#![allow(dead_code, unreachable_code, unused)]

use utils::*;

pub type Data = (Vec<usize>, Vec<usize>);
pub const DATE: utils::Date = (2024, 01);

pub fn parse_data(input: utils::Input) -> Data {
    input
        .lines()
        .map(|line| line.to_string())
        .filter_map(|line| {
            let it = line.trim().split_once("   ");
            it.map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
        })
        .unzip()
}

pub fn part_1(input: utils::Input) -> String {
    let (mut a, mut b) = parse_data(input);
    a.sort_unstable();
    b.sort_unstable();

    let diffs = a.iter().zip(b.iter()).map(|(&a, &b)| {
        let diff = (a as isize - b as isize).abs() as usize;
        diff
    });

    let sum = diffs.sum::<usize>();

    format!("{}", sum)
}

pub fn part_2(input: utils::Input) -> String {
    let (mut a, mut b) = parse_data(input);

    let mut b_counts = HashMap::new();

    for n in b {
        let m = b_counts.entry(n).or_insert(0);
        *m += 1;
    }

    let result = a
        .into_iter()
        .map(|n| {
            let tot = b_counts.get(&n).copied().unwrap_or(0);
            n * tot
        })
        .sum::<usize>();

    format!("{}", result)
}

fn run_1() {
    #[rustfmt::skip]
    let tests_1 = [
        (1, Some("11")),
    ];

    let all_correct_1 = utils::test(part_1, DATE, 1, &tests_1);
    if all_correct_1 {
        let answer = utils::run(part_1, 1, DATE);
    }
}

fn run_2() {
    #[rustfmt::skip]
    let tests_2 = [
        (1, Some("31")),
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
