cargo install cargo-watch 

COMMAND="clear; RUST_BACKTRACE=1 cargo lrun --bin $1 $2 $3"
cargo watch -w utils -w solutions -w run.sh -w Cargo.toml -s "$COMMAND"
