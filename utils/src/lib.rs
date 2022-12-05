use std::fs::read_to_string;
pub use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

pub type Date = (usize, usize);
pub type Input = String;

pub fn get_input(date: Date, suffix: &str) -> Option<Input> {
    let (year, day) = date;

    let path = format!("solutions/input/{:04}_{:02}/{}", year, day, suffix);

    read_to_string(&path).ok()
}

pub fn input_to_string(input: Input) -> String {
    let mut buf = String::new();

    input.lines().for_each(|line| {
        let line = line.trim();
        buf.push_str(&line);
        buf.push('\n');
    });

    buf
}

pub fn run<F, D>(part_function: F, part: usize, date: Date) -> D
where
    F: Fn(Input) -> D,
    D: std::fmt::Display,
{
    /*
        TODO(Erik): Automatically fetch data for the day
    */

    println!("");
    println!("== Running Part {} ==", part);

    let input = get_input(date, "input").unwrap();
    let answer = part_function(input);

    println!("");
    println!(" Answer:  {:}", answer);
    println!("");

    answer
}

pub fn test<F, D>(
    part_function: F,
    date: Date,
    part: usize,
    tests: &[(usize, Option<&str>)],
) -> bool
where
    F: Fn(Input) -> D,
    D: std::fmt::Display + std::cmp::Eq,
{
    let mut all_correct = true;

    println!("== Running Tests for Part {} ==", part);
    println!("");

    for (test_index, (test_id, expected_maybe)) in tests.iter().enumerate() {
        let input = get_input(date, &format!("test_{:02}", test_id));

        if let Some(input) = input {
            let answer = part_function(input);
            let answer_text = format!("{}", answer);

            print!(
                " Test {}/{} (test_{:02}):  {}",
                test_index + 1,
                tests.len(),
                test_id,
                answer_text
            );
            if let Some(expected) = expected_maybe.as_ref() {
                let expected = format!("{}", expected);

                let is_correct = answer_text == expected;
                let is_correct_text = if is_correct {
                    "YES".to_owned()
                } else {
                    format!("NO, should be: {}", expected)
                };
                print!("  Correct: {}", is_correct_text);

                if !is_correct {
                    all_correct = false;
                }
            } else {
                print!("  Correct: ?");
            }
            println!("");
        } else {
            print!(
                " Test {}/{} (test_{:02}):  Could not get",
                test_index + 1,
                tests.len(),
                test_id,
            );
        }
    }

    all_correct
}
