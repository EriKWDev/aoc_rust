#![allow(dead_code, unreachable_code, unused)]

use utils::*;

pub type Data = ((usize, usize), (usize, usize), usize, usize, Vec<Vec<u8>>);
pub const DATE: utils::Date = (2022, 12);

pub fn parse_data(input: utils::Input) -> Data {
    let (mut start_x, mut start_y) = (0, 0);
    let (mut end_x, mut end_y) = (0, 0);

    let mut grid = input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.as_bytes()
                .iter()
                .copied()
                .enumerate()
                .map(|(x, char_val)| match char_val {
                    b'S' => {
                        (start_x, start_y) = (x, y);
                        b'a'
                    }
                    b'E' => {
                        (end_x, end_y) = (x, y);
                        b'z'
                    }

                    _ => char_val,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let (w, h) = (grid[0].len(), grid.len());

    ((start_x, start_y), (end_x, end_y), w, h, grid)
}

pub fn search_for_end(
    grid: &[Vec<u8>],
    w: usize,
    h: usize,
    (start_x, start_y): (usize, usize),
    (end_x, end_y): (usize, usize),
) -> Option<usize> {
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut stack = VecDeque::from([((start_y, start_x), 0)]);

    const DELTAS: [(isize, isize); 4] = [(0, -1), (-1, 0), (0, 1), (1, 0)];

    while let Some(((y, x), len)) = stack.pop_front() {
        if (y, x) == (end_y, end_x) {
            return Some(len);
        }

        let coords_in_grid = DELTAS.iter().filter_map(|(dx, dy)| {
            let (new_x, new_y) = ((x as isize + dx) as usize, (y as isize + dy) as usize);
            let value = grid.get(new_y).and_then(|col| col.get(new_x));

            if let Some(&value) = value {
                Some((new_x, new_y, value))
            } else {
                None
            }
        });

        let current_char_value = grid[y][x];
        coords_in_grid.for_each(|(new_x, new_y, new_char_value)| {
            if current_char_value + 1 >= new_char_value && !visited[new_y][new_x] {
                visited[new_y][new_x] = true;
                stack.push_back(((new_y, new_x), len + 1));
            }
        });
    }

    None
}

pub fn part_1(input: utils::Input) -> String {
    let (start, end, w, h, grid) = parse_data(input);
    let result = search_for_end(&grid, w, h, start, end).unwrap_or(0);

    format!("{}", result)
}

pub fn part_2(input: utils::Input) -> String {
    let (start, end, w, h, grid) = parse_data(input);

    let a_coords = (0..h)
        .flat_map(|y| (0..w).into_iter().map(move |x| (x, y)))
        .filter(|(x, y)| grid[*y][*x] == b'a');

    let result = a_coords
        .filter_map(|(x, y)| search_for_end(&grid, w, h, (x, y), end))
        .min()
        .unwrap_or(0);

    format!("{}", result)
}

fn run_1() {
    #[rustfmt::skip]
    let tests_1 = [
        (1, Some("31")),
    ];

    let all_correct_1 = utils::test(part_1, DATE, 1, &tests_1);
    if all_correct_1 {
        let answer = utils::run(part_1, 1, DATE);
    }
}

fn run_2() {
    #[rustfmt::skip]
    let tests_2 = [
        (1, Some("29")),
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
