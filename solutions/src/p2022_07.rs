#![allow(dead_code, unreachable_code, unused)]

use utils::*;

pub type Data = HashMap<String, (Vec<(usize, String)>, Vec<String>)>;
pub const DATE: utils::Date = (2022, 07);

pub fn parse_data(input: utils::Input) -> Data {
    let mut current_buf = vec![];
    let mut current_path_children = vec![];
    let mut current_path: Vec<String> = vec![];

    input
        .lines()
        .filter_map(|line| {
            let mut it = line.split_whitespace();

            if line.starts_with("$") {
                let mut it = it.skip(1);
                let new_command = it.next().unwrap();

                match new_command {
                    "cd" => {
                        let next_path = it.next().unwrap();
                        let key;

                        if next_path == ".." {
                            key = Some(current_path.last().unwrap().to_owned());

                            current_path.pop();
                        } else {
                            if current_path.is_empty() {
                                key = None;
                            } else {
                                key = Some(current_path.last().unwrap().to_owned());
                            }

                            current_path.push(next_path.to_owned());
                        }

                        if let Some(key) = key {
                            if current_buf.is_empty() && current_path_children.is_empty() {
                                None
                            } else {
                                Some((
                                    key,
                                    (
                                        current_buf.drain(..).collect(),
                                        current_path_children.drain(..).collect(),
                                    ),
                                ))
                            }
                        } else {
                            None
                        }
                    }

                    _ => None,
                }
            } else {
                let size_or_dir = it.next().unwrap().trim();
                let file_or_path = it.next().unwrap().to_owned();

                if size_or_dir == "dir" {
                    current_path_children.push(file_or_path);
                } else {
                    let size = size_or_dir.parse().unwrap();
                    current_buf.push((size, file_or_path));
                }

                None
            }
        })
        .collect()
}

pub fn part_1(input: utils::Input) -> String {
    let data = parse_data(input);

    let dir_size_lookup = data
        .iter()
        .map(|(path, (files, _))| (path, files.iter().map(|(size, file)| *size).sum::<usize>()))
        .collect::<HashMap<_, _>>();

    let dirs = data.keys().cloned().collect::<Vec<String>>();
    let result = dirs
        .iter()
        .filter_map(|path| {
            let mut size = *dir_size_lookup.get(path).unwrap();
            let (files, children) = data.get(path).unwrap();

            let mut checked = HashSet::from([path.clone()]);
            let mut buffer = children.clone();

            while !buffer.is_empty() {
                let current = buffer.pop().unwrap();
                let could_insert = checked.insert(current.clone());

                if could_insert {
                    if let Some(new_size) = dir_size_lookup.get(&current) {
                        size += *new_size;

                        let (files, children) = data.get(&current).unwrap();
                        for child in children {
                            if !checked.contains(child) {
                                buffer.push(child.clone())
                            }
                        }
                    }
                }
            }

            if size <= 100_000 {
                Some(size)
            } else {
                None
            }
        })
        .sum::<usize>();

    format!("{}", result)
}

pub fn part_2(input: utils::Input) -> String {
    let data = parse_data(input);

    format!("{}", 0)
}

fn run_1() {
    #[rustfmt::skip]
    let tests_1 = [
        (1, Some("95437")),
    ];

    let all_correct_1 = utils::test(part_1, DATE, 1, &tests_1);
    if all_correct_1 {
        let answer = utils::run(part_1, 1, DATE);

        match &answer as &str {
            "1138249" | "" | "0" => println!(" WRONG\n"),

            _ => {}
        }
    }
}

fn run_2() {
    #[rustfmt::skip]
    let tests_2 = [
        (1, Some("1")),
    ];

    let all_correct_2 = utils::test(part_2, DATE, 2, &tests_2);
    if all_correct_2 {
        let answer = utils::run(part_2, 2, DATE);
    }
}

fn main() {
    run_1();
    // run_2();
}
