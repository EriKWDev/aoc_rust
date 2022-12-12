pub use std::{
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fs::{read_to_string, File},
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
    let max_time = max_time
        .into()
        .unwrap_or(std::time::Duration::from_secs(10));

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
