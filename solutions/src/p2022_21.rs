#![allow(dead_code, unreachable_code, unused)]

use utils::*;

#[derive(Debug)]
pub enum Value {
    Number(i128),
    Expression {
        left: String,
        right: String,
        kind: ExpressionKind,
    },
}

#[derive(Debug)]
pub enum ExpressionKind {
    Add,
    Multiply,
    Divide,
    Subtract,
    Eq,
}

pub type Data = HashMap<String, Value>;
pub const DATE: utils::Date = (2022, 21);

pub fn parse_data(input: utils::Input) -> Data {
    input
        .lines()
        .map(|line| {
            let (l, r) = line.split_once(": ").unwrap();

            let value: Value;

            if let Some((a, b)) = r.split_once(" + ") {
                value = Value::Expression {
                    left: a.to_owned(),
                    right: b.to_owned(),
                    kind: ExpressionKind::Add,
                };
            } else if let Some((a, b)) = r.split_once(" * ") {
                value = Value::Expression {
                    left: a.to_owned(),
                    right: b.to_owned(),
                    kind: ExpressionKind::Multiply,
                };
            } else if let Some((a, b)) = r.split_once(" / ") {
                value = Value::Expression {
                    left: a.to_owned(),
                    right: b.to_owned(),
                    kind: ExpressionKind::Divide,
                };
            } else if let Some((a, b)) = r.split_once(" - ") {
                value = Value::Expression {
                    left: a.to_owned(),
                    right: b.to_owned(),
                    kind: ExpressionKind::Subtract,
                };
            } else {
                let num = r.parse().unwrap();
                value = Value::Number(num);
            }

            (l.to_owned(), value)
        })
        .collect::<Data>()
}

pub fn evaluate(data: &Data, cache: &mut HashMap<String, i128>, key: &String) -> i128 {
    let contains = cache.get(key);

    if let Some(&value) = contains {
        value
    } else {
        let value = match data.get(key).unwrap() {
            Value::Number(n) => *n,
            Value::Expression { left, right, kind } => match kind {
                ExpressionKind::Add => evaluate(data, cache, left) + evaluate(data, cache, right),
                ExpressionKind::Multiply => {
                    evaluate(data, cache, left) * evaluate(data, cache, right)
                }
                ExpressionKind::Divide => {
                    evaluate(data, cache, left) / evaluate(data, cache, right)
                }
                ExpressionKind::Subtract => {
                    evaluate(data, cache, left) - evaluate(data, cache, right)
                }
                ExpressionKind::Eq => {
                    let l = evaluate(data, cache, left);
                    let r = evaluate(data, cache, right);

                    (l - r) * (l - r)
                }
            },
        };

        cache.insert(key.clone(), value);

        value
    }
}

pub fn part_1(input: utils::Input) -> String {
    let data = parse_data(input);
    let key = "root".to_owned();

    let mut cache = HashMap::new();
    let result = evaluate(&data, &mut cache, &key);

    format!("{}", result)
}

pub fn part_2(input: utils::Input) -> String {
    let mut data = parse_data(input);

    let root = "root".to_owned();
    let Some(Value::Expression { left, right, kind }) = data.get_mut(&root) else { unreachable!() };
    *kind = ExpressionKind::Eq;

    data.insert("humn".to_owned(), Value::Number(i));

    let mut cache = HashMap::new();
    let root = "root".to_owned();
    let n = evaluate(&data, &mut cache, &root);

    format!("{}", n)
}

fn run_1() {
    #[rustfmt::skip]
    let tests_1 = [
        (1, Some("152")),
    ];

    let all_correct_1 = utils::test(part_1, DATE, 1, &tests_1);
    if all_correct_1 {
        let answer = utils::run(part_1, 1, DATE);
    }
}

fn run_2() {
    #[rustfmt::skip]
    let tests_2 = [
        // (1, Some("301")),
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
