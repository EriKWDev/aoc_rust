cargo install cargo-watch 

MODE=""
MODE="--release"

COMMAND="clear; RUST_BACKTRACE=1 cargo -q lrun $MODE --bin $@"

cargo watch -w utils -w solutions -w run.sh -w Cargo.toml -s "$COMMAND"
