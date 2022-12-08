#![allow(dead_code, unreachable_code, unused)]

use utils::*;

pub type Data = (usize, usize, Vec<Vec<usize>>);
pub const DATE: utils::Date = (2022, 08);

pub fn parse_data(input: utils::Input) -> Data {
    let grid = input
        .lines()
        .filter_map(|line| {
            let res = line
                .trim()
                .chars()
                .map(|char| char.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>();

            if res.is_empty() {
                None
            } else {
                Some(res)
            }
        })
        .collect::<Vec<_>>();

    let (width, height) = (grid[0].len(), grid.len());
    (width, height, grid)
}

pub fn is_visible(width: usize, height: usize, grid: &Vec<Vec<usize>>, x: usize, y: usize) -> bool {
    if x >= width || y >= height {
        return false;
    }

    if x == 0 || y == 0 || x == width - 1 || y == height - 1 {
        return true;
    }

    let center_value = grid[y][x];

    let any_taller_right = (x + 1..width)
        .into_iter()
        .map(|i| {
            let value = grid[y][i];
            value >= center_value
        })
        .any(|p| p);

    let any_taller_left = (0..x)
        .into_iter()
        .map(|i| {
            let value = grid[y][i];
            value >= center_value
        })
        .any(|p| p);

    let any_taller_up = (y + 1..height)
        .into_iter()
        .map(|i| {
            let value = grid[i][x];
            value >= center_value
        })
        .any(|p| p);

    let any_taller_down = (0..y)
        .into_iter()
        .map(|i| {
            let value = grid[i][x];
            value >= center_value
        })
        .any(|p| p);

    if any_taller_down && any_taller_up && any_taller_left && any_taller_right {
        return false;
    }

    return true;
}

pub fn scenic_score(
    width: usize,
    height: usize,
    grid: &Vec<Vec<usize>>,
    x: usize,
    y: usize,
) -> usize {
    let center_value = grid[y][x];

    let mut right = 0;
    for i in x + 1..width {
        right += 1;
        if grid[y][i] >= center_value {
            break;
        }
    }

    let mut left = 0;
    for i in (0..x).rev() {
        left += 1;
        if grid[y][i] >= center_value {
            break;
        }
    }

    let mut up = 0;
    for i in (0..y).rev() {
        up += 1;
        if grid[i][x] >= center_value {
            break;
        }
    }

    let mut down = 0;
    for i in y + 1..height {
        down += 1;
        if grid[i][x] >= center_value {
            break;
        }
    }

    return right * left * up * down;
}

pub fn part_1(input: utils::Input) -> String {
    let (width, height, grid) = parse_data(input);

    let total = (0..height)
        .into_iter()
        .map(|y| {
            (0..width)
                .into_iter()
                .filter(|x| is_visible(width, height, &grid, *x, y))
                .count()
        })
        .sum::<usize>();

    format!("{}", total)
}

pub fn part_2(input: utils::Input) -> String {
    let (width, height, grid) = parse_data(input);

    let result = (0..height)
        .into_iter()
        .map(|y| {
            (0..width)
                .into_iter()
                .map(|x| scenic_score(width, height, &grid, x, y))
                .max()
                .unwrap_or(0)
        })
        .max()
        .unwrap_or(0);

    format!("{}", result)
}

fn run_1() {
    #[rustfmt::skip]
    let tests_1 = [
        (1, Some("21")),
    ];

    let all_correct_1 = utils::test(part_1, DATE, 1, &tests_1);
    if all_correct_1 {
        let answer = utils::run(part_1, 1, DATE);
    }
}

fn run_2() {
    #[rustfmt::skip]
    let tests_2 = [
        (1, Some("8")),
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
