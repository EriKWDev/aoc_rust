pub use std::{
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fs::{read_to_string, File},
    io::{BufRead, BufReader},
};

pub type Date = (usize, usize);
pub type Input = String;

pub fn date_from_file_name(name: &str) -> Date {
    let (_p, rest) = name.split_once("p").unwrap();
    let (year, rest) = rest.split_once("_").unwrap();
    let (day, _rest) = rest.split_once(".").unwrap();

    (year.parse().unwrap(), day.parse().unwrap())
}

#[rustfmt::skip]
pub fn char_to_n(c: char) -> usize {
    match c { 
        '0' => 0, '1' => 1, '2' => 2, '3' => 3, '4' => 4, '5' => 5, '6' => 6, '7' => 7, '8' => 8, '9' => 9,
        _ => unreachable!(),
    }
}

pub fn get_input(date: Date, suffix: &str) -> Option<Input> {
    let (year, day) = date;

    let path = format!("solutions/input/{:04}_{:02}/{}", year, day, suffix);

    read_to_string(&path).ok()
}

pub fn run<F, D>(part_function: F, part: usize, date: Date) -> D
where
    F: Fn(Input) -> D,
    D: std::fmt::Display,
{
    println!("");
    println!("== Running Part {} ==", part);

    let input = get_input(date, "input").unwrap();
    let answer = part_function(input);

    println!("");
    println!(" Answer:  {:}", answer);
    println!("");

    answer
}

pub struct BenchResult {
    pub total: std::time::Duration,
    pub average: std::time::Duration,
    pub times: usize,
    pub max_iterations: usize,
    pub date: Date,
    pub part: usize,
}

pub fn bench<F, D, O>(part_function: F, part: usize, date: Date, max_time: O) -> BenchResult
where
    F: Fn(Input) -> D,
    D: std::fmt::Display,
    O: Into<Option<std::time::Duration>>,
{
    let max_time = max_time.into().unwrap_or(std::time::Duration::MAX);

    let max_iterations = 10_000;

    /*
        NOTE: Opening input files and parsing that into a String is not
              considered part of the bench. Parsing that String to usable
              data is, however, of course included in the timings.
    */
    let input = get_input(date, "input").unwrap();

    let now = std::time::Instant::now();

    let mut timings = vec![];
    let mut times = 0;

    print!(
        "[ INFO ] Benchmarking {}_{:02} part {}",
        date.0, date.1, part
    );

    while now.elapsed() < max_time && times < max_iterations {
        times += 1;
        let start = std::time::Instant::now();
        let _ = part_function(input.clone());
        let time_taken = start.elapsed();
        timings.push(time_taken);
    }

    let time_taken = now.elapsed();
    let total = timings.iter().sum::<std::time::Duration>();
    let average = total.div_f64(times as f64);

    println!(
        " {:?} ({} / {}, took: {:?})",
        average, times, max_iterations, time_taken
    );

    BenchResult {
        total,
        times,
        max_iterations,
        average,
        date,
        part,
    }
}

pub fn summarize_results(tests: &[BenchResult]) {
    tests.iter().for_each(|result| {
        println!(
            "{:04}_{:02} part {} .......... {:?} (ran {}/{})",
            result.date.0,
            result.date.1,
            result.part,
            result.average,
            result.times,
            result.max_iterations
        );
    });
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
                "[ {:02}/{:02} (test_{:02}) ] output: {}",
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
                print!("\t\tCorrect: {}", is_correct_text);

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

#[inline]
pub const fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[inline]
pub const fn lcm(a: usize, b: usize) -> usize {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b) / gcd(a, b)
    }
}

#[inline]
pub fn lcm_of_slice(numbers: &[usize]) -> usize {
    let mut result = numbers[0];

    for &num in numbers.iter().skip(1) {
        result = lcm(result, num);
    }

    result
}
