#![allow(dead_code, unreachable_code, unused)]

use utils::*;

type Point = (isize, isize, isize);
pub type Data = HashSet<Point>;
pub const DATE: utils::Date = (2022, 18);

pub fn parse_data(input: utils::Input) -> Data {
    input
        .lines()
        .filter_map(|line| {
            let mut nums = line.trim().split(',').map(|n| n.parse().unwrap());

            Some((nums.next()?, nums.next()?, nums.next()?))
        })
        .collect::<HashSet<_>>()
}

const DIRS: [Point; 6] = [
    (0, 0, 1),
    (0, 1, 0),
    (1, 0, 0),
    (0, 0, -1),
    (0, -1, 0),
    (-1, 0, 0),
];

pub fn part_1(input: utils::Input) -> String {
    let data = parse_data(input);

    let result = data
        .iter()
        .flat_map(|&(x, y, z)| {
            DIRS.iter()
                .map(move |&(dx, dy, dz)| (x + dx, y + dy, z + dz))
        })
        .filter(|point| !data.contains(point))
        .count();

    format!("{}", result)
}

pub fn part_2(input: utils::Input) -> String {
    let mut data = parse_data(input);

    let ((max_x, max_y, max_z), (min_x, min_y, min_z)) = data.iter().fold(
        (
            (isize::MIN, isize::MIN, isize::MIN),
            (isize::MAX, isize::MAX, isize::MAX),
        ),
        |(max_acc, min_acc), &(x, y, z)| {
            (
                (max_acc.0.max(x), max_acc.1.max(y), max_acc.2.max(z)),
                (min_acc.0.min(x), min_acc.1.min(y), min_acc.2.min(z)),
            )
        },
    );

    let mut water = HashSet::new();
    let mut stack = VecDeque::from([(min_x, min_y, min_z)]);

    while let Some((x, y, z)) = stack.pop_front() {
        if water.contains(&(x, y, z)) {
            continue;
        }

        water.insert((x, y, z));

        let points = DIRS.iter().filter_map(|&(dx, dy, dz)| {
            let (nx, ny, nz) = (x + dx, y + dy, z + dz);
            if !(min_x <= nx && nx <= max_x) {
                None
            } else if !(min_y <= ny && ny <= max_y) {
                None
            } else if !(min_z <= nz && nz <= max_z) {
                None
            } else if data.contains(&(nx, ny, nz)) {
                None
            } else {
                Some((nx, ny, nz))
            }
        });

        stack.extend(points);
    }

    let lava = (min_x..=max_x)
        .flat_map(|x| (min_y..=max_y).flat_map(move |y| (min_z..=max_z).map(move |z| (x, y, z))))
        .filter(|point| !water.contains(point))
        .collect::<HashSet<_>>();

    let result = lava
        .iter()
        .flat_map(|&(x, y, z)| {
            DIRS.iter()
                .map(move |&(dx, dy, dz)| (x + dx, y + dy, z + dz))
        })
        .filter(|point| !lava.contains(point))
        .count();

    format!("{}", result)
}

fn run_1() {
    #[rustfmt::skip]
    let tests_1 = [
        (2, Some("10")),
        (1, Some("64")),
    ];

    let all_correct_1 = utils::test(part_1, DATE, 1, &tests_1);
    if all_correct_1 {
        let answer = utils::run(part_1, 1, DATE);
    }
}

fn run_2() {
    #[rustfmt::skip]
    let tests_2 = [
        (1, Some("58")),
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
