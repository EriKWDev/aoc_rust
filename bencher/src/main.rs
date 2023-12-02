#[rustfmt::skip]
pub fn bench_2022(output: &mut Vec<utils::BenchResult>, max_time: Option<std::time::Duration>) {
    output.push(utils::bench(solutions::p2022_01::part_1, 1, solutions::p2022_01::DATE, max_time));
    output.push(utils::bench(solutions::p2022_01::part_2, 2, solutions::p2022_01::DATE, max_time));

    output.push(utils::bench(solutions::p2022_02::part_1, 1, solutions::p2022_02::DATE, max_time));
    output.push(utils::bench(solutions::p2022_02::part_2, 2, solutions::p2022_02::DATE, max_time));

    output.push(utils::bench(solutions::p2022_03::part_1, 1, solutions::p2022_03::DATE, max_time));
    output.push(utils::bench(solutions::p2022_03::part_2, 2, solutions::p2022_03::DATE, max_time));

    output.push(utils::bench(solutions::p2022_03::part_1, 1, solutions::p2022_03::DATE, max_time));
    output.push(utils::bench(solutions::p2022_03::part_2, 2, solutions::p2022_03::DATE, max_time));

    output.push(utils::bench(solutions::p2022_04::part_1, 1, solutions::p2022_04::DATE, max_time));
    output.push(utils::bench(solutions::p2022_04::part_2, 2, solutions::p2022_04::DATE, max_time));

    output.push(utils::bench(solutions::p2022_05::part_1, 1, solutions::p2022_05::DATE, max_time));
    output.push(utils::bench(solutions::p2022_05::part_2, 2, solutions::p2022_05::DATE, max_time));

    output.push(utils::bench(solutions::p2022_06::part_1, 1, solutions::p2022_06::DATE, max_time));
    output.push(utils::bench(solutions::p2022_06::part_2, 2, solutions::p2022_06::DATE, max_time));

    output.push(utils::bench(solutions::p2022_07::part_1, 1, solutions::p2022_07::DATE, max_time));
    output.push(utils::bench(solutions::p2022_07::part_2, 2, solutions::p2022_07::DATE, max_time));

    output.push(utils::bench(solutions::p2022_08::part_1, 1, solutions::p2022_08::DATE, max_time));
    output.push(utils::bench(solutions::p2022_08::part_2, 2, solutions::p2022_08::DATE, max_time));

    output.push(utils::bench(solutions::p2022_09::part_1, 1, solutions::p2022_09::DATE, max_time));
    output.push(utils::bench(solutions::p2022_09::part_2, 2, solutions::p2022_09::DATE, max_time));

    output.push(utils::bench(solutions::p2022_10::part_1, 1, solutions::p2022_10::DATE, max_time));
    output.push(utils::bench(solutions::p2022_10::part_2, 2, solutions::p2022_10::DATE, max_time));

    output.push(utils::bench(solutions::p2022_11::part_1, 1, solutions::p2022_11::DATE, max_time));
    output.push(utils::bench(solutions::p2022_11::part_2, 2, solutions::p2022_11::DATE, max_time));

    output.push(utils::bench(solutions::p2022_12::part_1, 1, solutions::p2022_12::DATE, max_time));
    output.push(utils::bench(solutions::p2022_12::part_2, 2, solutions::p2022_12::DATE, max_time));

    output.push(utils::bench(solutions::p2022_13::part_1, 1, solutions::p2022_13::DATE, max_time));
    output.push(utils::bench(solutions::p2022_13::part_2, 2, solutions::p2022_13::DATE, max_time));

    output.push(utils::bench(solutions::p2022_14::part_1, 1, solutions::p2022_14::DATE, max_time));
    output.push(utils::bench(solutions::p2022_14::part_2, 2, solutions::p2022_14::DATE, max_time));

    output.push(utils::bench(solutions::p2022_15::part_1, 1, solutions::p2022_15::DATE, max_time));
    // output.push(utils::bench(solutions::p2022_15::part_2, 2, solutions::p2022_15::DATE, max_time));

    // output.push(utils::bench(solutions::p2022_16::part_1, 1, solutions::p2022_16::DATE, max_time));
    // output.push(utils::bench(solutions::p2022_16::part_2, 2, solutions::p2022_16::DATE, max_time));

    output.push(utils::bench(solutions::p2022_17::part_1, 1, solutions::p2022_17::DATE, max_time));
    // output.push(utils::bench(solutions::p2022_17::part_2, 2, solutions::p2022_17::DATE, max_time));

    output.push(utils::bench(solutions::p2022_18::part_1, 1, solutions::p2022_18::DATE, max_time));
    output.push(utils::bench(solutions::p2022_18::part_2, 2, solutions::p2022_18::DATE, max_time));

    output.push(utils::bench(solutions::p2022_19::part_1, 1, solutions::p2022_19::DATE, max_time));
    output.push(utils::bench(solutions::p2022_19::part_2, 2, solutions::p2022_19::DATE, max_time));

    output.push(utils::bench(solutions::p2022_20::part_1, 1, solutions::p2022_20::DATE, max_time));
    output.push(utils::bench(solutions::p2022_20::part_2, 2, solutions::p2022_20::DATE, max_time));
}

#[rustfmt::skip]
pub fn bench_2023(output: &mut Vec<utils::BenchResult>, max_time: Option<std::time::Duration>) {
    {   pub use solutions::p2023_01::*;
        output.push(utils::bench(part_1, 1, DATE, max_time));
        output.push(utils::bench(part_2, 2, DATE, max_time));
    }
}

#[rustfmt::skip]
fn main() {
    let max_time = Some(std::time::Duration::from_secs(10));

    let mut results = vec![];

    // bench_2022(&mut results, max_time);
    bench_2023(&mut results, max_time);

    println!("\n\n== Benchmarks by Date and Part ==");
    utils::summarize_results(&results);

    results.sort_by(|a, b| a.average.partial_cmp(&b.average).unwrap());

    println!("\n== Benchmarks by Speed ==");
    utils::summarize_results(&results);
}
