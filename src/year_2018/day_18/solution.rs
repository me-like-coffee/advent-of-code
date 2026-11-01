/*
    https://www.jagregory.com/abrash-black-book/
    ch 17 and 18
    https://github.com/slightknack/hashlife
*/

use crate::utils::grid::byte_grid;
use crate::utils::{Answer, Day, Year, read_input};
use anyhow::{Result, bail};

fn parse_grid(data: &str) -> Vec<Vec<u8>> {
    byte_grid(data)
}

fn update_grid(grid: &[Vec<u8>]) -> Result<Vec<Vec<u8>>> {
    let deltas = [
        [0, 1],
        [0, -1],
        [1, 0],
        [-1, 0],
        [1, 1],
        [-1, -1],
        [1, -1],
        [-1, 1],
    ];
    let mut new_grid = grid.to_vec();

    for (y, row) in grid.iter().enumerate() {
        for (x, &acre) in row.iter().enumerate() {
            let (mut trees, mut lumberyards) = (0, 0);
            for delta in deltas {
                let new_x = delta[0] + x as i32;
                let new_y = delta[1] + y as i32;

                if (0..grid.len() as i32).contains(&new_y)
                    && (0..grid[0].len() as i32).contains(&new_x)
                {
                    match grid[new_y as usize][new_x as usize] {
                        b'|' => trees += 1,
                        b'#' => lumberyards += 1,
                        _ => {}
                    }
                }
            }

            new_grid[y][x] = match acre {
                b'.' => {
                    if trees >= 3 {
                        b'|'
                    } else {
                        b'.'
                    }
                }
                b'|' => {
                    if lumberyards >= 3 {
                        b'#'
                    } else {
                        b'|'
                    }
                }
                b'#' => {
                    if trees >= 1 && lumberyards >= 1 {
                        b'#'
                    } else {
                        b'.'
                    }
                }
                c => bail!("unknown acre {:?}", c as char),
            }
        }
    }

    Ok(new_grid)
}

pub fn part_one(data: &str) -> Result<usize> {
    let mut grid = parse_grid(data);

    for _ in 0..10 {
        grid = update_grid(&grid)?;
    }

    let wooded_acres = grid
        .iter()
        .flat_map(|r| r.iter())
        .filter(|b| **b == b'|')
        .count();

    let lumberyard = grid
        .iter()
        .flat_map(|r| r.iter())
        .filter(|b| **b == b'#')
        .count();

    Ok(wooded_acres * lumberyard)
}

/// Solves Year 2018, Day 18: Settlers of The North Pole.
///
/// Part two is not solved: the original stopped at the first repeated state
/// without projecting the cycle to a billion minutes, so it returns 0.
///
/// # Errors
/// Returns an error if the input cannot be read or holds an unknown acre.
pub fn solve() -> Result<(Answer, Answer)> {
    let data = read_input(Year(2018), Day(18))?;
    Ok((u64::try_from(part_one(&data)?)?.into(), 0.into()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_example;

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_part_one() -> Result<()> {
        assert_eq!(part_one(&read_example(Year(2018), Day(18), 1)?)?, 1147);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_solve() -> Result<()> {
        let (p1, _) = solve()?;
        assert_eq!(p1, 427961.into());
        Ok(())
    }
}
