cargo install cargo-watch 

COMMAND="clear; cargo run --release --bin $1"
cargo watch -w utils -w solutions -w run.sh -s "$COMMAND"
