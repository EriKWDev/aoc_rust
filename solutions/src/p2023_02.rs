#![allow(dead_code, unreachable_code, unused)]

use utils::*;

pub type Data = (usize, [usize; 3]);
pub const DATE: utils::Date = (2023, 2);

pub fn parse_data<'a>(input: &'a utils::Input) -> impl IntoIterator<Item = Data> + 'a {
    input.lines().map(|line| {
        let (title, rest) = line.split_once(": ").unwrap();

        let id = title.split_once(" ").unwrap().1.parse().unwrap();
        let mut it = rest.split("; ");
        let mut out = [0, 0, 0];

        while let Some(next) = it.next() {
            let mut it = next.split(", ");
            while let Some(val) = it.next() {
                let (n, name) = val.split_once(" ").unwrap();
                let n: usize = n.parse().unwrap();

                #[rustfmt::skip]
                let i = match name.as_bytes()[0] {
                    b'r' => 0, b'g' => 1, b'b' => 2,
                    _ =>  unreachable!(),
                };
                out[i] = usize::max(out[i], n)
            }
        }

        (id, out)
    })
}

pub fn part_1(input: utils::Input) -> String {
    let data = parse_data(&input);
    let mut result = 0;
    let wanted = [12, 13, 14];

    for (id, rgb) in data {
        let impossible = rgb.into_iter().enumerate().any(|(i, n)| n > wanted[i]);
        if !impossible {
            result += id;
        }
    }

    format!("{}", result)
}

pub fn part_2(input: utils::Input) -> String {
    let data = parse_data(&input);
    let mut result = 0;

    for (id, rgb) in data {
        let power: usize = rgb.into_iter().product();
        result += power
    }

    format!("{}", result)
}

fn run_1() {
    #[rustfmt::skip]
    let tests_1 = [
        (1, Some("8")),
    ];

    let all_correct_1 = utils::test(part_1, DATE, 1, &tests_1);
    if all_correct_1 {
        let answer = utils::run(part_1, 1, DATE);
    }
}

fn run_2() {
    #[rustfmt::skip]
    let tests_2 = [
        (1, Some("2286")),
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
