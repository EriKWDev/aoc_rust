#![allow(dead_code, unreachable_code, unused)]

use utils::*;

pub type Data = HashMap<String, (usize, Vec<String>)>;
pub const DATE: utils::Date = (2022, 16);

pub fn parse_data(input: utils::Input) -> Data {
    input
        .lines()
        .map(|line| {
            let mut line = line.trim().split_whitespace();
            let mut line = line.skip(1);

            let valve = line.next().unwrap();

            let mut line = line.skip(2);
            let rate = line.next().unwrap();
            let (_, rate) = rate.split_once("=").unwrap();
            let (rate, _) = rate.split_once(";").unwrap();
            let rate = rate.parse::<usize>().unwrap();

            let mut line = line.skip(4);

            let valves = line.map(|valve| valve.to_owned()).collect::<Vec<_>>();

            (valve.to_owned(), (rate, valves))
        })
        .collect::<HashMap<_, _>>()
}

pub fn part_1(input: utils::Input) -> String {
    let data = parse_data(input);

    let pressure = data
        .iter()
        .map(|(key, (rate, _))| (key, rate))
        .collect::<HashMap<_, _>>();

    let mut visited = HashSet::<String>::with_capacity(data.len());

    let key = "AA".to_owned();
    let rate = pressure.get(&key).unwrap();

    let mut to_do = BinaryHeap::from([(rate, key)]);
    while let Some(current) = to_do.pop() {}

    let mut result = 0;
    format!("{}", result)
}

pub fn part_2(input: utils::Input) -> String {
    let data = parse_data(input);
    let mut result = 0;

    format!("{}", result)
}

fn run_1() {
    #[rustfmt::skip]
    let tests_1 = [
        (1, Some("1651")),
    ];

    let all_correct_1 = utils::test(part_1, DATE, 1, &tests_1);
    if all_correct_1 {
        let answer = utils::run(part_1, 1, DATE);
    }
}

fn run_2() {
    #[rustfmt::skip]
    let tests_2 = [
        (1, Some("1")),
    ];

    let all_correct_2 = utils::test(part_2, DATE, 2, &tests_2);
    if all_correct_2 {
        let answer = utils::run(part_2, 2, DATE);
    }
}

fn main() {
    run_1();
    // run_2();
}
