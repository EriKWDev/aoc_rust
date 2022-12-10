#![allow(dead_code, unreachable_code, unused)]

use utils::*;

#[derive(Debug, Copy, Clone)]
pub struct Registers {
    pub x: isize,
}

#[derive(Debug, Copy, Clone)]
pub enum InstructionKind {
    Noop,
    AddX(isize),
}

#[derive(Debug, Copy, Clone)]
pub struct Instruction {
    pub execution_time: usize,
    pub kind: InstructionKind,
}

pub type Data = Vec<Instruction>;

pub const DATE: utils::Date = (2022, 10);

pub fn parse_data(input: utils::Input) -> Data {
    input
        .lines()
        .map(|line| {
            let mut it = line.split_whitespace();
            let command = it.next().unwrap();

            let instruction = match command {
                "noop" => Instruction {
                    kind: InstructionKind::Noop,
                    execution_time: 1,
                },

                "addx" => {
                    let param_1 = it.next().unwrap();
                    let param_1 = param_1.trim().parse().unwrap();

                    Instruction {
                        kind: InstructionKind::AddX(param_1),
                        execution_time: 2,
                    }
                }

                _ => unreachable!(),
            };

            instruction
        })
        .collect::<Vec<_>>()
}

pub fn part_1(input: utils::Input) -> String {
    let mut instructions = parse_data(input);

    let mut time_to_instructions_map =
        std::collections::BTreeMap::<usize, Vec<InstructionKind>>::new();
    let mut tmp = 0;
    instructions.iter().for_each(|instruction| {
        let entry = time_to_instructions_map
            .entry(tmp + instruction.execution_time)
            .or_insert(Vec::with_capacity(1));

        entry.push(instruction.kind);
        tmp += instruction.execution_time;
    });

    let mut cycle = 0;
    let mut registers = Registers { x: 1 };
    let mut result = 0;
    loop {
        cycle += 1;
        if cycle >= 20 && (cycle - 20) % 40 == 0 {
            result += (cycle as isize) * registers.x;
        }

        if let Some(instructions) = time_to_instructions_map.remove(&cycle) {
            for instruction_kind in instructions {
                match instruction_kind {
                    InstructionKind::Noop => {}
                    InstructionKind::AddX(dx) => registers.x += dx,
                }
            }
        }

        if time_to_instructions_map.is_empty() {
            break;
        }
    }

    format!("{}", result)
}

pub fn part_2(input: utils::Input) -> String {
    let mut instructions = parse_data(input);

    let mut time_to_instructions_map =
        std::collections::BTreeMap::<usize, Vec<InstructionKind>>::new();
    let mut tmp = 0;
    instructions.iter().for_each(|instruction| {
        let entry = time_to_instructions_map
            .entry(tmp + instruction.execution_time)
            .or_insert(Vec::with_capacity(1));

        entry.push(instruction.kind);
        tmp += instruction.execution_time;
    });

    let mut cycle = 0;
    let mut registers = Registers { x: 1 };
    let mut crt = [[false; 40]; 6];
    let mut crt_position: usize = 0;

    loop {
        cycle += 1;
        crt_position = (crt_position + 1) % (40 * 6);

        if let Some(instructions) = time_to_instructions_map.remove(&cycle) {
            for instruction_kind in instructions {
                match instruction_kind {
                    InstructionKind::Noop => {}
                    InstructionKind::AddX(dx) => registers.x += dx,
                }
            }
        }

        let x = crt_position % 40;
        let y = crt_position / 40;

        if x as isize == registers.x
            || x as isize == registers.x - 1
            || x as isize == registers.x + 1
        {
            crt[y][x] = true;
        }

        if time_to_instructions_map.is_empty() {
            break;
        }
    }

    let mut result = String::new();

    for y in (0..6).into_iter() {
        for x in 0..40 {
            let value = crt[y][x];
            let value = if value { "█" } else { " " };
            result += value;
        }
        result += "\n";
    }

    format!("\n{}", result)
}

fn run_1() {
    #[rustfmt::skip]
    let tests_1 = [
        // (2, None),
        (1, Some("13140")),
    ];

    let all_correct_1 = utils::test(part_1, DATE, 1, &tests_1);
    if all_correct_1 {
        let answer = utils::run(part_1, 1, DATE);
    }
}

fn run_2() {
    #[rustfmt::skip]
    let tests_2 = [
        (1, None),
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
