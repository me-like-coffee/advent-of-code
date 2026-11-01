//! Shared utilities for the Advent of Code runner.
//!
//! This module provides the central types and functions for running puzzles,
//! reading input files, and managing year/day registration.

use anyhow::{Context, Result};
use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

pub mod answer;
pub mod grid;
pub mod parser;
pub mod scaffold;

pub use answer::Answer;

/// Environment variable that overrides the directory holding the puzzle inputs.
pub const INPUTS_ENV: &str = "AOC_INPUTS";

/// A wrapper around a year value to avoid primitive obsession.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Year(pub u16);

/// A wrapper around a day value to avoid primitive obsession.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Day(pub u8);

/// Signature for a puzzle solver function.
///
/// Every solver returns a result containing a tuple of (Part 1, Part 2) answers.
pub type SolveFn = fn() -> Result<(Answer, Answer)>;

/// Macro to register multiple days within a year module.
///
/// This macro automates the creation of module declarations and the `get_solver`
/// function for a specific year. A module name that isn't `day_01`..`day_25`
/// is a compile error.
#[macro_export]
macro_rules! register_days {
    ($($day:ident),* $(,)?) => {
        $(
            pub mod $day;
        )*

        /// Resolves a solver function for a specific day.
        pub(crate) fn get_solver(day: u8) -> Option<$crate::utils::SolveFn> {
            $(
                if day == const { $crate::utils::day_to_u8(stringify!($day)) } {
                    return Some($day::solve);
                }
            )*
            None
        }
    };
}

/// Macro to register the year modules of the crate.
///
/// It declares the modules and generates `YEARS` and `get_year_solver`.
/// A module name that isn't `year_` followed by four digits is a compile error.
#[macro_export]
macro_rules! register_years {
    ($($year:ident),* $(,)?) => {
        $(
            pub(crate) mod $year;
        )*

        /// The registered years, in registration order.
        pub const YEARS: &[u16] = &[$($crate::utils::year_to_u16(stringify!($year))),*];

        /// Top-level dispatcher to resolve a solver for a specific year and day.
        #[must_use]
        pub fn get_year_solver(year: u16, day: u8) -> Option<$crate::utils::SolveFn> {
            $(
                if year == const { $crate::utils::year_to_u16(stringify!($year)) } {
                    return $year::get_solver(day);
                }
            )*
            None
        }
    };
}

/// Parses a module name made of `prefix` followed by exactly `digits` ASCII digits.
///
/// Panics on any other name; in a const context that is a compile error.
const fn module_number(name: &str, prefix: &str, digits: usize) -> u32 {
    let name = name.as_bytes();
    let prefix = prefix.as_bytes();
    assert!(
        name.len() == prefix.len() + digits,
        "module name has the wrong length"
    );
    let mut i = 0;
    while i < prefix.len() {
        assert!(name[i] == prefix[i], "module name has the wrong prefix");
        i += 1;
    }
    let mut value = 0;
    while i < name.len() {
        assert!(name[i].is_ascii_digit(), "module name must end in digits");
        value = value * 10 + (name[i] - b'0') as u32;
        i += 1;
    }
    value
}

/// Converts a "day_XX" module name to its numeric value.
///
/// # Panics
/// Panics if the name isn't `day_` followed by two digits from `01` to `25`.
/// The registration macros call it in a const context, so a bad name fails the build.
///
/// # Examples
/// ```
/// use advent_of_code::utils::day_to_u8;
/// assert_eq!(day_to_u8("day_01"), 1);
/// assert_eq!(day_to_u8("day_25"), 25);
/// ```
///
/// ```compile_fail
/// const DAY: u8 = advent_of_code::utils::day_to_u8("day_1");
/// ```
#[must_use]
pub const fn day_to_u8(name: &str) -> u8 {
    let day = module_number(name, "day_", 2);
    assert!(day >= 1 && day <= 25, "day must be from 01 to 25");
    day as u8
}

/// Converts a "year_XXXX" module name to its numeric value.
///
/// # Panics
/// Panics if the name isn't `year_` followed by four digits.
/// The registration macros call it in a const context, so a bad name fails the build.
///
/// # Examples
/// ```
/// use advent_of_code::utils::year_to_u16;
/// assert_eq!(year_to_u16("year_2015"), 2015);
/// ```
///
/// ```compile_fail
/// const YEAR: u16 = advent_of_code::utils::year_to_u16("year_15");
/// ```
#[must_use]
pub const fn year_to_u16(name: &str) -> u16 {
    module_number(name, "year_", 4) as u16
}

/// Returns the directory holding the puzzle inputs.
///
/// This is `AOC_INPUTS` if set and not empty, otherwise `inputs/` in the crate root.
#[must_use]
pub fn inputs_root() -> PathBuf {
    inputs_root_from(env::var_os(INPUTS_ENV))
}

fn inputs_root_from(var: Option<OsString>) -> PathBuf {
    match var {
        Some(dir) if !dir.is_empty() => PathBuf::from(dir),
        _ => Path::new(env!("CARGO_MANIFEST_DIR")).join("inputs"),
    }
}

/// Returns the path of the puzzle input for a year and day under `root`.
#[must_use]
pub fn input_path(root: &Path, year: Year, day: Day) -> PathBuf {
    year_dir(root, year).join(format!("day_{:02}.txt", day.0))
}

/// Returns the path of example `n` for a year and day under `root`.
#[must_use]
pub fn example_path(root: &Path, year: Year, day: Day, n: u8) -> PathBuf {
    year_dir(root, year).join(format!("day_{:02}_example{n}.txt", day.0))
}

/// Returns the path of the extra puzzle file `name` for a year and day under `root`.
///
/// Extra files hold puzzle data that is neither the input nor an example, such as a
/// starting pattern given in the puzzle text.
#[must_use]
pub fn extra_path(root: &Path, year: Year, day: Day, name: &str) -> PathBuf {
    year_dir(root, year).join(format!("day_{:02}_{name}.txt", day.0))
}

fn year_dir(root: &Path, year: Year) -> PathBuf {
    root.join(format!("year_{}", year.0))
}

fn read_file(path: &Path, what: &str, year: Year, day: Day) -> Result<String> {
    fs::read_to_string(path).with_context(|| {
        format!(
            "Failed to read {what} for Year {}, Day {:02} at: {:?}",
            year.0, day.0, path
        )
    })
}

/// Reads the puzzle input for a specified year and day.
///
/// The input is expected at `year_XXXX/day_XX.txt` under [`inputs_root`].
///
/// # Errors
/// Returns an `anyhow::Error` if the input file cannot be found or read.
pub fn read_input(year: Year, day: Day) -> Result<String> {
    read_file(
        &input_path(&inputs_root(), year, day),
        "input file",
        year,
        day,
    )
}

/// Reads puzzle example `n` for a specified year and day.
///
/// The example is expected at `year_XXXX/day_XX_example{n}.txt` under [`inputs_root`].
///
/// # Errors
/// Returns an `anyhow::Error` if the example file cannot be found or read.
pub fn read_example(year: Year, day: Day, n: u8) -> Result<String> {
    read_file(
        &example_path(&inputs_root(), year, day, n),
        "example file",
        year,
        day,
    )
}

/// Reads the extra puzzle file `name` for a specified year and day.
///
/// The file is expected at `year_XXXX/day_XX_{name}.txt` under [`inputs_root`].
///
/// # Errors
/// Returns an `anyhow::Error` if the file cannot be found or read.
pub fn read_extra(year: Year, day: Day, name: &str) -> Result<String> {
    read_file(
        &extra_path(&inputs_root(), year, day, name),
        "extra puzzle file",
        year,
        day,
    )
}

/// The line break an input uses: `\r\n` if it has any, otherwise `\n`.
fn blank_line(input: &str) -> &'static str {
    if input.contains("\r\n") {
        "\r\n\r\n"
    } else {
        "\n\n"
    }
}

/// Splits an input into its sections, which are separated by blank lines.
///
/// Works with `\n` and `\r\n` line endings alike.
pub fn sections(input: &str) -> impl Iterator<Item = &str> {
    input.split(blank_line(input))
}

/// Splits an input at its first blank line, or returns `None` if it has none.
///
/// Works with `\n` and `\r\n` line endings alike.
#[must_use]
pub fn split_at_blank_line(input: &str) -> Option<(&str, &str)> {
    input.split_once(blank_line(input))
}

/// Standardized runner for a puzzle.
///
/// Executes the provided `solve_fn`, tracks its execution time, and prints the results.
///
/// # Errors
/// Returns an error if the puzzle solver function itself returns an `Err`.
pub fn run_day(year: Year, day: Day, solve_fn: SolveFn) -> Result<()> {
    let start = Instant::now();
    let (p1, p2) = solve_fn()?;
    let duration = start.elapsed();

    println!(
        "[{}] Day {:02}: p1={p1:<15} p2={p2:<15} ({:?})",
        year.0, day.0, duration
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_to_u8() {
        assert_eq!(day_to_u8("day_01"), 1);
        assert_eq!(day_to_u8("day_10"), 10);
        assert_eq!(day_to_u8("day_25"), 25);
    }

    #[test]
    #[should_panic(expected = "wrong length")]
    fn test_day_to_u8_rejects_one_digit() {
        let _ = day_to_u8("day_1");
    }

    #[test]
    #[should_panic(expected = "digits")]
    fn test_day_to_u8_rejects_non_digits() {
        let _ = day_to_u8("day_XY");
    }

    #[test]
    #[should_panic(expected = "wrong prefix")]
    fn test_day_to_u8_rejects_wrong_prefix() {
        let _ = day_to_u8("dey_01");
    }

    #[test]
    #[should_panic(expected = "from 01 to 25")]
    fn test_day_to_u8_rejects_day_00() {
        let _ = day_to_u8("day_00");
    }

    #[test]
    #[should_panic(expected = "from 01 to 25")]
    fn test_day_to_u8_rejects_day_26() {
        let _ = day_to_u8("day_26");
    }

    #[test]
    fn test_year_to_u16() {
        assert_eq!(year_to_u16("year_2015"), 2015);
        assert_eq!(year_to_u16("year_2025"), 2025);
    }

    #[test]
    #[should_panic(expected = "wrong length")]
    fn test_year_to_u16_rejects_short_year() {
        let _ = year_to_u16("year_15");
    }

    #[test]
    fn test_inputs_root_from_env() {
        let dir = env::temp_dir().join("aoc-inputs");
        assert_eq!(inputs_root_from(Some(dir.clone().into_os_string())), dir);
    }

    #[test]
    fn test_inputs_root_defaults_to_crate_root() {
        let default = Path::new(env!("CARGO_MANIFEST_DIR")).join("inputs");
        assert_eq!(inputs_root_from(None), default);
        assert_eq!(inputs_root_from(Some(OsString::new())), default);
    }

    #[test]
    fn test_input_path() {
        let root = Path::new("root");
        let expected = root.join("year_2019").join("day_05.txt");
        assert_eq!(input_path(root, Year(2019), Day(5)), expected);
    }

    #[test]
    fn test_example_path() {
        let root = Path::new("root");
        let expected = root.join("year_2024").join("day_01_example2.txt");
        assert_eq!(example_path(root, Year(2024), Day(1), 2), expected);
    }

    #[test]
    fn test_extra_path() {
        let root = Path::new("root");
        let expected = root.join("year_2017").join("day_21_start.txt");
        assert_eq!(extra_path(root, Year(2017), Day(21), "start"), expected);
    }

    #[test]
    fn test_sections() {
        let found: Vec<&str> = sections("a\nb\n\nc\n\nd\n").collect();
        assert_eq!(found, vec!["a\nb", "c", "d\n"]);
    }

    #[test]
    fn test_sections_windows_line_endings() {
        let found: Vec<&str> = sections("a\r\nb\r\n\r\nc\r\n").collect();
        assert_eq!(found, vec!["a\r\nb", "c\r\n"]);
    }

    #[test]
    fn test_sections_without_blank_line() {
        let found: Vec<&str> = sections("a\nb").collect();
        assert_eq!(found, vec!["a\nb"]);
    }

    #[test]
    fn test_split_at_blank_line() {
        assert_eq!(split_at_blank_line("a\n\nb\n\nc"), Some(("a", "b\n\nc")));
        assert_eq!(split_at_blank_line("a\r\n\r\nb"), Some(("a", "b")));
        assert_eq!(split_at_blank_line("a\nb"), None);
    }

    #[test]
    fn test_read_file() -> Result<()> {
        let dir = env::temp_dir().join(format!("aoc-read-file-{}", std::process::id()));
        fs::create_dir_all(&dir)?;
        let path = dir.join("day_01.txt");
        fs::write(&path, "1\n2\n")?;
        let content = read_file(&path, "input file", Year(2015), Day(1));
        fs::remove_dir_all(&dir)?;
        assert_eq!(content?, "1\n2\n");
        Ok(())
    }

    #[test]
    fn test_read_file_missing() {
        let path = env::temp_dir().join("aoc-no-such-dir").join("day_01.txt");
        let err = read_file(&path, "input file", Year(2015), Day(1)).unwrap_err();
        assert!(err.to_string().contains("Year 2015, Day 01"));
    }
}
