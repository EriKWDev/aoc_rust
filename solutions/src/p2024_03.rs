#![allow(dead_code, unreachable_code, unused)]
#![cfg(feature = "regex")]

use utils::*;

pub type Data = Vec<(usize, usize)>;

pub fn parse_data_1(input: utils::Input) -> Data {
    let input = input.trim();

    let mut output = vec![];
    let mut iter = input.chars();
    let mut current = None;

    let mut num = String::new();

    current = iter.next();
    'outer: loop {
        if current.is_none() {
            break;
        }

        let Some('m') = current else {
            current = iter.next();
            continue;
        };
        current = iter.next();
        let Some('u') = current else { continue };
        current = iter.next();
        let Some('l') = current else { continue };
        current = iter.next();
        let Some('(') = current else { continue };

        num.clear();
        'num: loop {
            current = iter.next();
            if let Some(n) = current {
                if n.is_numeric() {
                    num.push(n);
                    continue 'num;
                } else {
                    break 'num;
                }
            } else {
                continue 'outer;
            }
        }
        let left_num = num.parse::<usize>().unwrap();

        let Some(',') = current else { continue };

        num.clear();
        'num: loop {
            current = iter.next();
            if let Some(n) = current {
                if n.is_numeric() {
                    num.push(n);
                    continue 'num;
                } else {
                    break 'num;
                }
            } else {
                continue 'outer;
            }
        }
        let right_num = num.parse::<usize>().unwrap();

        let Some(')') = current else { continue };

        output.push((left_num, right_num))
    }

    output
}

pub fn part_1(input: utils::Input) -> String {
    let data = parse_data_1(input);
    let mut result = data.iter().map(|(a, b)| a * b).sum::<usize>();

    format!("{}", result)
}

#[derive(Default)]
pub struct Program {
    pub instructions: Vec<Instruction>,
}

pub enum Instruction {
    Do,
    Dont,
    Mul(usize, usize),
}

pub struct State {
    pub mul_enabled: bool,
    pub instruction: usize,
    pub memory: usize,
}

impl Default for State {
    fn default() -> Self {
        Self {
            mul_enabled: true,
            instruction: 0,
            memory: 0,
        }
    }
}

pub fn parse_program(input: &utils::Input) -> Program {
    use regex::Regex;

    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    let mut instructions = vec![];

    for cap in re.captures_iter(input) {
        if let Some(matched) = cap.get(0) {
            if matched.as_str().starts_with("mul(") {
                // Extract the two numbers for `mul(num, num)`
                let num1 = cap[1].parse().unwrap();
                let num2 = cap[2].parse().unwrap();
                instructions.push(Instruction::Mul(num1, num2));
            } else if matched.as_str() == "do()" {
                instructions.push(Instruction::Do);
            } else if matched.as_str() == "don't()" {
                instructions.push(Instruction::Dont);
            }
        }
    }

    Program { instructions }
}

pub fn run_program(state: &mut State, program: &Program) {
    loop {
        let Some(instruction) = program.instructions.get(state.instruction) else {
            break;
        };
        state.instruction += 1;

        match instruction {
            Instruction::Do => state.mul_enabled = true,
            Instruction::Dont => state.mul_enabled = false,

            Instruction::Mul(left, right) => {
                if state.mul_enabled {
                    state.memory += left * right;
                }
            }
        }
    }
}

pub fn part_2(input: utils::Input) -> String {
    let program = parse_program(&input);
    let mut state = State::default();
    run_program(&mut state, &program);

    let result = state.memory;
    format!("{}", result)
}

fn run_1(date: utils::Date) {
    #[rustfmt::skip]
    let tests_1 = [
        (1, Some("161")),
    ];

    let all_correct_1 = utils::test(part_1, date, 1, &tests_1);
    if all_correct_1 {
        let answer = utils::run(part_1, 1, date);
    }
}

fn run_2(date: utils::Date) {
    #[rustfmt::skip]
    let tests_2 = [
        (2, Some("48")),
    ];

    let all_correct_2 = utils::test(part_2, date, 2, &tests_2);
    if all_correct_2 {
        let answer = utils::run(part_2, 2, date);
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
