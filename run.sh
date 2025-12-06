cargo install cargo-watch 

MODE=""
# MODE="--release"

COMMAND="clear; RUST_BACKTRACE=1 cargo lrun $MODE --bin $1 $2 $3"

cargo watch -w utils -w solutions -w run.sh -w Cargo.toml -s "$COMMAND"
