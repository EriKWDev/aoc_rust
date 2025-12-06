#![allow(dead_code, unreachable_code, unused)]

use utils::*;

pub type Data = (Vec<(usize, usize)>, Vec<usize>);

pub fn parse_data(input: utils::Input) -> Data {
    let (a, b) = input.split_once("\n\n").unwrap();

    let a = a
        .lines()
        .map(|line| {
            let (id_a, id_b) = line.split_once('-').unwrap();
            let id_a = id_a.parse().unwrap();
            let id_b = id_b.parse().unwrap();

            (id_a, id_b)
        })
        .collect::<Vec<_>>();

    let b = b
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<_>>();

    (a, b)
}

pub fn part_1(input: utils::Input) -> String {
    let data = parse_data(input);

    let (ranges, ids) = data;

    let mut result = 0;

    'next_id: for id in ids {
        for &(a, b) in ranges.iter() {
            if (a..=b).contains(&id) {
                result += 1;
                continue 'next_id;
            }
        }
    }

    format!("{}", result)
}

fn merge(o: &mut Vec<(usize, usize)>, i: usize) {}

pub fn part_2(input: utils::Input) -> String {
    let data = parse_data(input);

    let (ranges, _ids) = data;

    let mut o = vec![];
    let mut todo = ranges;

    'next_range: while let Some((id_a, id_b)) = todo.pop() {
        o.sort_unstable();

        for i in 0..o.len() {
            let (min, max) = o[i];

            if id_a >= min && id_b <= max {
                continue 'next_range;
            }
            if id_a <= min && id_b >= max {
                o[i] = (id_a, id_b);
                continue 'next_range;
            }
            if id_a < min && id_b <= max && id_b >= min {
                o[i] = (id_a, max);
                continue 'next_range;
            }
            if id_a >= min && id_a <= max && id_b > max {
                o[i] = (min, id_b);
                todo.extend(o.drain(i + 1..));
                continue 'next_range;
            }
        }
        o.push((id_a, id_b));
    }

    let mut result = 0;
    for (min, max) in o {
        result += max - min + 1;
    }

    format!("{}", result)
}

pub fn run_1(date: utils::Date) {
    #[rustfmt::skip]
    let tests_1 = [
        (1, Some("3")),
    ];

    let all_correct_1 = utils::test(part_1, date, 1, &tests_1);
    if all_correct_1 {
        let answer = utils::run(part_1, 1, date);
    }
}

pub fn run_2(date: utils::Date) {
    #[rustfmt::skip]
    let tests_2 = [
        (1, Some("14")),
        (2, Some("1")),
    ];

    let all_correct_2 = utils::test(part_2, date, 2, &tests_2);
    if all_correct_2 {
        let answer = utils::run(part_2, 2, date);
        let n: usize = answer.parse().unwrap();
        assert!(n < 356496312736338);
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
