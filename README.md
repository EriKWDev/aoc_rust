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
2022_01 part 1 .......... 45.632µs (ran 10000/10000)
2022_01 part 2 .......... 48.544µs (ran 10000/10000)
2022_02 part 1 .......... 68.059µs (ran 10000/10000)
2022_02 part 2 .......... 70.443µs (ran 10000/10000)
2022_03 part 1 .......... 193.892µs (ran 10000/10000)
2022_03 part 2 .......... 234.697µs (ran 10000/10000)
2022_03 part 1 .......... 199.969µs (ran 10000/10000)
2022_03 part 2 .......... 240.486µs (ran 10000/10000)
2022_04 part 1 .......... 75.857µs (ran 10000/10000)
2022_04 part 2 .......... 75.503µs (ran 10000/10000)
2022_05 part 1 .......... 24.515µs (ran 10000/10000)
2022_05 part 2 .......... 25.306µs (ran 10000/10000)
2022_06 part 1 .......... 67.954µs (ran 10000/10000)
2022_06 part 2 .......... 642.042µs (ran 10000/10000)
2022_07 part 1 .......... 178.947µs (ran 10000/10000)
2022_07 part 2 .......... 197.051µs (ran 10000/10000)

== Benchmarks by Speed ==
2022_05 part 1 .......... 24.515µs (ran 10000/10000)
2022_05 part 2 .......... 25.306µs (ran 10000/10000)
2022_01 part 1 .......... 45.632µs (ran 10000/10000)
2022_01 part 2 .......... 48.544µs (ran 10000/10000)
2022_06 part 1 .......... 67.954µs (ran 10000/10000)
2022_02 part 1 .......... 68.059µs (ran 10000/10000)
2022_02 part 2 .......... 70.443µs (ran 10000/10000)
2022_04 part 2 .......... 75.503µs (ran 10000/10000)
2022_04 part 1 .......... 75.857µs (ran 10000/10000)
2022_07 part 1 .......... 178.947µs (ran 10000/10000)
2022_03 part 1 .......... 193.892µs (ran 10000/10000)
2022_07 part 2 .......... 197.051µs (ran 10000/10000)
2022_03 part 1 .......... 199.969µs (ran 10000/10000)
2022_03 part 2 .......... 234.697µs (ran 10000/10000)
2022_03 part 2 .......... 240.486µs (ran 10000/10000)
2022_06 part 2 .......... 642.042µs (ran 10000/10000)
```
