#![allow(dead_code, unreachable_code, unused)]

use utils::*;

pub type Point = (isize, isize);
pub type Data = ([&'static [Point]; 5], [Point; 5], Vec<isize>);
pub const DATE: utils::Date = (2022, 17);

pub fn parse_data(input: utils::Input) -> Data {
    const SHAPES: [&[Point]; 5] = [
        &[(0, 0), (1, 0), (2, 0), (3, 0)],
        &[(1, 0), (0, -1), (1, -1), (2, -1), (1, -2)],
        &[(2, 0), (2, -1), (0, -2), (1, -2), (2, -2)],
        &[(0, 0), (0, -1), (0, -2), (0, -3)],
        &[(0, 0), (1, 0), (0, -1), (1, -1)],
    ];

    const SIZES: [Point; 5] = [(3, 0), (2, 2), (2, 2), (0, 3), (1, 1)];

    let jet_dx = input
        .trim()
        .chars()
        .map(|char| match char {
            '>' => 1,
            '<' => -1,
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();

    (SHAPES, SIZES, jet_dx)
}

pub fn rock_step(
    map: &mut HashSet<Point>,
    (x, y): &mut Point,
    shape: &[Point],
    (w, h): Point,
    jet_dx: isize,
) -> bool {
    // dbg!(jet_dx);
    let new_x = *x + jet_dx;
    if new_x + w < 7 && new_x >= 0 {
        let any = shape
            .iter()
            .any(|&(px, py)| map.contains(&(new_x + px, *y + py)));

        if !any {
            *x = new_x;
        }
    }

    let new_y = *y - 1;
    let any = shape
        .iter()
        .any(|&(px, py)| map.contains(&(*x + px, new_y + py)));

    if !any {
        *y = new_y;
        return true;
    }

    false
}

pub fn print_map(map: &HashSet<Point>, max_y: isize, shape: &[Point], point: Point) {
    for y in (-1..max_y).into_iter().rev() {
        for x in 0..7 {
            let any = shape
                .iter()
                .any(|&(px, py)| x == point.0 + px && y == point.1 + py);

            if any {
                print!("@");
            } else if map.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

pub fn simulation((shapes, shape_sizes, jet_dxs): Data, n: usize) -> isize {
    let mut max_y = 0;

    let mut map = HashSet::new();
    let mut cache = HashMap::new();

    for i in 0..7 {
        map.insert((i, -1));
    }

    let mut jet_i = 0;

    let mut i = 0;
    while i < n {
        let shape_index = i % shapes.len();

        let shape = &shapes[shape_index];
        let shape_size = shape_sizes[shape_index];

        let mut point = (2, max_y + 3 + shape_size.1 + if i == 0 { 0 } else { 1 });

        loop {
            let should_continue =
                rock_step(&mut map, &mut point, shape, shape_size, jet_dxs[jet_i]);

            jet_i = (jet_i + 1) % jet_dxs.len();

            if !should_continue {
                break;
            }
        }

        shape.iter().for_each(|&(px, py)| {
            let (x, y) = (point.0 + px, point.1 + py);

            max_y = max_y.max(y);
            map.insert((x, y));
        });

        let key = (i % shapes.len(), jet_i % jet_dxs.len(), max_y);
        if let Some((idx, height)) = cache.get(&key) {
            let repeats = (n - idx) / (i - idx) - 1;
            i += (i - idx) * repeats;
            // total_height += (get_height(&map) - height) * repeats;
        } else {
            cache.insert(key, (i, max_y));
        }

        i += 1;
    }

    max_y + 1
}

pub fn part_1(input: utils::Input) -> String {
    let data = parse_data(input);

    const N: usize = 2022;
    let result = simulation(data, N);

    format!("{}", result)
}

pub fn part_2(input: utils::Input) -> String {
    let data = parse_data(input);

    const N: usize = 1000000000000;
    let result = simulation(data, N);

    format!("{}", result)
}

fn run_1() {
    #[rustfmt::skip]
    let tests_1 = [
        (1, Some("3068")),
    ];

    let all_correct_1 = utils::test(part_1, DATE, 1, &tests_1);
    if all_correct_1 {
        let answer = utils::run(part_1, 1, DATE);
    }
}

fn run_2() {
    #[rustfmt::skip]
    let tests_2 = [
        (1, Some("1514285714288")),
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
