#![allow(dead_code, unreachable_code, unused)]

use utils::*;

pub type Map = HashSet<(isize, isize)>;
pub type Data = (isize, Map);

pub const DATE: utils::Date = (2022, 14);

pub fn parse_data(input: utils::Input) -> Data {
    let mut buf2 = String::new();

    let line_coords = input
        .lines()
        .map(|line| {
            let mut chars = line.chars().peekable();
            let mut buf = vec![];

            while let Some(char) = chars.next() {
                if char.is_ascii_digit() {
                    buf2.push(char);
                    while let Some(nc) = chars.peek() {
                        if nc.is_ascii_digit() {
                            buf2.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }

                    buf.push(buf2.parse::<isize>().unwrap());
                    buf2.clear();
                }
            }

            buf.chunks(2).map(|w| (w[0], w[1])).collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>();

    let mut map = Map::new();

    let mut lowest = isize::MIN;

    for line in line_coords {
        line.windows(2)
            .map(|w| {
                let ((a_x, a_y), (b_x, b_y)) = (w[0], w[1]);

                let sx = a_x.min(b_x);
                let sy = a_y.min(b_y);

                let ex = b_x.max(a_x);
                let ey = b_y.max(a_y);

                ((sx, sy), (ex, ey))
            })
            .for_each(|((sx, sy), (ex, ey))| {
                for x in sx..=ex {
                    map.insert((x as isize, sy as isize));
                }

                for y in sy..=ey {
                    map.insert((ex as isize, y as isize));
                    lowest = lowest.max(y as isize);
                }
            });
    }

    (lowest, map)
}

pub fn sand_sim_step(lowest: isize, map: &Map, mut sand: (isize, isize)) -> Option<(isize, isize)> {
    while sand.1 < lowest {
        let old_sand = sand;
        sand.1 += 1;

        if map.contains(&sand) {
            sand.0 -= 1;
        } else {
            continue;
        }

        if map.contains(&sand) {
            sand.0 += 2;
        } else {
            continue;
        }

        if map.contains(&sand) {
            return Some(old_sand);
        } else {
            continue;
        }
    }

    None
}

pub fn part_1(input: utils::Input) -> String {
    let (lowest, mut map) = parse_data(input);

    let mut result = 0;
    while let Some(sand_pos) = sand_sim_step(lowest, &map, (500, 0)) {
        result += 1;
        map.insert(sand_pos);
    }

    format!("{}", result)
}

pub fn part_2(input: utils::Input) -> String {
    let (lowest, mut map) = parse_data(input);
    let lowest = lowest + 2;

    for x in -1000..1000 {
        map.insert((x, lowest));
    }

    let mut result = 0;
    while let Some(sand_pos) = sand_sim_step(lowest, &map, (500, 0)) {
        result += 1;

        if sand_pos == (500, 0) {
            break;
        }

        map.insert(sand_pos);
    }

    format!("{}", result)
}

fn run_1() {
    #[rustfmt::skip]
    let tests_1 = [
        (1, Some("24")),
    ];

    let all_correct_1 = utils::test(part_1, DATE, 1, &tests_1);
    if all_correct_1 {
        let answer = utils::run(part_1, 1, DATE);
    }
}

fn run_2() {
    #[rustfmt::skip]
    let tests_2 = [
        (1, Some("93")),
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
