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
