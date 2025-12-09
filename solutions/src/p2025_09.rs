#![allow(dead_code, unreachable_code, unused)]

use utils::*;

pub type Tile = (usize, usize);
pub type Data = Vec<Tile>;

pub fn parse_data(input: utils::Input) -> Data {
    input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(',').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect::<Vec<_>>()
}

#[inline(always)]
pub fn area((x0, y0): Tile, (x1, y1): Tile) -> i128 {
    ((x0 as i128 - x1 as i128).abs() + 1) * ((y0 as i128 - y1 as i128).abs() + 1)
}

#[inline(always)]
pub fn in_line((x0, y0): Tile, (x1, y1): Tile) -> bool {
    x0 == x1 || y0 == y1
}

pub fn part_1(input: utils::Input) -> String {
    let data = parse_data(input);

    if utils::is_test() {
        assert_eq!(area((2, 5), (9, 7)), 24);
        assert_eq!(area((2, 5), (11, 1)), 50);
        assert_eq!(area((7, 1), (11, 7)), 35);
        assert_eq!(area((7, 3), (2, 3)), 6);
    }

    let mut result = 0;
    let n = data.len();
    for i in 0..n {
        for j in i + 1..n - 1 {
            result = result.max(area(data[i], data[j]));
        }
    }

    format!("{}", result)
}

pub fn part_2(input: utils::Input) -> String {
    let data = parse_data(input);

    let n = data.len();

    let mut min_x = usize::MAX;
    let mut min_y = usize::MAX;
    let mut max_x = usize::MIN;
    let mut max_y = usize::MIN;

    for i in 0..n {
        let (x, y) = data[i];
        min_x = min_x.min(x);
        max_x = max_x.max(x);
        min_y = min_y.min(y);
        max_y = max_y.max(y);
    }

    let mut lines = vec![];
    for i in 0..n {
        for j in 0..n {
            let a = data[i];
            let b = data[j];
            if in_line(a, b) {
                lines.push((i, j));
            }
        }
    }

    let w = (max_x - min_x + 1);
    let h = (max_y - min_y + 1);

    let mut map = vec![0_u8; w * h];

    for (i, j) in lines {
        let (x0, y0) = data[i];
        let (x1, y1) = data[j];

        let mi_x = x0.min(x1);
        let mi_y = y0.min(y1);
        let ma_x = x0.max(x1);
        let ma_y = y0.max(y1);

        for x in mi_x..=ma_x {
            for y in mi_y..=ma_y {
                let x = x - min_x;
                let y = y - min_y;
                map[x + y * w] = 1;
            }
        }
    }

    let mut todo = vec![];
    'outer: for y in 0..h {
        let mut fill = 0;
        for x in 0..w {
            let v = map[x + y * w];
            if v == 1 {
                fill = (fill + 1) % 2;
            }
            if fill == 1 && v == 0 {
                todo.push((x, y));
                break 'outer;
            }
        }
    }

    for &(x, y) in &todo {
        map[x + y * w] = 8;
    }

    for i in 0..n {
        let (x, y) = data[i];
        let x = x - min_x;
        let y = y - min_y;
        map[x + y * w] = 4;
    }

    while let Some((x, y)) = todo.pop() {
        for (dx, dy) in [(1, 0), (-1, 0), (0, -1), (0, 1)] {
            let x = x as isize + dx;
            let y = y as isize + dy;
            if x < 0 || x >= w as isize || y < 0 || y >= h as isize {
                continue;
            }
            let x = x as usize;
            let y = y as usize;
            if map[x + y * w] != 0 {
                continue;
            }
            map[x + y * w] = 3;
            todo.push(((x, y)));
        }
    }

    let mut jobs = vec![];
    for i in 0..n {
        for j in i + 1..n - 1 {
            jobs.push((area(data[i], data[j]), i, j));
        }
    }
    jobs.sort_unstable();
    jobs.reverse();

    let mut result = 0;

    'next: for (r, i, j) in jobs {
        let (x0, y0) = data[i];
        let (x1, y1) = data[j];

        let mi_x = x0.min(x1);
        let mi_y = y0.min(y1);
        let ma_x = x0.max(x1);
        let ma_y = y0.max(y1);

        if r <= result {
            continue;
        }

        for x in mi_x..=ma_x {
            for y in mi_y..=ma_y {
                let x = x - min_x;
                let y = y - min_y;
                if map[x + y * w] == 0 {
                    continue 'next;
                }
            }
        }

        for x in mi_x..=ma_x {
            for y in mi_y..=ma_y {
                let x = x - min_x;
                let y = y - min_y;
                map[x + y * w] = 7;
            }
        }

        if r > result {
            result = r;
        }
    }

    if utils::is_test() {
        for y in 0..h {
            for x in 0..w {
                let v = map[x + y * w];
                if v == 0 {
                    print!(" ");
                    continue;
                }
                print!("{v}");
            }
            println!();
        }
        println!();
    } else {
        if std::env::args().any(|it| it == "--dump") {
            std::fs::write("out.bin", &map);
        }
    }

    format!("{}", result)
}

pub fn run_1(date: utils::Date) {
    #[rustfmt::skip]
    let tests_1 = [
        (1, Some("50")),
    ];

    let all_correct_1 = utils::test(part_1, date, 1, &tests_1);
    if all_correct_1 {
        let answer = utils::run(part_1, 1, date);
        let n: usize = answer.parse().unwrap();
        assert!(n > 2147436216);
    }
}

pub fn run_2(date: utils::Date) {
    #[rustfmt::skip]
    let tests_2 = [
        (1, Some("24")),
    ];

    let all_correct_2 = utils::test(part_2, date, 2, &tests_2);
    if all_correct_2 {
        let answer = utils::run(part_2, 2, date);
        let it: usize = answer.parse().unwrap();
        assert!(it > 2337984);
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
