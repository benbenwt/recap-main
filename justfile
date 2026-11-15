export ELEFANT_TRUNK_ROOT := `git rev-parse --show-toplevel`

# list all available recipes
help:
    @just --list

# run in release mode for better performance
run: 
    cargo run -p recap --release --bin recap

# run in debug mode with with recap logs at trace level
trace:
    GST_DEBUG="3" RUST_LOG="warn,iced_wgpu::window::compositor=info,recap=trace" cargo run -p recap --release --bin recap --features trace

# run in dev mode for faster compilation
dev: 
    cargo run -p recap --bin recap

# run encoding check - all captures will automatically be stopped after 60s
encoding-check: 
    cargo run -p recap --release --bin recap --features encoding-check


convert args="":
    cargo run -p recap --bin convert --features convert -- {{args}}

watch:
    cargo watch -x 'run'
