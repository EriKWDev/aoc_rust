#![allow(dead_code, unreachable_code, unused)]

use std::collections::VecDeque;
use std::ops::{Add, Div, Mul};
use std::str::FromStr;
use utils::*;

pub type Data = Vec<Monkey>;

#[derive(Debug, Copy, Clone)]
pub enum Operation {
    Add(usize, bool),
    Mul(usize, bool),
}

#[derive(Debug, Clone)]
pub struct Monkey {
    pub items: VecDeque<usize>,
    pub operation: Operation,
    pub div_test: usize,
    pub result_monkeys: (usize, usize),
}

pub const DATE: utils::Date = (2022, 11);

pub fn parse_data(input: utils::Input) -> Data {
    input
        .split("\n\n")
        .filter_map(|lines| {
            let mut lines = lines.lines().skip(1);

            let items = lines
                .next()?
                .trim_start_matches("  Starting items: ")
                .split(", ")
                .map(|i| i.parse().unwrap())
                .collect::<VecDeque<usize>>();

            let mut operation = lines
                .next()?
                .trim_start_matches("  Operation: new = ")
                .split_whitespace()
                .skip(1);

            let op = operation.next()?;

            let (num, is_old) = operation
                .next()?
                .parse::<usize>()
                .map_or((0, true), |val| (val, false));

            let operation = match op {
                "+" => Operation::Add(num, is_old),
                "*" => Operation::Mul(num, is_old),
                _ => unreachable!(),
            };

            let div_test = lines
                .next()?
                .trim_start_matches("  Test: divisible by ")
                .parse()
                .ok()?;

            let target_0 = lines
                .next()?
                .trim_start_matches("    If true: throw to monkey ")
                .parse()
                .ok()?;

            let target_1 = lines
                .next()?
                .trim_start_matches("    If false: throw to monkey ")
                .parse()
                .ok()?;

            Some(Monkey {
                items,
                operation,
                div_test,
                result_monkeys: (target_0, target_1),
            })
        })
        .collect()
}

pub fn monkey_simulation(rounds: usize, monkeys: &mut [Monkey], is_part_2: bool) -> usize {
    let num_monkeys = monkeys.len();
    let mut results = vec![0; num_monkeys];

    let divisor_prod = monkeys
        .iter()
        .map(|monkey| monkey.div_test)
        .product::<usize>();

    (1..=rounds).for_each(|round| {
        (0..num_monkeys).for_each(|monkey_i| {
            while let Some(item) = monkeys[monkey_i].items.pop_front() {
                let current_monkey = &monkeys[monkey_i];
                results[monkey_i] += 1;

                let new_item = match current_monkey.operation {
                    Operation::Add(_, true) => item.add(item),
                    Operation::Add(num, _) => item.add(num),
                    Operation::Mul(_, true) => item.mul(item),
                    Operation::Mul(num, _) => item.mul(num),
                } % divisor_prod;

                let new_item = if is_part_2 { new_item } else { new_item / 3 };

                if new_item % current_monkey.div_test == 0 {
                    monkeys[current_monkey.result_monkeys.0]
                        .items
                        .push_back(new_item);
                } else {
                    monkeys[current_monkey.result_monkeys.1]
                        .items
                        .push_back(new_item);
                }
            }
        })
    });

    results.sort_unstable();

    let result = results
        .iter()
        .rev()
        .take(2)
        .fold(1, |acc, val| acc.mul(val));

    result
}

pub fn part_1(input: utils::Input) -> String {
    let mut monkeys = parse_data(input);

    const ROUNDS: usize = 20;
    let result = monkey_simulation(ROUNDS, &mut monkeys, false);

    format!("{}", result)
}

pub fn part_2(input: utils::Input) -> String {
    let mut monkeys = parse_data(input);

    const ROUNDS: usize = 10_000;
    let result = monkey_simulation(ROUNDS, &mut monkeys, true);

    format!("{}", result)
}

fn run_1() {
    #[rustfmt::skip]
    let tests_1 = [
        (1, Some("10605")),
    ];

    let all_correct_1 = utils::test(part_1, DATE, 1, &tests_1);
    if all_correct_1 {
        let answer = utils::run(part_1, 1, DATE);
    }
}

fn run_2() {
    #[rustfmt::skip]
    let tests_2 = [
        (1, Some("2713310158")),
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
