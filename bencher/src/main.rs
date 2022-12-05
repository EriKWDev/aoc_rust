#[rustfmt::skip]
fn main() {
    println!("== Benchmarks 2022 ==");

    let max_time = None;

    utils::bench(solutions::p2022_01::part_1, 1, solutions::p2022_01::DATE, max_time);
    utils::bench(solutions::p2022_01::part_2, 2, solutions::p2022_01::DATE, max_time);

    utils::bench(solutions::p2022_02::part_1, 1, solutions::p2022_02::DATE, max_time);
    utils::bench(solutions::p2022_02::part_2, 2, solutions::p2022_02::DATE, max_time);

    utils::bench(solutions::p2022_03::part_1, 1, solutions::p2022_03::DATE, max_time);
    utils::bench(solutions::p2022_03::part_2, 2, solutions::p2022_03::DATE, max_time);

    utils::bench(solutions::p2022_03::part_1, 1, solutions::p2022_03::DATE, max_time);
    utils::bench(solutions::p2022_03::part_2, 2, solutions::p2022_03::DATE, max_time);

    utils::bench(solutions::p2022_04::part_1, 1, solutions::p2022_04::DATE, max_time);
    utils::bench(solutions::p2022_04::part_2, 2, solutions::p2022_04::DATE, max_time);

    utils::bench(solutions::p2022_05::part_1, 1, solutions::p2022_05::DATE, max_time);
    utils::bench(solutions::p2022_05::part_2, 2, solutions::p2022_05::DATE, max_time);
}
