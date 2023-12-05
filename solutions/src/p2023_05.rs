#![allow(dead_code, unreachable_code, unused)]

use utils::*;

pub type Data = Vec<String>;
pub const DATE: utils::Date = (2023, 5);

pub fn parse_data(input: utils::Input) -> Data {
    input
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<_>>()
}

pub fn part_1(input: utils::Input) -> String {
    let data = parse_data(input);

    let (_, rest) = data[0].split_once("seeds: ").unwrap();
    let mut nums = rest
        .split_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut maps = vec![];

    let mut lines = (&data[2..]).iter();
    while let Some(line) = lines.next() {
        if let Some((name, _)) = line.split_once(" map:") {
            let (a, b) = name.split_once("-to-").unwrap();
            let j = maps.len();
            maps.push(vec![]);

            while let Some(line) = lines.next() {
                if line.is_empty() {
                    break;
                }

                let mut abc = [0, 0, 0];
                let mut i = 0;
                for n in line.split_whitespace().map(|n| n.parse().unwrap()) {
                    abc[i % 3] = n;
                    i += 1;
                    if i > 0 && i % 3 == 0 {
                        let [a, b, c] = abc;
                        maps[j].push((a, b, c));
                    }
                }
            }
        }
    }

    maps.reverse();

    let mut new_nums = vec![];
    while let Some(map) = maps.pop() {
        new_nums.clear();

        'outer: for num in nums.drain(..) {
            for &(d_start, source_start, range) in &map {
                if num >= source_start && num <= source_start + range {
                    let d = num - source_start;
                    new_nums.push(d + d_start);
                    continue 'outer;
                }
            }

            new_nums.push(num);
        }

        nums.append(&mut new_nums);
    }

    let mut result = nums.into_iter().min().unwrap();

    format!("{}", result)
}

pub fn part_2(input: utils::Input) -> String {
    let data = parse_data(input);

    let (_, rest) = data[0].split_once("seeds: ").unwrap();
    let mut pre_nums = rest
        .split_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut nums = vec![];
    for ab in pre_nums.chunks_exact(2) {
        let &[a, b] = ab else { panic!() };

        for i in a..=a + b {
            nums.push(i);
        }
    }

    let mut maps = vec![];

    let mut lines = (&data[2..]).iter();
    while let Some(line) = lines.next() {
        if let Some((name, _)) = line.split_once(" map:") {
            let (a, b) = name.split_once("-to-").unwrap();
            let j = maps.len();
            maps.push(vec![]);

            while let Some(line) = lines.next() {
                if line.is_empty() {
                    break;
                }

                let mut abc = [0, 0, 0];
                let mut i = 0;
                for n in line.split_whitespace().map(|n| n.parse().unwrap()) {
                    abc[i % 3] = n;
                    i += 1;
                    if i > 0 && i % 3 == 0 {
                        let [a, b, c] = abc;
                        maps[j].push((a, b, c));
                    }
                }
            }
        }
    }

    maps.reverse();

    let mut new_nums = vec![];
    while let Some(map) = maps.pop() {
        new_nums.clear();

        'outer: for num in nums.drain(..) {
            for &(d_start, source_start, range) in &map {
                if num >= source_start && num <= source_start + range {
                    let d = num - source_start;
                    new_nums.push(d + d_start);
                    continue 'outer;
                }
            }

            new_nums.push(num);
        }

        nums.append(&mut new_nums);
    }

    let mut result = nums.into_iter().min().unwrap();

    format!("{}", result)
}

fn run_1() {
    #[rustfmt::skip]
    let tests_1 = [
        (1, Some("35")),
    ];

    let all_correct_1 = utils::test(part_1, DATE, 1, &tests_1);
    if all_correct_1 {
        let answer = utils::run(part_1, 1, DATE);
    }
}

fn run_2() {
    #[rustfmt::skip]
    let tests_2 = [
        (1, Some("46")),
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
