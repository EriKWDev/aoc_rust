#![allow(dead_code, unreachable_code, unused)]

use utils::*;

pub type Data = (HashMap<(usize, usize), (usize, usize)>, Vec<Vec<usize>>);

pub fn parse_data(input: utils::Input) -> Data {
    let Some((a, b)) = input.split_once("\n\n") else {
        panic!()
    };

    let mut rules = HashMap::new();
    for (k, v) in a
        .lines()
        .map(|line| line.trim().split_once('|').unwrap())
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
    {
        rules.insert((k, v), (k, v));
        rules.insert((v, k), (k, v));
    }

    let updates = b
        .lines()
        .map(|line| line.trim().split(',').map(|n| n.parse().unwrap()).collect())
        .collect();

    (rules, updates)
}

pub fn make_ok(job: &mut Vec<usize>, map: &HashMap<usize, Vec<usize>>, printed: &mut Vec<usize>) {
    printed.clear();

    loop {
        let mut ok = true;

        'job: for i in 0..job.len() {
            let c = job[i];
            let Some(e) = map.get(&c) else {
                printed.push(c);
                continue;
            };

            for &j in e {
                if printed.contains(&j) {
                    let c = job.remove(i);
                    job.insert(0, c);
                    ok = false;
                    printed.clear();
                    break 'job;
                }
            }

            printed.push(c);
        }

        if ok {
            break;
        }
    }
}

pub fn is_ok(
    job: &[usize],
    rules: &HashMap<(usize, usize), (usize, usize)>,
    map: &mut HashMap<usize, Vec<usize>>,
    printed: &mut Vec<usize>,
) -> bool {
    printed.clear();
    map.clear();

    for i in 0..job.len() {
        let a = job[i];
        for j in i + 1..job.len() {
            let b = job[j];

            if let Some((a, b)) = rules.get(&(a, b)).copied() {
                let e = map.entry(a).or_insert_with(|| vec![]);
                e.push(b);
            }
        }
    }

    let mut ok = true;
    'job: for i in 0..job.len() {
        let c = job[i];
        let Some(e) = map.get(&c) else {
            printed.push(c);
            continue;
        };

        for &j in e {
            if printed.contains(&j) {
                ok = false;
                break 'job;
            }
        }

        printed.push(c);
    }

    ok
}

pub fn part_1(input: utils::Input) -> String {
    let (rules, updates) = parse_data(input);

    let mut result = 0;

    let mut printed = vec![];
    let mut map = HashMap::default();
    'jobs: for job in updates {
        let ok = is_ok(&job, &rules, &mut map, &mut printed);

        if ok {
            result += job[job.len() / 2];
        }
    }

    format!("{}", result)
}

pub fn part_2(input: utils::Input) -> String {
    let (rules, updates) = parse_data(input);

    let mut result = 0;

    let mut printed = vec![];
    let mut map = HashMap::default();
    'jobs: for mut job in updates {
        let ok = is_ok(&job, &rules, &mut map, &mut printed);
        if !ok {
            make_ok(&mut job, &map, &mut printed);
            result += job[job.len() / 2];
        }
    }

    format!("{}", result)
}

pub fn run_1(date: utils::Date) {
    #[rustfmt::skip]
    let tests_1 = [
        (1, Some("143")),
    ];

    let all_correct_1 = utils::test(part_1, date, 1, &tests_1);
    if all_correct_1 {
        let answer = utils::run(part_1, 1, date);
    }
}

pub fn run_2(date: utils::Date) {
    #[rustfmt::skip]
    let tests_2 = [
        (1, Some("123")),
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
