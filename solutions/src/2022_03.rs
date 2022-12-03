#![allow(dead_code, unreachable_code, unused)]

use utils::*;

const DATE: utils::Date = (2022, 03);

fn score_intersection<'a, I>(intersection: I) -> usize
where
    I: std::iter::IntoIterator<Item = &'a char>,
{
    const LOWER_N: usize = 96;
    const UPPER_N: usize = 38;

    intersection
        .into_iter()
        .map(|char| {
            let char_number = *char as usize;

            let score = match char {
                'a'..='z' => char_number.wrapping_sub(LOWER_N),
                'A'..='Z' => char_number.wrapping_sub(UPPER_N),

                _ => unreachable!(),
            };

            score
        })
        .sum::<usize>()
}

pub fn part_1(input: utils::Input) -> String {
    let mut buff_a = HashSet::new();
    let mut buff_b = HashSet::new();

    let result = input
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let (a, b) = line.split_at(line.len() / 2);

            buff_a.extend(a.chars());
            buff_b.extend(b.chars());

            let inter = buff_a.intersection(&buff_b);
            let score = score_intersection(inter);

            buff_a.clear();
            buff_b.clear();

            score
        })
        .sum::<usize>();

    format!("{}", result)
}

pub fn part_2(input: utils::Input) -> String {
    let mut rucksack_buffer = HashSet::new();
    let mut common_items = HashSet::new();

    let lines = input.lines().map(|line| line.unwrap()).collect::<Vec<_>>();

    let result = lines
        .chunks(3)
        .map(|group| {
            group.iter().for_each(|rucksack| {
                let chars = rucksack.chars();

                if common_items.is_empty() {
                    common_items.extend(chars);
                } else {
                    rucksack_buffer.extend(chars);

                    common_items = common_items
                        .intersection(&rucksack_buffer)
                        .cloned()
                        .collect();

                    rucksack_buffer.clear();
                }
            });

            let score = score_intersection(common_items.iter());
            common_items.clear();

            score
        })
        .sum::<usize>();

    format!("{}", result)
}

fn run_1() {
    #[rustfmt::skip]
    let tests_1 = [
        (1, Some("157")),
    ];

    let all_correct_1 = utils::test(part_1, DATE, 1, &tests_1);
    if all_correct_1 {
        let answer = utils::run(part_1, 1, DATE);
    }
}

fn run_2() {
    #[rustfmt::skip]
    let tests_2 = [
        (1, Some("70")),
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
