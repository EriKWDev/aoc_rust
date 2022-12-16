#![allow(dead_code, unreachable_code, unused)]

use utils::*;

pub type Point = (isize, isize);
pub type Map = HashMap<Point, (Point, isize)>;
pub type Data = Map;

pub const DATE: utils::Date = (2022, 15);

pub fn parse_data(input: utils::Input) -> Data {
    let mut buf2 = String::new();
    let mut buf = vec![];

    input
        .lines()
        .map(|line| {
            let mut chars = line.chars().peekable();

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

            let a = (buf[0], buf[1]);
            let b = (buf[2], buf[3]);
            buf.clear();

            let d = manhattan_distance(a, b);

            (a, (b, d))
        })
        .collect::<HashMap<_, _>>()
}

#[inline(always)]
pub fn manhattan_distance((ax, ay): Point, (bx, by): Point) -> isize {
    (ax - bx).abs() + (ay - by).abs()
}

pub fn part_1(input: utils::Input) -> String {
    let map = parse_data(input);
    let sensors = map.keys().collect::<HashSet<_>>();

    const Y: isize = 2_000_000;

    let beacons = map
        .values()
        .map(|&(point, _)| point)
        .collect::<HashSet<_>>();

    let mut no_beacons = HashSet::new();
    map.iter()
        .for_each(|(&(sensor_x, sensor_y), &(beacon, manhattan))| {
            let distance = (sensor_y - Y).abs();

            let (mut x, mut dist_right) = (sensor_x, distance);
            while dist_right <= manhattan {
                no_beacons.insert((x, Y));
                dist_right += 1;
                x += 1;
            }

            let (mut x, mut dist_left) = (sensor_x, distance);
            while dist_left <= manhattan {
                no_beacons.insert((x, Y));
                dist_left += 1;
                x -= 1;
            }
        });

    let result = no_beacons.difference(&beacons).count();

    format!("{}", result)
}

pub fn point_is_valid(sensor_beacon_map: &Map, point: Point) -> bool {
    let it = sensor_beacon_map
        .iter()
        .find(|(&sensor, &(beacon, distance_to_beacon))| {
            manhattan_distance(point, sensor) <= distance_to_beacon
        });

    !it.is_some()
}

pub fn part_2(input: utils::Input) -> String {
    let data = parse_data(input);
    let mut result = 0;

    format!("{}", result)
}

fn run_1() {
    #[rustfmt::skip]
    let tests_1 = [
        // (1, Some("26")),
    ];

    let all_correct_1 = utils::test(part_1, DATE, 1, &tests_1);
    if all_correct_1 {
        let answer = utils::run(part_1, 1, DATE);

        match &answer[..] {
            "5838454" | "4651419" => println!(" WRONG!\n"),
            _ => {}
        }
    }
}

fn run_2() {
    #[rustfmt::skip]
    let tests_2 = [
        (1, Some("56000011")),
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
