# Advent of Code

Advent of Code solutions. Rust is the main language: one crate with a CLI runner.
The 2022 C++ solutions sit under `cpp/`.
`REFACTORING.md` describes the target layout and the migration status.

## Layout
- `src/year_YYYY/day_NN/solution.rs` exposes `pub fn solve() -> Result<(u64, u64)>`.
  For days with only one part, return `0` as the second value.
- Register each day in `register_days!` in `src/year_YYYY/mod.rs`.
- Shared code goes in `src/utils/`. Read inputs with `utils::read_input`.
- Run a day with `cargo run --release -- -y 2024 -d 1`, or everything with `-a`.

## Puzzle data stays local
- Inputs live in `inputs/year_YYYY/day_NN.txt`, which is gitignored.
- Never commit inputs or puzzle text, and never hard-code an input in source.
- Expected answers in tests are fine.

## Tests
- Add unit tests for shared code only (`src/utils/`, `cpp/common/`). Don't add tests for
  individual day solutions.
- No `unwrap()` or `expect()` in `src/` outside `#[cfg(test)]`.
