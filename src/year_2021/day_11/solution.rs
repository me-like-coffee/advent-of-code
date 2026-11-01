use crate::utils::{Answer, Day, Year, read_input};
use anyhow::Result;

type IntType = u32;

fn parse(data: &str) -> Vec<Vec<u8>> {
    data.lines()
        .map(|l| l.bytes().map(|c| c - b'0').collect())
        .collect()
}

fn part_one(data: &str, num_steps: usize) -> IntType {
    let mut grid = parse(data);
    let mut num_flashes = 0;

    for _ in 0..num_steps {
        // gain energy
        for o in grid.iter_mut().flat_map(|r| r.iter_mut()) {
            *o += 1;
        }

        // flash
        let mut flashed = vec![vec![false; grid[0].len()]; grid.len()];
        let mut changed = true;
        while changed {
            changed = false;
            let mut new_grid = grid.clone();
            for (y, row) in grid.iter().enumerate() {
                for (x, o) in row.iter().enumerate() {
                    if *o > 9 {
                        new_grid[y][x] = 0;
                        flashed[y][x] = true;
                        changed = true;
                        num_flashes += 1;
                        for dy in -1..=1 {
                            for dx in -1..=1 {
                                if dx == 0 && dy == 0 {
                                    continue;
                                }
                                let x_new = dx + x as i32;
                                let y_new = dy + y as i32;
                                if (0..new_grid.len() as i32).contains(&y_new)
                                    && (0..new_grid[0].len() as i32).contains(&x_new)
                                    && !flashed[y_new as usize][x_new as usize]
                                {
                                    new_grid[y_new as usize][x_new as usize] += 1;
                                }
                            }
                        }
                    }
                }
            }
            grid = new_grid;
        }
    }
    num_flashes
}

fn part_two(data: &str) -> IntType {
    let mut grid = parse(data);
    let mut num_cycles = 0;

    loop {
        num_cycles += 1;
        // gain energy
        for o in grid.iter_mut().flat_map(|r| r.iter_mut()) {
            *o += 1;
        }

        // flash
        let mut flashed = vec![vec![false; grid[0].len()]; grid.len()];
        let mut changed = true;
        while changed {
            changed = false;
            let mut new_grid = grid.clone();
            for (y, row) in grid.iter().enumerate() {
                for (x, o) in row.iter().enumerate() {
                    if *o > 9 {
                        new_grid[y][x] = 0;
                        flashed[y][x] = true;
                        changed = true;

                        for dy in -1..=1 {
                            for dx in -1..=1 {
                                if dx == 0 && dy == 0 {
                                    continue;
                                }
                                let x_new = dx + x as i32;
                                let y_new = dy + y as i32;
                                if (0..new_grid.len() as i32).contains(&y_new)
                                    && (0..new_grid[0].len() as i32).contains(&x_new)
                                    && !flashed[y_new as usize][x_new as usize]
                                {
                                    new_grid[y_new as usize][x_new as usize] += 1;
                                }
                            }
                        }
                    }
                }
            }
            grid = new_grid;
        }
        if grid.iter().flat_map(|r| r.iter()).all(|c| c == &0) {
            break;
        }
    }

    num_cycles
}

/// Solves Year 2021, Day 11: Dumbo Octopus.
///
/// # Errors
/// Returns an error if the input cannot be read or parsed.
pub fn solve() -> Result<(Answer, Answer)> {
    let data = read_input(Year(2021), Day(11))?;
    Ok((
        u64::from(part_one(&data, 100)).into(),
        u64::from(part_two(&data)).into(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_example;

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test1_part_one() -> Result<()> {
        let data = read_example(Year(2021), Day(11), 1)?;
        assert_eq!(part_one(&data, 2), 9);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test2_part_one() -> Result<()> {
        let data = read_example(Year(2021), Day(11), 2)?;
        assert_eq!(part_one(&data, 100), 1656);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test1_part_two() -> Result<()> {
        let data = read_example(Year(2021), Day(11), 1)?;
        assert_eq!(part_two(&data), 6);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test2_part_two() -> Result<()> {
        let data = read_example(Year(2021), Day(11), 2)?;
        assert_eq!(part_two(&data), 195);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_solve() -> Result<()> {
        let (p1, p2) = solve()?;
        assert_eq!(p1, 1652.into());
        assert_eq!(p2, 220.into());
        Ok(())
    }
}
