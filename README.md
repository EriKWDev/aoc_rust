
# Advent of Code, in Rust
## Setup
Put your AoC secret session token in the file `solutions/SECRET` so that input data can be downloaded automatically

To start a new day, copy `solutions/src/template.rs` into `solutions/src/p{YEAR}_{DAY}.rs`

Modify `solutions/Cargo.toml` to include
```toml
[[bin]]
name = "{YEAR}_{DAY}"
path = "src/p{YEAR}_{DAY}.rs"
```

Then, if you also want benches, you need to modify `solutions/src/lib.rs` to export your solution, as well as `bencher/src/main.rs` to use and run it

## Usage
To run a specific day, do
```console
cargo run --release --bin {YEAR}_{DAY}
```
..or, using the run.sh script which stats cargo-watch:
```console
./run.sh {YEAR}_{DAY}
```
For example:
```console
./run.sh 2022_01
```

## Adding Automatic Tests
Add input for tests to `input/{YEAR}_{DAY}/test_{n:02}`

These can then be automatically tested before running on the real input as such:

```rust
type Data = /* ... */;
fn part_1(input: Data) -> String { /* ... */ }

fn run_1() {
    /*
        NOTE: The number symbolizes the 'n' in 'test_{n:02}', then an optional answer to
              the test-case can be given which will be checked. If 'None', the test will be
              run but won't block running on real input
    */
    #[rustfmt::skip]
    let tests_1 = [
        (1, Some("7")),
        (2, Some("5")),
        (3, Some("6")),
        (4, Some("10")),
        (5, Some("11")),
    ];

    let all_correct_1 = utils::test(part_1, DATE, 1, &tests_1);
    if all_correct_1 {
        let answer = utils::run(part_1, 1, DATE);
    }
}

fn main() {
    run_1();
}
```

## Benchmarks 2023 | Updated 2023, december 8
```
== Benchmarks by Date and Part ==
2023_01 part 1 .......... 117.482µs (ran 10000/10000)
2023_01 part 2 .......... 771.592µs (ran 10000/10000)
2023_02 part 1 .......... 88.542µs (ran 10000/10000)
2023_02 part 2 .......... 88.75µs (ran 10000/10000)
2023_03 part 1 .......... 157.697µs (ran 10000/10000)
2023_03 part 2 .......... 265.176µs (ran 10000/10000)
2023_04 part 1 .......... 487.224µs (ran 10000/10000)
2023_04 part 2 .......... 479.073µs (ran 10000/10000)
2023_05 part 1 .......... 54.483µs (ran 10000/10000)
2023_06 part 1 .......... 839ns (ran 10000/10000)
2023_06 part 2 .......... 118.144091ms (ran 85/10000)
2023_08 part 1 .......... 673.708µs (ran 10000/10000)
2023_08 part 2 .......... 2.919208ms (ran 3426/10000)

== Benchmarks by Speed ==
2023_06 part 1 .......... 839ns (ran 10000/10000)
2023_05 part 1 .......... 54.483µs (ran 10000/10000)
2023_02 part 1 .......... 88.542µs (ran 10000/10000)
2023_02 part 2 .......... 88.75µs (ran 10000/10000)
2023_01 part 1 .......... 117.482µs (ran 10000/10000)
2023_03 part 1 .......... 157.697µs (ran 10000/10000)
2023_03 part 2 .......... 265.176µs (ran 10000/10000)
2023_04 part 2 .......... 479.073µs (ran 10000/10000)
2023_04 part 1 .......... 487.224µs (ran 10000/10000)
2023_08 part 1 .......... 673.708µs (ran 10000/10000)
2023_01 part 2 .......... 771.592µs (ran 10000/10000)
2023_08 part 2 .......... 2.919208ms (ran 3426/10000)
2023_06 part 2 .......... 118.144091ms (ran 85/10000)
```

## Benchmarks 2022 | Updated 2023, 1 december
```
== Benchmarks by Date and Part ==
2022_01 part 1 .......... 83.046µs (ran 10000/10000)
2022_01 part 2 .......... 84.475µs (ran 10000/10000)
2022_02 part 1 .......... 118.216µs (ran 10000/10000)
2022_02 part 2 .......... 120.237µs (ran 10000/10000)
2022_03 part 1 .......... 260.713µs (ran 10000/10000)
2022_03 part 2 .......... 325.875µs (ran 10000/10000)
2022_03 part 1 .......... 267.471µs (ran 10000/10000)
2022_03 part 2 .......... 325.235µs (ran 10000/10000)
2022_04 part 1 .......... 132.511µs (ran 10000/10000)
2022_04 part 2 .......... 131.719µs (ran 10000/10000)
2022_05 part 1 .......... 40.015µs (ran 10000/10000)
2022_05 part 2 .......... 39.19µs (ran 10000/10000)
2022_06 part 1 .......... 95.959µs (ran 10000/10000)
2022_06 part 2 .......... 804.769µs (ran 10000/10000)
2022_07 part 1 .......... 252.026µs (ran 10000/10000)
2022_07 part 2 .......... 277.773µs (ran 10000/10000)
2022_08 part 1 .......... 440.725µs (ran 10000/10000)
2022_08 part 2 .......... 332.754µs (ran 10000/10000)
2022_09 part 1 .......... 475.941µs (ran 10000/10000)
2022_09 part 2 .......... 752.154µs (ran 10000/10000)
2022_10 part 1 .......... 27.781µs (ran 10000/10000)
2022_10 part 2 .......... 29.571µs (ran 10000/10000)
2022_11 part 1 .......... 22.937µs (ran 10000/10000)
2022_11 part 2 .......... 7.162686ms (ran 1397/10000)
2022_12 part 1 .......... 67.417µs (ran 10000/10000)
2022_12 part 2 .......... 7.19752ms (ran 1390/10000)
2022_13 part 1 .......... 503.181µs (ran 10000/10000)
2022_13 part 2 .......... 636.322µs (ran 10000/10000)
2022_14 part 1 .......... 2.306372ms (ran 4336/10000)
2022_14 part 2 .......... 144.057772ms (ran 70/10000)
2022_15 part 1 .......... 1.738861349s (ran 6/10000)
2022_17 part 1 .......... 3.238669ms (ran 3088/10000)
2022_18 part 1 .......... 918.689µs (ran 10000/10000)
2022_18 part 2 .......... 4.639779ms (ran 2156/10000)
2022_19 part 1 .......... 714.733149ms (ran 14/10000)
2022_19 part 2 .......... 2.87080121s (ran 4/10000)
2022_20 part 1 .......... 12.476327ms (ran 802/10000)
2022_20 part 2 .......... 123.555747ms (ran 81/10000)

== Benchmarks by Speed ==
2022_11 part 1 .......... 22.937µs (ran 10000/10000)
2022_10 part 1 .......... 27.781µs (ran 10000/10000)
2022_10 part 2 .......... 29.571µs (ran 10000/10000)
2022_05 part 2 .......... 39.19µs (ran 10000/10000)
2022_05 part 1 .......... 40.015µs (ran 10000/10000)
2022_12 part 1 .......... 67.417µs (ran 10000/10000)
2022_01 part 1 .......... 83.046µs (ran 10000/10000)
2022_01 part 2 .......... 84.475µs (ran 10000/10000)
2022_06 part 1 .......... 95.959µs (ran 10000/10000)
2022_02 part 1 .......... 118.216µs (ran 10000/10000)
2022_02 part 2 .......... 120.237µs (ran 10000/10000)
2022_04 part 2 .......... 131.719µs (ran 10000/10000)
2022_04 part 1 .......... 132.511µs (ran 10000/10000)
2022_07 part 1 .......... 252.026µs (ran 10000/10000)
2022_03 part 1 .......... 260.713µs (ran 10000/10000)
2022_03 part 1 .......... 267.471µs (ran 10000/10000)
2022_07 part 2 .......... 277.773µs (ran 10000/10000)
2022_03 part 2 .......... 325.235µs (ran 10000/10000)
2022_03 part 2 .......... 325.875µs (ran 10000/10000)
2022_08 part 2 .......... 332.754µs (ran 10000/10000)
2022_08 part 1 .......... 440.725µs (ran 10000/10000)
2022_09 part 1 .......... 475.941µs (ran 10000/10000)
2022_13 part 1 .......... 503.181µs (ran 10000/10000)
2022_13 part 2 .......... 636.322µs (ran 10000/10000)
2022_09 part 2 .......... 752.154µs (ran 10000/10000)
2022_06 part 2 .......... 804.769µs (ran 10000/10000)
2022_18 part 1 .......... 918.689µs (ran 10000/10000)
2022_14 part 1 .......... 2.306372ms (ran 4336/10000)
2022_17 part 1 .......... 3.238669ms (ran 3088/10000)
2022_18 part 2 .......... 4.639779ms (ran 2156/10000)
2022_11 part 2 .......... 7.162686ms (ran 1397/10000)
2022_12 part 2 .......... 7.19752ms (ran 1390/10000)
2022_20 part 1 .......... 12.476327ms (ran 802/10000)
2022_20 part 2 .......... 123.555747ms (ran 81/10000)
2022_14 part 2 .......... 144.057772ms (ran 70/10000)
2022_19 part 1 .......... 714.733149ms (ran 14/10000)
2022_15 part 1 .......... 1.738861349s (ran 6/10000)
2022_19 part 2 .......... 2.87080121s (ran 4/10000)
```
