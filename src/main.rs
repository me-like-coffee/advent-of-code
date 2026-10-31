use advent_of_code::utils::{self, scaffold};
use advent_of_code::{Day, Year};
use anyhow::{Context, Result, anyhow};
use clap::{Parser, Subcommand};
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Year to run (e.g. 2024)
    #[arg(short, long)]
    year: Option<u16>,

    /// Day to run (1-25)
    #[arg(short, long)]
    day: Option<u8>,

    /// Run all implemented puzzles
    #[arg(short, long)]
    all: bool,

    /// Run in parallel
    #[arg(short, long)]
    parallel: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a new day's scaffolding
    Generate {
        /// Year (2015-2025)
        #[arg(short, long)]
        year: u16,
        /// Day (1-25)
        #[arg(short, long)]
        day: u8,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Generate { year, day } => return generate_day(year, day),
        }
    }

    if cli.all {
        run_all(cli.parallel)?;
    } else {
        match (cli.year, cli.day) {
            (Some(year), Some(day)) => {
                let solver = advent_of_code::get_year_solver(year, day)
                    .ok_or_else(|| anyhow!("Year {}, Day {} not implemented", year, day))?;
                advent_of_code::run_day(Year(year), Day(day), solver)?;
            }
            (Some(year), None) => {
                run_year(year, cli.parallel)?;
            }
            (None, _) => {
                return Err(anyhow!(
                    "Please specify a year (-y), run all (-a), or use a subcommand"
                ));
            }
        }
    }

    Ok(())
}

fn run_year(year: u16, parallel: bool) -> Result<()> {
    let mut solvers = Vec::new();
    for day in 1..=25 {
        if let Some(solver) = advent_of_code::get_year_solver(year, day) {
            solvers.push((day, solver));
        }
    }

    if solvers.is_empty() {
        println!("No implemented days found for year {}", year);
        return Ok(());
    }

    if parallel {
        use rayon::prelude::*;
        solvers
            .into_par_iter()
            .try_for_each(|(day, solver)| advent_of_code::run_day(Year(year), Day(day), solver))?;
    } else {
        for (day, solver) in solvers {
            advent_of_code::run_day(Year(year), Day(day), solver)?;
        }
    }
    Ok(())
}

fn run_all(parallel: bool) -> Result<()> {
    for &year in advent_of_code::YEARS {
        run_year(year, parallel)?;
    }
    Ok(())
}

fn generate_day(year: u16, day: u8) -> Result<()> {
    let src_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let year_module = format!("year_{year}");
    let day_module = format!("day_{day:02}");
    let year_dir = src_dir.join(&year_module);
    let day_dir = year_dir.join(&day_module);
    let input_path = utils::input_path(&utils::inputs_root(), Year(year), Day(day));

    // Create directories
    fs::create_dir_all(&day_dir)?;
    if let Some(input_dir) = input_path.parent() {
        fs::create_dir_all(input_dir)?;
    }

    // Create day_XX.txt
    create_file(&input_path, "")?;

    // Create day_XX/mod.rs
    create_file(
        &day_dir.join("mod.rs"),
        "pub mod solution;\npub use solution::solve;\n",
    )?;

    // Create day_XX/solution.rs
    let template = format!(
        r#"use crate::utils::{{Answer, Day, Year, read_input}};
use anyhow::Result;

/// Core logic for Year {year}, Day {day:02}
fn calculate_solution(input: &str) -> Result<(u64, u64)> {{
    // TODO: Implement solution
    Ok((0, 0))
}}

pub fn solve() -> Result<(Answer, Answer)> {{
    let content = read_input(Year({year}), Day({day}))?;
    let (p1, p2) = calculate_solution(&content)?;
    Ok((p1.into(), p2.into()))
}}

#[cfg(test)]
mod tests {{
    use super::*;

    const TEST_INPUT: &str = "";

    #[test]
    fn test_day_{day:02}_solution() -> Result<()> {{
        let (p1, p2) = calculate_solution(TEST_INPUT)?;
        assert_eq!(p1, 0);
        assert_eq!(p2, 0);
        Ok(())
    }}
}}
"#
    );
    create_file(&day_dir.join("solution.rs"), &template)?;

    // Register the day in year_XXXX/mod.rs, creating it if needed
    let year_mod_path = year_dir.join("mod.rs");
    if year_mod_path.exists() {
        register(&year_mod_path, "register_days", &day_module)?;
    } else {
        create_file(
            &year_mod_path,
            &format!("use crate::register_days;\n\nregister_days!({day_module});\n"),
        )?;
    }

    // Register the year in lib.rs
    register(&src_dir.join("lib.rs"), "register_years", &year_module)?;

    Ok(())
}

/// Writes `content` to `path` unless the file already exists.
fn create_file(path: &Path, content: &str) -> Result<()> {
    if !path.exists() {
        fs::write(path, content)?;
        println!("Created {}", path.display());
    }
    Ok(())
}

/// Adds `item` to the `macro_name!` call in the file at `path`.
fn register(path: &Path, macro_name: &str, item: &str) -> Result<()> {
    let source = fs::read_to_string(path)?;
    if let Some(updated) = scaffold::register_in_macro(&source, macro_name, item)
        .with_context(|| format!("Failed to register {item} in {}", path.display()))?
    {
        fs::write(path, updated)?;
        println!("Registered {item} in {}", path.display());
    }
    Ok(())
}
