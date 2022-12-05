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
== Benchmarks 2022 ==
2022_01 part 1 .......... 49.781µs (ran 10000/10000)
2022_01 part 2 .......... 53.148µs (ran 10000/10000)
2022_02 part 1 .......... 75.877µs (ran 10000/10000)
2022_02 part 2 .......... 77.112µs (ran 10000/10000)
2022_03 part 1 .......... 168.579µs (ran 10000/10000)
2022_03 part 2 .......... 199.277µs (ran 10000/10000)
2022_03 part 1 .......... 164.793µs (ran 10000/10000)
2022_03 part 2 .......... 198.057µs (ran 10000/10000)
2022_04 part 1 .......... 78.744µs (ran 10000/10000)
2022_04 part 2 .......... 78.635µs (ran 10000/10000)
2022_05 part 1 .......... 26.964µs (ran 10000/10000)
2022_05 part 2 .......... 26.624µs (ran 10000/10000)
```

