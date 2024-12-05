#![allow(dead_code, unreachable_code, unused)]

use utils::*;

pub type Data = Vec<Vec<isize>>;

pub fn parse_data(input: utils::Input) -> Data {
    input
        .lines()
        .map(|line| {
            line.to_string()
                .split_whitespace()
                .filter_map(|n| n.parse().ok())
                .collect()
        })
        .collect()
}

pub fn is_diff_safe(diff: isize, seq_sign: isize) -> bool {
    let abs = diff.abs();
    if abs < 1 || abs > 3 {
        return false;
    }

    if diff.signum() != seq_sign {
        return false;
    }

    true
}

pub fn is_safe_1(nums: &[isize]) -> bool {
    let mut seq_sign = (nums[1] - nums[0]).signum();

    nums.windows(2).all(|s| {
        let &[a, b] = s else { unreachable!() };
        let diff = b - a;
        is_diff_safe(diff, seq_sign)
    })
}

pub fn is_safe_2(nums: &[isize]) -> bool {
    let mut v = Vec::with_capacity(50);

    for i in 0..nums.len() {
        v.extend_from_slice(&nums[0..i]);
        v.extend_from_slice(&nums[i + 1..nums.len()]);

        if is_safe_1(&v) {
            return true;
        }

        v.clear();
    }

    false
}

pub fn part_1(input: utils::Input) -> String {
    let result = parse_data(input)
        .iter()
        .filter(|nums| is_safe_1(&nums))
        .count();
    format!("{}", result)
}

pub fn part_2(input: utils::Input) -> String {
    let result = parse_data(input)
        .iter()
        .filter(|nums| is_safe_2(&nums))
        .count();
    format!("{}", result)
}

fn run_1(date: utils::Date) {
    #[rustfmt::skip]
    let tests_1 = [
        (1, Some("2")),
    ];

    let all_correct_1 = utils::test(part_1, date, 1, &tests_1);
    if all_correct_1 {
        let answer = utils::run(part_1, 1, date);
    }
}

fn run_2(date: utils::Date) {
    #[rustfmt::skip]
    let tests_2 = [
        (1, Some("4")),
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
