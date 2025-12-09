#![allow(dead_code, unreachable_code, unused)]

use utils::*;

#[allow(non_snake_case_types)]
pub type s = i32;
type Point = (s, s, s);
pub type Data = Vec<(s, s, s)>;

pub fn parse_data(input: utils::Input) -> Data {
    input
        .lines()
        .map(|line| {
            let mut it = line.split(',');
            (
                it.next().unwrap().parse().unwrap(),
                it.next().unwrap().parse().unwrap(),
                it.next().unwrap().parse().unwrap(),
            )
        })
        .collect::<Vec<_>>()
}

pub fn in_line((x0, y0, z0): Point, (x1, y1, z1): Point) -> bool {
    let a = if (x0 == x1) { 0 } else { 1 };
    let b = if (y0 == y1) { 0 } else { 1 };
    let c = if (z0 == z1) { 0 } else { 1 };
    a + b + c <= 1
}

pub fn straight_line_distance((x0, y0, z0): Point, (x1, y1, z1): Point) -> s {
    // (x0 - x1).abs() + (y0 - y1).abs() + (z0 - z1).abs()
    (x0 - x1).pow(2) + (y0 - y1).pow(2) + (z0 - z1).pow(2)
}

pub fn part_1(input: utils::Input) -> String {
    let data = parse_data(input);

    let points = data;

    let n = points.len();

    let mut names = HashMap::<Point, usize>::new();
    let mut map = HashMap::<Point, Vec<(s, Point)>>::new();

    let mut pairs = vec![];

    let mut name = 0;

    for i in 0..n {
        for j in i + 1..n - 1 {
            let p0 = points[i];
            let p1 = points[j];
            for p in [p0, p1] {
                names.entry(p).or_insert_with(|| {
                    name += 1;
                    name
                });
            }
            let d = straight_line_distance(p0, p1);
            map.entry(p0).or_default().push((d, p1));
            map.entry(p1).or_default().push((d, p0));
            pairs.push((d, p0, p1));
        }
    }

    pairs.sort_unstable();
    pairs.reverse();

    // if utils::is_test() {
    //     for i in 0..pairs.len() {
    //         let (d, p0, p1) = pairs[i];
    //         println!("{d} : {p0:<3?} - {p1:<3?}");
    //     }
    // }

    let mut k = 1;
    let mut dd = BTreeMap::<s, s>::new();
    let mut c = HashMap::<Point, s>::new();
    let mut i = 0;
    while let Some((d, a, b)) = pairs.pop() {
        println!(
            "{}:{a:?} and {}:{b:?}",
            names.get(&a).unwrap(),
            names.get(&b).unwrap()
        );

        if c.contains_key(&a) || c.contains_key(&b) {
            continue;
        }

        i += 1;

        if i > 3 {
            break;
        }

        if let Some(&o) = c.get(&a) {
            c.insert(b, o);
            *dd.entry(o).or_insert(1) += 1;
            continue;
        }
        if let Some(&o) = c.get(&b) {
            c.insert(a, o);
            *dd.entry(o).or_insert(1) += 1;
            continue;
        }

        c.insert(a, k);
        c.insert(b, k);
        *dd.entry(k).or_insert(1) += 1;
        k += 1;
    }

    let mut result = 1;
    for (k, v) in dd {
        println!("{k:?}: {v}");
        result *= v;
    }

    format!("{}", result)
}

pub fn part_2(input: utils::Input) -> String {
    let data = parse_data(input);
    let mut result = 0;

    format!("{}", result)
}

pub fn run_1(date: utils::Date) {
    #[rustfmt::skip]
    let tests_1 = [
        (1, Some("40")),
    ];

    let all_correct_1 = utils::test(part_1, date, 1, &tests_1);
    if all_correct_1 {
        let answer = utils::run(part_1, 1, date);
    }
}

pub fn run_2(date: utils::Date) {
    #[rustfmt::skip]
    let tests_2 = [
        (1, Some("ANSWER_PART_2")),
    ];

    let all_correct_2 = utils::test(part_2, date, 2, &tests_2);
    if all_correct_2 {
        let answer = utils::run(part_2, 2, date);
    }
}

pub fn date() -> utils::Date {
    utils::date_from_file_name(file!())
}

fn main() {
    let date = date();
    run_1(date);
    // run_2(date);
}
