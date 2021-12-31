# C++ + OpenMP + SDL2 implementation of Game of Life

## About

This version uses C++20 with OpenMP for parallelization and SDL2 for visualization of the game board.

## Usage

```bash
# default seed
./gol-omp

# custom seed (.rle file)
./gol-omp <custom_seed_file_path>
```

## Rquirements

* SDL2

## Build

The project uses CMake and should be buildable as any other C++ CMAKE project.