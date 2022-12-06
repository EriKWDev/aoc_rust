# Advent of Code, in Rust

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

Remember to put your secret session cookie token
in the file `solutions/SECRET` so that input data
can be downloaded automatically

## Benchmarks | Updated 2022 05 dec
```
== Benchmarks by Date and Part ==
2022_01 part 1 .......... 62.496µs (ran 10000/10000)
2022_01 part 2 .......... 66.774µs (ran 10000/10000)
2022_02 part 1 .......... 98.036µs (ran 10000/10000)
2022_02 part 2 .......... 100.684µs (ran 10000/10000)
2022_03 part 1 .......... 227.346µs (ran 10000/10000)
2022_03 part 2 .......... 275.773µs (ran 10000/10000)
2022_03 part 1 .......... 226.632µs (ran 10000/10000)
2022_03 part 2 .......... 276.679µs (ran 10000/10000)
2022_04 part 1 .......... 102.587µs (ran 10000/10000)
2022_04 part 2 .......... 101.495µs (ran 10000/10000)
2022_05 part 1 .......... 33.849µs (ran 10000/10000)
2022_05 part 2 .......... 34.301µs (ran 10000/10000)
2022_06 part 1 .......... 73.134µs (ran 10000/10000)
2022_06 part 2 .......... 737.245µs (ran 10000/10000)

== Benchmarks by Speed ==
2022_05 part 1 .......... 33.849µs (ran 10000/10000)
2022_05 part 2 .......... 34.301µs (ran 10000/10000)
2022_01 part 1 .......... 62.496µs (ran 10000/10000)
2022_01 part 2 .......... 66.774µs (ran 10000/10000)
2022_06 part 1 .......... 73.134µs (ran 10000/10000)
2022_02 part 1 .......... 98.036µs (ran 10000/10000)
2022_02 part 2 .......... 100.684µs (ran 10000/10000)
2022_04 part 2 .......... 101.495µs (ran 10000/10000)
2022_04 part 1 .......... 102.587µs (ran 10000/10000)
2022_03 part 1 .......... 226.632µs (ran 10000/10000)
2022_03 part 1 .......... 227.346µs (ran 10000/10000)
2022_03 part 2 .......... 275.773µs (ran 10000/10000)
2022_03 part 2 .......... 276.679µs (ran 10000/10000)
2022_06 part 2 .......... 737.245µs (ran 10000/10000)
```
