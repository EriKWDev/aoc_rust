#![allow(dead_code, unreachable_code, unused)]
#![cfg(feature = "serde")]

use serde_json::Value;
use utils::*;

pub type Data = Vec<Value>;
pub const DATE: utils::Date = (2022, 13);

fn compare_values(a: &Value, b: &Value) -> std::cmp::Ordering {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => x.as_u64().unwrap().cmp(&y.as_u64().unwrap()),

        (Value::Array(a), Value::Array(b)) => {
            let max_len = a.len().max(b.len());

            for i in 0..max_len {
                match (a.get(i), b.get(i)) {
                    (None, _) => return std::cmp::Ordering::Less,

                    (_, None) => return std::cmp::Ordering::Greater,

                    (Some(x), Some(y)) => match compare_values(x, y) {
                        std::cmp::Ordering::Equal => {}

                        c => return c,
                    },
                }
            }
            std::cmp::Ordering::Equal
        }

        (Value::Array(_), Value::Number(_)) => compare_values(a, &Value::Array(vec![b.clone()])),
        (Value::Number(_), Value::Array(_)) => compare_values(&Value::Array(vec![a.clone()]), b),

        _ => unreachable!(),
    }
}

pub fn parse_data(input: utils::Input) -> Data {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| serde_json::from_str::<Value>(line).unwrap())
        .collect::<Vec<_>>()
}

pub fn part_1(input: utils::Input) -> String {
    let data = parse_data(input);

    let result = data
        .chunks(2)
        .enumerate()
        .filter_map(|(i, w)| {
            let (a, b) = (&w[0], &w[1]);

            match compare_values(a, b) {
                std::cmp::Ordering::Greater => None,

                _ => Some(i + 1),
            }
        })
        .sum::<usize>();

    format!("{}", result)
}

pub fn part_2(input: utils::Input) -> String {
    let mut data = parse_data(input);

    let div_a = serde_json::from_str::<Value>("[[2]]").unwrap();
    let div_b = serde_json::from_str::<Value>("[[6]]").unwrap();
    let divs = [div_a.clone(), div_b.clone()];

    data.push(div_a);
    data.push(div_b);

    data.sort_unstable_by(|a, b| compare_values(a, b));

    let result = data
        .iter()
        .enumerate()
        .filter_map(|(i, value)| {
            if divs.contains(value) {
                Some(i + 1)
            } else {
                None
            }
        })
        .product::<usize>();

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
        (1, Some("140")),
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
