#![allow(dead_code, unreachable_code, unused)]

use utils::*;

pub type Item = Vec<usize>;
pub type Data = Vec<Item>;

pub fn parse_data(input: utils::Input) -> Data {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|it| it.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

pub fn part_1(input: utils::Input) -> String {
    let data = parse_data(input);
    let mut result = 0;

    for line in data {
        let n = line.len();
        let mut c = 0;
        let mut the_a = 0;
        let mut the_b = 0;

        for i in 0..n {
            for j in i + 1..n {
                let a = line[i];
                let b = line[j];
                let n = a * 10 + b;
                if n > c {
                    c = n;
                    the_a = a;
                    the_b = b;
                }
            }
        }
        result += c;
    }

    format!("{}", result)
}

fn max_k_subsequence_digits(s: &str, k: usize, stack: &mut Vec<(u8, usize)>, output: &mut String) {
    let n = s.len();
    if k > n {
        return;
    }
    let bytes = s.as_bytes();

    stack.clear();

    for (i, &b) in bytes.iter().enumerate() {
        let rem = n - i;
        while let Some(&(top, _)) = stack.last() {
            if top < b && stack.len() + rem > k {
                stack.pop();
            } else {
                break;
            }
        }
        if stack.len() < k {
            stack.push((b, i));
        }
    }

    output.clear();
    output.extend(stack.iter().take(k).map(|&(b, _)| b as char));
}

pub fn part_2(input: utils::Input) -> String {
    let mut result = 0;
    let mut stack_buf = vec![];
    let mut number = String::new();
    for line in input.lines() {
        stack_buf.clear();
        number.clear();
        max_k_subsequence_digits(&line, 12, &mut stack_buf, &mut number);
        let n: usize = number.parse().unwrap();
        result += n;
    }

    format!("{}", result)
}

pub fn run_1(date: utils::Date) {
    #[rustfmt::skip]
    let tests_1 = [
        (1, Some("357")),
    ];

    let all_correct_1 = utils::test(part_1, date, 1, &tests_1);
    if all_correct_1 {
        let answer = utils::run(part_1, 1, date);
    }
}

pub fn run_2(date: utils::Date) {
    #[rustfmt::skip]
    let tests_2 = [
        (1, Some("3121910778619")),
    ];

    let all_correct_2 = utils::test(part_2, date, 2, &tests_2);
    if all_correct_2 {
        let answer = utils::run(part_2, 2, date);
        assert!(answer.parse::<usize>().unwrap() < 15632830017380259612);
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
