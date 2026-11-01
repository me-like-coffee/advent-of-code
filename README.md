# Advent of Code

My solutions to [Advent of Code](https://adventofcode.com) puzzles, in Rust.

## Usage

```sh
cargo run --release -- -y 2024 -d 1     # one day
cargo run --release -- -y 2024          # every solved day of a year
cargo run --release -- -a -p            # everything, days of a year in parallel
cargo run -- generate -y 2025 -d 13     # scaffold a new day
```

## Puzzle inputs

Inputs are not included: Advent of Code asks that they not be shared.
Save your own as `inputs/year_YYYY/day_NN.txt` (two-digit day), for example
`inputs/year_2024/day_01.txt`. The `inputs/` directory is gitignored.
Puzzle examples used by the tests live there too, as `day_NN_example{n}.txt`.
To keep them elsewhere, set `AOC_INPUTS` to a directory with the same layout.

## C++ (2022)

The 2022 C++ solutions are in `cpp/year_2022/` and read the same `inputs/` files.
They build with CMake 4 and vcpkg (`VCPKG_ROOT` must point to a vcpkg checkout):

```sh
cmake --preset dev && cmake --build --preset dev   # Debug, with sanitizers
ctest --preset dev                                 # unit tests of cpp/common
build/cpp/aoc_2022_day_01                          # one day; an input path may follow
```

## Tests

```sh
cargo test                          # tests that need no puzzle files
cargo test --features puzzle-inputs # also the tests that read inputs and examples
```

## License

[Apache-2.0](LICENSE)
