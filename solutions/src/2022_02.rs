#![allow(dead_code, unreachable_code, unused)]

use utils::*;

#[derive(Debug, Copy, Clone)]
pub enum Shape {
    Rock,
    Paper,
    Scissor,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Outcome {
    Won,
    Draw,
    Lost,
}

pub type Data = Vec<(Shape, Shape)>;
const DATE: utils::Date = (2022, 02);

pub fn parse_data(input: utils::Input) -> Data {
    input
        .lines()
        .map(|line| {
            let mut it = line.trim().split_whitespace().take(2);

            let a = it.next().unwrap();
            let b = it.next().unwrap();

            let a = match a {
                "A" => Shape::Rock,
                "B" => Shape::Paper,
                "C" => Shape::Scissor,
                _ => unreachable!(),
            };

            let b = match b {
                "X" => Shape::Rock,
                "Y" => Shape::Paper,
                "Z" => Shape::Scissor,
                _ => unreachable!(),
            };

            (a, b)
        })
        .collect::<Vec<_>>()
}

const fn get_game_outcome(you: &Shape, opponent: &Shape) -> Outcome {
    match (you, opponent) {
        (Shape::Rock, Shape::Scissor) => Outcome::Won,
        (Shape::Paper, Shape::Rock) => Outcome::Won,
        (Shape::Scissor, Shape::Paper) => Outcome::Won,

        (Shape::Scissor, Shape::Rock) => Outcome::Lost,
        (Shape::Rock, Shape::Paper) => Outcome::Lost,
        (Shape::Paper, Shape::Scissor) => Outcome::Lost,

        _ => Outcome::Draw,
    }
}

fn make_it_end(opponent: &Shape, goal_outcome: &Outcome) -> Shape {
    let tests = [Shape::Rock, Shape::Paper, Shape::Scissor];

    for test in &tests {
        let outcome = get_game_outcome(test, opponent);

        if outcome == *goal_outcome {
            return *test;
        }
    }

    unreachable!()
}

fn get_points_for_game(you: &Shape, opponent: &Shape) -> usize {
    let starting_points = match you {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissor => 3,
    };

    let match_points = match get_game_outcome(you, opponent) {
        Outcome::Won => 6,
        Outcome::Draw => 3,
        Outcome::Lost => 0,
    };

    let score = starting_points + match_points;

    score
}

pub fn part_1(input: utils::Input) -> String {
    let data = parse_data(input);

    let points = data
        .iter()
        .map(|(opponent, you)| get_points_for_game(you, opponent))
        .sum::<usize>();

    format!("{}", points)
}

pub fn part_2(input: utils::Input) -> String {
    let data = parse_data(input);

    let points = data
        .iter()
        .map(|(opponent, outcome)| {
            let outcome = match outcome {
                Shape::Rock => Outcome::Lost,
                Shape::Paper => Outcome::Draw,
                Shape::Scissor => Outcome::Won,
            };

            let you = make_it_end(opponent, &outcome);

            get_points_for_game(&you, opponent)
        })
        .sum::<usize>();

    format!("{}", points)
}

fn run_1() {
    #[rustfmt::skip]
    let tests_1 = [
        (1, Some("15")),
    ];

    let all_correct_1 = utils::test(part_1, DATE, 1, &tests_1);
    if all_correct_1 {
        let answer = utils::run(part_1, 1, DATE);
    }
}

fn run_2() {
    #[rustfmt::skip]
    let tests_2 = [
        (1, Some("12")),
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
