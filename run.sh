cargo install cargo-watch 

COMMAND="clear; cargo run --bin $1"
cargo watch -w utils -w solutions -w run.sh -w Cargo.toml -s "$COMMAND"
