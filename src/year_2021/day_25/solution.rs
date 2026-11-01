use crate::utils::{Answer, Day, Year, read_input};
use anyhow::{Result, ensure};

fn parse(data: &str) -> Vec<Vec<u8>> {
    data.lines().map(|l| l.bytes().collect()).collect()
}

#[allow(
    clippy::needless_range_loop,
    reason = "the vertical pass scans by column"
)]
fn part_one(data: &str) -> Result<usize> {
    let mut grid = parse(data);
    ensure!(!grid.is_empty() && !grid[0].is_empty(), "empty grid");
    let (num_rows, num_cols) = (grid.len(), grid[0].len());
    let mut num_steps = 0;
    let mut moved = true;

    while moved {
        num_steps += 1;
        moved = false;
        let mut new_grid = grid.clone();

        // horizontal
        for y in 0..num_rows {
            for x in 0..num_cols {
                if grid[y][x] == b'>' && grid[y][(x + 1) % num_cols] == b'.' {
                    moved = true;
                    new_grid[y][x] = b'.';
                    new_grid[y][(x + 1) % num_cols] = b'>';
                }
            }
        }

        // vertical
        for x in 0..num_cols {
            for y in 0..num_rows {
                if grid[y][x] == b'v'
                    && new_grid[(y + 1) % num_rows][x] == b'.'
                    && grid[(y + 1) % num_rows][x] != b'v'
                {
                    moved = true;
                    new_grid[y][x] = b'.';
                    new_grid[(y + 1) % num_rows][x] = b'v';
                }
            }
        }
        grid = new_grid;
    }
    Ok(num_steps)
}

/// Solves Year 2021, Day 25: Sea Cucumber. There is no second part.
///
/// # Errors
/// Returns an error if the input cannot be read or parsed.
pub fn solve() -> Result<(Answer, Answer)> {
    let data = read_input(Year(2021), Day(25))?;
    Ok((u64::try_from(part_one(&data)?)?.into(), 0.into()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_example;

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test2_part_one() -> Result<()> {
        let data = read_example(Year(2021), Day(25), 2)?;
        assert_eq!(part_one(&data)?, 58);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_solve() -> Result<()> {
        let (p1, _) = solve()?;
        assert_eq!(p1, 400.into());
        Ok(())
    }
}
