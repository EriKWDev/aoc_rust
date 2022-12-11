#![allow(dead_code, unreachable_code, unused)]

use utils::*;

pub type Data = Vec<((isize, isize), usize)>;
pub const DATE: utils::Date = (2022, 09);

pub mod directions {
    pub const UP: (isize, isize) = (0, 1);
    pub const DOWN: (isize, isize) = (0, -1);
    pub const LEFT: (isize, isize) = (-1, 0);
    pub const RIGHT: (isize, isize) = (1, 0);
}

pub fn parse_data(input: utils::Input) -> Data {
    input
        .lines()
        .map(|line| {
            let mut it = line.split_whitespace();

            let command = it.next().unwrap();
            let amount = it.next().unwrap();

            let command = match command.trim() {
                "U" => directions::UP,
                "D" => directions::DOWN,
                "L" => directions::LEFT,
                "R" => directions::RIGHT,

                _ => unreachable!(),
            };

            let amount = amount.parse::<usize>().unwrap();

            (command, amount)
        })
        .collect::<Vec<_>>()
}

#[inline(always)]
pub fn rope_step(mut head: (isize, isize), direction: (isize, isize)) -> (isize, isize) {
    head.0 += direction.0;
    head.1 += direction.1;

    head
}

pub fn solve_constraints(
    mut head: (isize, isize),
    mut tail: (isize, isize),
) -> ((isize, isize), (isize, isize)) {
    let dx = head.0 - tail.0;
    let dy = head.1 - tail.1;

    // Are they "touching"?
    if dx.abs() <= 1 && dy.abs() <= 1 {
        return (head, tail);
    }

    // Same row
    if tail.1 == head.1 && dx.abs() == 2 {
        tail.0 += dx.signum();
        return (head, tail);
    }

    // Same col
    if tail.0 == head.0 && dy.abs() == 2 {
        tail.1 += dy.signum();
        return (head, tail);
    }

    // Move diagonally
    tail.0 += dx.signum();
    tail.1 += dy.signum();

    (head, tail)
}

pub fn part_1(input: utils::Input) -> String {
    let data = parse_data(input);

    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut visited_positions = HashSet::with_capacity(10000);

    data.iter().for_each(|(direction, amount)| {
        (0..*amount).into_iter().for_each(|_| {
            head = rope_step(head, *direction);
            (head, tail) = solve_constraints(head, tail);

            visited_positions.insert(tail);
        });
    });

    format!("{}", visited_positions.len())
}

pub fn part_2(input: utils::Input) -> String {
    let data = parse_data(input);

    const N: usize = 10;
    const HEAD_POS: usize = 0;
    const TAIL_POS: usize = N - 1;

    let mut rope = [(0, 0); N];
    let mut visited_positions = HashSet::with_capacity(10000);

    data.iter().for_each(|(direction, amount)| {
        (0..*amount).into_iter().for_each(|_| {
            rope[HEAD_POS] = rope_step(rope[HEAD_POS], *direction);

            (HEAD_POS + 1..=TAIL_POS).into_iter().for_each(|i| {
                let (a, b) = (i - 1, i);
                (rope[a], rope[b]) = solve_constraints(rope[a], rope[b]);
            });

            visited_positions.insert(rope[TAIL_POS]);
        });
    });

    format!("{}", visited_positions.len())
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
        (1, Some("1")),
        (2, Some("36")),
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
