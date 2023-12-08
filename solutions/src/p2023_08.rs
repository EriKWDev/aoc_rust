#![allow(dead_code, unreachable_code, unused)]

use utils::*;

pub type Data<'a> = (Vec<usize>, std::collections::HashMap<&'a str, [&'a str; 2]>);
pub const DATE: utils::Date = (2023, 8);

pub fn parse_data<'a>(input: &'a utils::Input) -> Data<'a> {
    let mut lines = input.lines();

    let first_line = lines.next().unwrap();
    let mut instructions = first_line
        .trim()
        .chars()
        .map(|lr| match lr {
            'L' => 0,
            'R' => 1,
            _ => panic!(),
        })
        .collect::<Vec<_>>();

    let _ = lines.next();

    let map = lines
        .map(|line| {
            let (name, lr) = line.split_once(" = (").unwrap();

            let (l, r) = lr.split_once(", ").unwrap();
            let (r, _) = r.split_once(")").unwrap();

            (name, [l, r])
        })
        .collect::<std::collections::HashMap<_, _>>();

    (instructions, map)
}

pub fn part_1(input: utils::Input) -> String {
    let (instructions, map) = parse_data(&input);

    let mut result = 0;
    let mut current = "AAA";
    while current != "ZZZ" {
        let it = instructions[result % instructions.len()];
        current = map.get(current).unwrap()[it];
        result += 1;
    }

    format!("{}", result)
}

pub fn part_2(input: utils::Input) -> String {
    let (instructions, map) = parse_data(&input);

    let mut current = map
        .keys()
        .copied()
        .flat_map(|k| k.ends_with('A').then_some(k));

    let mut nums = vec![];
    while let Some(mut current) = current.next() {
        let mut n = 0;
        while !current.ends_with('Z') {
            let it = instructions[n % instructions.len()];
            current = map.get(current).unwrap()[it];
            n += 1;
        }
        nums.push(n);
    }

    let result = lcm_of_slice(&nums);
    format!("{}", result)
}

fn run_1() {
    #[rustfmt::skip]
    let tests_1 = [
        (1, Some("2")),
        (2, Some("6")),
    ];

    let all_correct_1 = utils::test(part_1, DATE, 1, &tests_1);
    if all_correct_1 {
        let answer = utils::run(part_1, 1, DATE);
    }
}

fn run_2() {
    #[rustfmt::skip]
    let tests_2 = [
        (3, Some("6")),
    ];

    let all_correct_2 = utils::test(part_2, DATE, 2, &tests_2);
    if all_correct_2 {
        let answer = utils::run(part_2, 2, DATE);

        let n: u128 = answer.parse().unwrap();
        if n <= 1031579235 {
            println!("Correct: NO, too low");
        }
    }
}

fn main() {
    // run_1();
    run_2();
}
