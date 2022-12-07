#![allow(dead_code, unreachable_code, unused)]

use utils::*;

pub type Data = Vec<Entry>;
pub const DATE: utils::Date = (2022, 07);

#[derive(Debug, Clone)]
pub enum Entry {
    File {
        size: usize,
        name: String,
        parent_index: usize,
    },
    Folder {
        children: Vec<usize>,
        parent_index: usize,
        name: String,
    },
}

pub fn parse_data(input: utils::Input) -> Data {
    let mut files_and_folders = vec![];

    let mut current_index = 0;

    for line in input.lines() {
        let mut it = line.split_whitespace();
        let first = it.next().unwrap();

        if first == "$" {
            let command = it.next().unwrap();

            if command == "cd" {
                let path = it.next().unwrap();

                if path == ".." {
                    let Entry::Folder { parent_index, .. } = &files_and_folders[current_index] else { panic!() };
                    current_index = *parent_index;
                } else {
                    let new_path = path.to_owned();

                    if !files_and_folders.is_empty() {
                        let Entry::Folder {
                        parent_index,
                        children,
                        ..
                    } = &files_and_folders[current_index] else { panic!() }                        ;
                        for child_index in children {
                            if let Entry::Folder { name, .. } = &files_and_folders[*child_index] {
                                if path == name {
                                    current_index = *child_index;
                                }
                            }
                        }
                    } else {
                        files_and_folders.push(Entry::Folder {
                            children: vec![],
                            parent_index: 0,
                            name: "/".to_owned(),
                        });
                    }
                }
            }
        } else {
            let dir_or_size = first;
            let entry = it.next().unwrap().to_owned();

            if dir_or_size == "dir" {
                files_and_folders.push(Entry::Folder {
                    children: vec![],
                    parent_index: current_index,
                    name: entry,
                });
            } else {
                let size = dir_or_size.trim().parse().unwrap();
                files_and_folders.push(Entry::File {
                    name: entry,
                    size,
                    parent_index: current_index,
                });
            }

            let index = files_and_folders.len() - 1;

            let Entry::Folder { children, .. } = &mut files_and_folders[current_index] else { panic!() };
            children.push(index);
        }
    }

    files_and_folders
}

pub fn size_of_index(files_and_folders: &Data, index: usize) -> Option<usize> {
    let mut checked = HashSet::<usize>::new();
    let mut buffer = vec![];

    let mut current = index;
    let Entry::Folder { children, .. } = &files_and_folders[current]  else { return None };
    buffer.extend(children.iter().cloned());

    let mut total = 0;
    while !buffer.is_empty() {
        let current = buffer.pop().unwrap();
        let could_add = checked.insert(current);

        if could_add {
            match &files_and_folders[current] {
                Entry::Folder { children, .. } => {
                    for child in children {
                        if !checked.contains(child) {
                            buffer.push(child.clone());
                        }
                    }
                }

                Entry::File { size, .. } => {
                    total += *size;
                }
            }
        }
    }

    return Some(total);
}

pub fn part_1(input: utils::Input) -> String {
    let data = parse_data(input);

    const MIN_SIZE: usize = 100_000;

    let result = (0..data.len())
        .into_iter()
        .filter_map(|i| (size_of_index(&data, i)))
        .filter(|size| *size <= MIN_SIZE)
        .sum::<usize>();

    format!("{}", result)
}

pub fn part_2(input: utils::Input) -> String {
    let data = parse_data(input);
    let used = size_of_index(&data, 0).unwrap();

    const TOTAL: usize = 70000000;
    const REQUIRED: usize = 30000000;

    let free = TOTAL - used;
    let min_to_delete = REQUIRED - free;

    let result = (0..data.len())
        .into_iter()
        .filter_map(|i| (size_of_index(&data, i)))
        .filter(|size| *size >= min_to_delete)
        .min()
        .unwrap_or(0);

    format!("{}", result)
}

fn run_1() {
    #[rustfmt::skip]
    let tests_1 = [
        (1, Some("95437")),
    ];

    let all_correct_1 = utils::test(part_1, DATE, 1, &tests_1);
    if all_correct_1 {
        let answer = utils::run(part_1, 1, DATE);
    }
}

fn run_2() {
    #[rustfmt::skip]
    let tests_2 = [
        (1, Some("24933642")),
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
