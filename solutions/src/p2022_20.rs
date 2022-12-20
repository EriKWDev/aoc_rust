#![allow(dead_code, unreachable_code, unused)]

use utils::*;

pub type Data = Vec<i64>;
pub const DATE: utils::Date = (2022, 20);

pub fn parse_data(input: utils::Input) -> Data {
    input
        .lines()
        .filter_map(|line| line.trim().parse::<i64>().ok())
        .collect::<Vec<_>>()
}

fn decrypt_file(nums: &[i64], n: usize, decryption: i64) -> i64 {
    let mut indices_and_numbers = nums
        .iter()
        .enumerate()
        .map(|(i, x)| (i as i64, *x as i64 * decryption))
        .collect::<Vec<_>>();

    let total = indices_and_numbers.len() as i64;

    for _ in 0..n {
        for i in 0..total {
            let index = indices_and_numbers
                .iter()
                .position(|&(index, _)| index == i)
                .unwrap();

            let value = indices_and_numbers.remove(index);

            let new_index = (index as i64 + value.1).rem_euclid(total - 1);
            indices_and_numbers.insert(new_index as usize, value);
        }
    }

    let zero_index = indices_and_numbers.iter().position(|x| x.1 == 0).unwrap();
    let indices = [
        (1000 + zero_index) % total as usize,
        (2000 + zero_index) % total as usize,
        (3000 + zero_index) % total as usize,
    ];

    indices.iter().map(|&i| indices_and_numbers[i].1).sum()
}

pub fn part_1(input: utils::Input) -> String {
    let data = parse_data(input);
    let result = decrypt_file(&data, 1, 1);

    format!("{}", result)
}

pub fn part_2(input: utils::Input) -> String {
    let data = parse_data(input);
    let result = decrypt_file(&data, 10, 811589153);

    format!("{}", result)
}

fn run_1() {
    #[rustfmt::skip]
    let tests_1 = [
        (1, Some("3")),
    ];

    let all_correct_1 = utils::test(part_1, DATE, 1, &tests_1);
    if all_correct_1 {
        let answer = utils::run(part_1, 1, DATE);
    }
}

fn run_2() {
    #[rustfmt::skip]
    let tests_2 = [
        (1, Some("1623178306")),
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
