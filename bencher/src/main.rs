#[rustfmt::skip]
fn main() {
    let max_time = None;
    let mut results = vec![];

    results.push(utils::bench(solutions::p2022_01::part_1, 1, solutions::p2022_01::DATE, max_time));
    results.push(utils::bench(solutions::p2022_01::part_2, 2, solutions::p2022_01::DATE, max_time));

    results.push(utils::bench(solutions::p2022_02::part_1, 1, solutions::p2022_02::DATE, max_time));
    results.push(utils::bench(solutions::p2022_02::part_2, 2, solutions::p2022_02::DATE, max_time));

    results.push(utils::bench(solutions::p2022_03::part_1, 1, solutions::p2022_03::DATE, max_time));
    results.push(utils::bench(solutions::p2022_03::part_2, 2, solutions::p2022_03::DATE, max_time));

    results.push(utils::bench(solutions::p2022_03::part_1, 1, solutions::p2022_03::DATE, max_time));
    results.push(utils::bench(solutions::p2022_03::part_2, 2, solutions::p2022_03::DATE, max_time));

    results.push(utils::bench(solutions::p2022_04::part_1, 1, solutions::p2022_04::DATE, max_time));
    results.push(utils::bench(solutions::p2022_04::part_2, 2, solutions::p2022_04::DATE, max_time));

    results.push(utils::bench(solutions::p2022_05::part_1, 1, solutions::p2022_05::DATE, max_time));
    results.push(utils::bench(solutions::p2022_05::part_2, 2, solutions::p2022_05::DATE, max_time));

    results.push(utils::bench(solutions::p2022_06::part_1, 1, solutions::p2022_06::DATE, max_time));
    results.push(utils::bench(solutions::p2022_06::part_2, 2, solutions::p2022_06::DATE, max_time));

    results.push(utils::bench(solutions::p2022_07::part_1, 1, solutions::p2022_07::DATE, max_time));
    results.push(utils::bench(solutions::p2022_07::part_2, 2, solutions::p2022_07::DATE, max_time));

    results.push(utils::bench(solutions::p2022_08::part_1, 1, solutions::p2022_08::DATE, max_time));
    results.push(utils::bench(solutions::p2022_08::part_2, 2, solutions::p2022_08::DATE, max_time));

    results.push(utils::bench(solutions::p2022_09::part_1, 1, solutions::p2022_09::DATE, max_time));
    results.push(utils::bench(solutions::p2022_09::part_2, 2, solutions::p2022_09::DATE, max_time));

    println!("\n\n== Benchmarks by Date and Part ==");
    utils::summarize_results(&results);

    results.sort_by(|a, b| a.average.partial_cmp(&b.average).unwrap());

    println!("\n== Benchmarks by Speed ==");
    utils::summarize_results(&results);
}
