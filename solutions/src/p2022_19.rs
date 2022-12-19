#![allow(dead_code, unreachable_code, unused)]

use utils::*;

pub type Data = Vec<(u16, u16, u16, u16, u16, u16, u16)>;
pub const DATE: utils::Date = (2022, 19);

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

                    buf.push(buf2.parse::<u16>().unwrap());
                    buf2.clear();
                }
            }

            let res = (buf[0], buf[1], buf[2], buf[3], buf[4], buf[5], buf[6]);
            buf.clear();

            res
        })
        .collect::<Vec<_>>()
}

fn solve(
    ore_cost: u16,
    clay_cost: u16,
    obsidian_cost_ore: u16,
    obsidian_cost_clay: u16,
    geode_cost_ore: u16,
    geode_cost_obsidian: u16,
    time: u16,
) -> u16 {
    let initial_state = (0, 0, 0, 0, 1, 0, 0, 0, time);

    let mut stack = Vec::from([initial_state]);
    let mut seen = HashMap::new();

    let mut result = 0;

    let max_ore_cost = *[ore_cost, clay_cost, obsidian_cost_ore, geode_cost_ore]
        .iter()
        .max()
        .unwrap();
    let max_clay_cost = obsidian_cost_clay;
    let max_obsidian_cost = geode_cost_obsidian;

    while let Some(state) = stack.pop() {
        let (mut ore, mut clay, mut obsidian, mut geode, mut g1, mut g2, mut g3, mut g4, time_left) =
            state;

        result = result.max(geode);

        if time_left == 0 {
            continue;
        }

        let new_time = time_left - 1;

        g1 = g1.min(max_ore_cost);
        g2 = g2.min(max_clay_cost);
        g3 = g3.min(max_obsidian_cost);

        ore = ore.min(time_left * max_ore_cost - g1 * new_time);
        clay = clay.min(time_left * obsidian_cost_clay - g2 * new_time);
        obsidian = obsidian.min(time_left * geode_cost_obsidian - g3 * new_time);

        let state = (ore, clay, obsidian, geode, g1, g2, g3, g4, time_left);
        let std::collections::hash_map::Entry::Vacant(v) = seen.entry(state)else {
            continue
        };
        v.insert(());

        stack.push((
            ore + g1,
            clay + g2,
            obsidian + g3,
            geode + g4,
            g1,
            g2,
            g3,
            g4,
            new_time,
        ));

        if ore >= ore_cost {
            stack.push((
                ore - ore_cost + g1,
                clay + g2,
                obsidian + g3,
                geode + g4,
                g1 + 1,
                g2,
                g3,
                g4,
                new_time,
            ));
        }
        if ore >= clay_cost {
            stack.push((
                ore - clay_cost + g1,
                clay + g2,
                obsidian + g3,
                geode + g4,
                g1,
                g2 + 1,
                g3,
                g4,
                new_time,
            ));
        }
        if ore >= obsidian_cost_ore && clay >= obsidian_cost_clay {
            stack.push((
                ore - obsidian_cost_ore + g1,
                clay - obsidian_cost_clay + g2,
                obsidian + g3,
                geode + g4,
                g1,
                g2,
                g3 + 1,
                g4,
                new_time,
            ));
        }
        if ore >= geode_cost_ore && obsidian >= geode_cost_obsidian {
            stack.push((
                ore - geode_cost_ore + g1,
                clay + g2,
                obsidian - geode_cost_obsidian + g3,
                geode + g4,
                g1,
                g2,
                g3,
                g4 + 1,
                new_time,
            ));
        }
    }

    result
}

pub fn part_1(input: utils::Input) -> String {
    let data = parse_data(input);

    const TIME: u16 = 24;

    let result = data
        .iter()
        .map(|&blueprint| {
            let (i, or_cost, cl_cost, ob_cost_ore, ob_cost_clay, ge_cost_ore, ge_cost_obsidian) =
                blueprint;

            i * solve(
                or_cost,
                cl_cost,
                ob_cost_ore,
                ob_cost_clay,
                ge_cost_ore,
                ge_cost_obsidian,
                TIME,
            )
        })
        .sum::<u16>();

    format!("{}", result)
}

pub fn part_2(input: utils::Input) -> String {
    let data = parse_data(input);

    const TIME: u16 = 32;

    let result = data
        .iter()
        .take(3)
        .map(|&blueprint| {
            let (_, or_cost, cl_cost, ob_cost_ore, ob_cost_clay, ge_cost_ore, ge_cost_obsidian) =
                blueprint;

            solve(
                or_cost,
                cl_cost,
                ob_cost_ore,
                ob_cost_clay,
                ge_cost_ore,
                ge_cost_obsidian,
                TIME,
            )
        })
        .product::<u16>();

    format!("{}", result)
}

fn run_1() {
    #[rustfmt::skip]
    let tests_1 = [
        (1, Some("33")),
    ];

    let all_correct_1 = utils::test(part_1, DATE, 1, &tests_1);
    if all_correct_1 {
        let answer = utils::run(part_1, 1, DATE);
    }
}

fn run_2() {
    #[rustfmt::skip]
    let tests_2 = [
        (1, Some("3472")),
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
