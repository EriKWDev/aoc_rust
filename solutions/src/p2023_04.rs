#![allow(dead_code, unreachable_code, unused)]

use utils::*;

pub type Data = Vec<(
    std::collections::HashSet<usize>,
    std::collections::HashSet<usize>,
)>;
pub const DATE: utils::Date = (2023, 4);

pub fn parse_data(input: utils::Input) -> Data {
    input
        .lines()
        .map(|line| {
            let (title, rest) = line.split_once(": ").unwrap();
            let (nums1, nums2) = rest.split_once(" | ").unwrap();

            let nums1 = nums1
                .split_ascii_whitespace()
                .into_iter()
                .filter_map(|v| v.parse().ok())
                .collect::<std::collections::HashSet<usize>>();
            let nums2 = nums2
                .split_ascii_whitespace()
                .into_iter()
                .filter_map(|v| v.parse().ok())
                .collect::<std::collections::HashSet<usize>>();

            (nums1, nums2)
        })
        .collect()
}

pub fn part_1(input: utils::Input) -> String {
    let data = parse_data(input);
    let mut result = 0;

    for (winning, yours) in data {
        let mut value = 0;
        for num in winning.intersection(&yours) {
            if value == 0 {
                value = 1;
            } else {
                value *= 2;
            }
        }

        result += value;
    }

    format!("{}", result)
}

pub fn part_2(input: utils::Input) -> String {
    let data = parse_data(input);
    let mut nums = vec![1; data.len()];
    for (i, (winning, yours)) in data.iter().enumerate() {
        let mut value = 0;
        let mut num_winning = 0;
        for num in winning.intersection(&yours) {
            num_winning += 1;

            if value == 0 {
                value = 1;
            } else {
                value *= 2;
            }
        }

        let n = nums[i];
        for j in i + 1..=i + num_winning {
            nums[j] += n * 1;
        }
    }

    let result: usize = nums.into_iter().sum();
    format!("{}", result)
}

fn run_1() {
    #[rustfmt::skip]
    let tests_1 = [
        (1, Some("13")),
    ];

    let all_correct_1 = utils::test(part_1, DATE, 1, &tests_1);
    if all_correct_1 {
        let answer = utils::run(part_1, 1, DATE);
    }
}

fn run_2() {
    #[rustfmt::skip]
    let tests_2 = [
        (1, Some("30")),
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
