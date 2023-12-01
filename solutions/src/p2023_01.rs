#![allow(dead_code, unreachable_code, unused)]

use utils::*;

pub type Data = Vec<String>;
pub const DATE: utils::Date = (2023, 01);

pub fn parse_data(input: utils::Input) -> Data {
    input.lines().map(|s| s.to_string()).collect::<Vec<_>>()
}

pub fn part_1(input: utils::Input) -> String {
    let data = parse_data(input);
    let mut result = 0;

    for line in data {
        let Some(a) = line.chars().find_map(|c| c.is_numeric().then_some(c)) else {
            continue;
        };
        let Some(b) = line.chars().rev().find_map(|c| c.is_numeric().then_some(c)) else {
            continue;
        };

        let (a, b): (u32, u32) = (a.to_digit(10).unwrap(), b.to_digit(10).unwrap());
        let v = a * 10 + b;
        result += v;
    }

    format!("{}", result)
}

pub fn part_2(input: utils::Input) -> String {
    let data = parse_data(input);
    let mut result = 0;

    let numbers: [&[char]; 9] = [
        &['o', 'n', 'e'],
        &['t', 'w', 'o'],
        &['t', 'h', 'r', 'e', 'e'],
        &['f', 'o', 'u', 'r'],
        &['f', 'i', 'v', 'e'],
        &['s', 'i', 'x'],
        &['s', 'e', 'v', 'e', 'n'],
        &['e', 'i', 'g', 'h', 't'],
        &['n', 'i', 'n', 'e'],
    ];

    let mut nums: Vec<usize> = vec![];

    for line in data {
        nums.clear();

        // let mut chars = line.chars();
        let mut rest = &line[..];
        let mut a = "";
        let mut c = 'c';

        loop {
            (a, rest) = rest.split_at(1);
            c = a.chars().next().unwrap();

            if c.is_numeric() {
                nums.push(c.to_digit(10).unwrap() as _);
            } else {
                let mut streaks = [0_i32; 9];

                'words: for char in a.chars().chain(rest.chars()) {
                    let mut any = false;
                    for (i, word) in numbers.into_iter().enumerate() {
                        let d = streaks[i];
                        if d == -1 {
                            continue;
                        }
                        let d = d as usize;

                        if char == word[d] {
                            any = true;
                            if (d + 1) == word.len() {
                                streaks.fill(0);
                                nums.push((i + 1));
                                break 'words;
                            } else {
                                streaks[i] += 1;
                            }
                        } else {
                            streaks[i] = -1;
                        }
                    }
                    if !any {
                        break;
                    }
                }
            }

            if rest.is_empty() {
                break;
            }
        }

        let v = nums.first().unwrap() * 10 + nums.last().unwrap();
        // let ns1 = format!("{nums:?}");
        // println!("{line:.<50} n{ns1:.<30} {v}");
        result += v;
    }

    format!("{}", result)
}

fn run_1() {
    #[rustfmt::skip]
    let tests_1 = [
        (1, Some("142")),
    ];

    let all_correct_1 = utils::test(part_1, DATE, 1, &tests_1);
    if all_correct_1 {
        let answer = utils::run(part_1, 1, DATE);
    }
}

fn run_2() {
    #[rustfmt::skip]
    let tests_2 = [
        (2, Some("281")),
        (3, Some("36")),
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
