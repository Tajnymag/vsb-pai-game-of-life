# Rust + SDL2 implementation of Game of Life

## About

This version uses Rust with SDL2 for visualization of the game board.

## Usage

```bash
# default seed
cargo run --package gol-rs --bin gol-rs --release

# custom seed (.rle file)
cargo run --package gol-rs --bin gol-rs --release <custom_seed_file_path>
```

## Build

The [Usage](#Usage) instructions above should be enough to build and run the project on your system.
