#![allow(dead_code, unreachable_code, unused)]

use utils::*;

pub const DATE: utils::Date = (2023, 3);

pub fn part_1(input: utils::Input) -> String {
    let mut result = 0;

    let mut lines = input
        .lines()
        .map(|line| format!(".{line}.").chars().collect::<Vec<_>>())
        .collect::<std::collections::VecDeque<_>>();

    let n = lines[0].len();

    lines.push_front(vec!['.'; n]);
    lines.push_back(vec!['.'; n]);

    for (y, chars) in lines.iter().enumerate() {
        if y < 1 || y > lines.len() - 1 {
            continue;
        }

        let mut streak = 1;
        let mut current = 0;
        let mut added = true;
        let mut is_part = false;

        for (x, c) in chars.iter().enumerate().rev() {
            if x < 1 || x > chars.len() - 1 {
                continue;
            }

            if c.is_numeric() {
                if !is_part {
                    for (px, py) in [
                        (x - 1, y - 1),
                        (x - 1, y + 1),
                        (x, y - 1),
                        (x, y + 1),
                        (x - 1, y),
                        (x + 1, y),
                        (x + 1, y - 1),
                        (x + 1, y + 1),
                    ] {
                        let c = lines[py][px];
                        if !c.is_numeric() && c != '.' {
                            is_part = true;
                        }
                    }
                }

                let n = c.to_digit(10).unwrap() as usize;

                added = false;
                current += n * streak;
                streak *= 10;
            } else {
                if !added {
                    if is_part {
                        result += current;
                    }

                    is_part = false;
                    added = true;
                    streak = 1;
                    current = 0;
                }
            }
        }

        if !added && is_part {
            result += current;
        }
    }

    format!("{}", result)
}

pub fn part_2(input: utils::Input) -> String {
    let mut lines = input
        .lines()
        .map(|line| format!(".{line}.").chars().collect::<Vec<_>>())
        .collect::<std::collections::VecDeque<_>>();

    let n = lines[0].len();

    lines.push_front(vec!['.'; n]);
    lines.push_back(vec!['.'; n]);

    let mut gears = std::collections::HashMap::new();

    for (y, chars) in lines.iter().enumerate() {
        if y < 1 || y > lines.len() - 1 {
            continue;
        }

        let mut streak = 1;
        let mut current = 0;
        let mut handled = true;
        let mut is_part = false;
        let mut gear = None;

        for (x, c) in chars.iter().enumerate().rev() {
            if x < 1 || x > chars.len() - 1 {
                continue;
            }

            if c.is_numeric() {
                for (px, py) in [
                    (x - 1, y - 1),
                    (x - 1, y + 1),
                    (x, y - 1),
                    (x, y + 1),
                    (x - 1, y),
                    (x + 1, y),
                    (x + 1, y - 1),
                    (x + 1, y + 1),
                ] {
                    let c = lines[py][px];
                    if !c.is_numeric() && c != '.' {
                        is_part = true;

                        if c == '*' {
                            gears.entry((px, py)).or_insert(vec![]);
                            gear.replace((px, py));
                        }
                    }
                }

                let n = c.to_digit(10).unwrap() as usize;

                handled = false;
                current += n * streak;
                streak *= 10;
            } else {
                if !handled {
                    gear.take().map(|p| {
                        gears.get_mut(&p).unwrap().push(current);
                    });

                    is_part = false;
                    handled = true;
                    streak = 1;
                    current = 0;
                }
            }
        }

        if !handled {
            gear.take().map(|p| {
                gears.get_mut(&p).unwrap().push(current);
            });
        }
    }

    let mut result = 0;

    for ratio in gears.into_values() {
        if ratio.len() > 1 {
            result += ratio.iter().product::<usize>();
        }
    }

    format!("{}", result)
}

fn run_1() {
    #[rustfmt::skip]
    let tests_1 = [
        (1, Some("4361")),
    ];

    let all_correct_1 = utils::test(part_1, DATE, 1, &tests_1);
    if all_correct_1 {
        let answer = utils::run(part_1, 1, DATE);

        if ["503901", "503999", "537164"].contains(&answer.as_str()) {
            println!("Correct: NO");
            println!("");
        }
    }
}

fn run_2() {
    #[rustfmt::skip]
    let tests_2 = [
        (1, Some("467835")),
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
