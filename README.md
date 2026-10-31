# Advent of Code

My solutions to [Advent of Code](https://adventofcode.com) puzzles, in Rust.

## Usage

Run from the repository root:

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

## Tests

```sh
cargo test                          # tests that need no puzzle files
cargo test --features puzzle-inputs # also the tests that read inputs/
```

## License

[Apache-2.0](LICENSE)
