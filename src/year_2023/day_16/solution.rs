use crate::utils::grid::byte_grid;
use crate::utils::{Answer, Day, Year, read_input};
use anyhow::{Result, bail, ensure};
use std::collections::{HashMap, HashSet};

use Heading::*;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Heading {
    East,
    North,
    South,
    West,
}

fn parse(data: &str) -> Vec<Vec<u8>> {
    byte_grid(data)
}

fn visit_and_stack(
    item: (usize, usize, Heading),
    stack: &mut Vec<(usize, usize, Heading)>,
    visited: &mut HashSet<(usize, usize, Heading)>,
) {
    if !visited.contains(&item) {
        stack.push(item.clone());
        visited.insert(item);
    }
}

fn energize_grid(grid: &[Vec<u8>], x: usize, y: usize, heading: Heading) -> Result<usize> {
    let (num_rows, num_cols) = (grid.len(), grid[0].len());

    let mut energy: HashMap<(usize, usize), Vec<Heading>> = HashMap::new();
    let mut stack = Vec::new();
    let mut visited = HashSet::new();

    stack.push((x, y, heading));

    while let Some((x, y, heading)) = stack.pop() {
        energy.entry((x, y)).or_default().push(heading.clone());
        visited.insert((x, y, heading.clone()));

        match grid[y][x] {
            b'-' => match heading {
                East => {
                    if x < num_cols - 1 {
                        visit_and_stack((x + 1, y, East), &mut stack, &mut visited)
                    }
                }
                North | South => {
                    if x > 0 {
                        visit_and_stack((x - 1, y, West), &mut stack, &mut visited);
                    };
                    if x < num_cols - 1 {
                        visit_and_stack((x + 1, y, East), &mut stack, &mut visited);
                    };
                }
                West => {
                    if x > 0 {
                        visit_and_stack((x - 1, y, West), &mut stack, &mut visited)
                    }
                }
            },
            b'|' => match heading {
                East | West => {
                    if y > 0 {
                        visit_and_stack((x, y - 1, North), &mut stack, &mut visited);
                    };
                    if y < num_rows - 1 {
                        visit_and_stack((x, y + 1, South), &mut stack, &mut visited);
                    };
                }
                North => {
                    if y > 0 {
                        visit_and_stack((x, y - 1, North), &mut stack, &mut visited);
                    }
                }
                South => {
                    if y < num_rows - 1 {
                        visit_and_stack((x, y + 1, South), &mut stack, &mut visited);
                    }
                }
            },
            b'.' => match heading {
                East => {
                    if x < num_cols - 1 {
                        visit_and_stack((x + 1, y, East), &mut stack, &mut visited)
                    }
                }
                North => {
                    if y > 0 {
                        visit_and_stack((x, y - 1, North), &mut stack, &mut visited);
                    }
                }
                South => {
                    if y < num_rows - 1 {
                        visit_and_stack((x, y + 1, South), &mut stack, &mut visited);
                    }
                }
                West => {
                    if x > 0 {
                        visit_and_stack((x - 1, y, West), &mut stack, &mut visited)
                    }
                }
            },
            b'/' => match heading {
                East => {
                    if y > 0 {
                        visit_and_stack((x, y - 1, North), &mut stack, &mut visited);
                    }
                }
                North => {
                    if x < num_cols - 1 {
                        visit_and_stack((x + 1, y, East), &mut stack, &mut visited)
                    }
                }
                South => {
                    if x > 0 {
                        visit_and_stack((x - 1, y, West), &mut stack, &mut visited)
                    }
                }
                West => {
                    if y < num_rows - 1 {
                        visit_and_stack((x, y + 1, South), &mut stack, &mut visited);
                    }
                }
            },
            b'\\' => match heading {
                East => {
                    if y < num_rows - 1 {
                        visit_and_stack((x, y + 1, South), &mut stack, &mut visited);
                    }
                }
                North => {
                    if x > 0 {
                        visit_and_stack((x - 1, y, West), &mut stack, &mut visited)
                    }
                }
                South => {
                    if x < num_cols - 1 {
                        visit_and_stack((x + 1, y, East), &mut stack, &mut visited)
                    }
                }
                West => {
                    if y > 0 {
                        visit_and_stack((x, y - 1, North), &mut stack, &mut visited);
                    }
                }
            },
            c => bail!("unknown tile {:?}", c as char),
        }
    }

    Ok(energy.len())
}

fn parse_grid(data: &str) -> Result<Vec<Vec<u8>>> {
    let grid = parse(data);
    ensure!(!grid.is_empty() && !grid[0].is_empty(), "empty grid");
    Ok(grid)
}

fn part_one(data: &str) -> Result<usize> {
    let grid = parse_grid(data)?;
    energize_grid(&grid, 0, 0, East)
}

fn part_two(data: &str) -> Result<usize> {
    let grid = parse_grid(data)?;
    let mut max_energized = 0;

    for x in 0..grid[0].len() {
        max_energized = max_energized.max(energize_grid(&grid, x, 0, South)?);
        max_energized = max_energized.max(energize_grid(&grid, x, grid.len() - 1, North)?);
    }

    for (y, row) in grid.iter().enumerate() {
        max_energized = max_energized.max(energize_grid(&grid, 0, y, East)?);
        max_energized = max_energized.max(energize_grid(&grid, row.len() - 1, y, West)?);
    }

    Ok(max_energized)
}

/// Solves Year 2023, Day 16: The Floor Will Be Lava.
///
/// # Errors
/// Returns an error if the input cannot be read or parsed.
pub fn solve() -> Result<(Answer, Answer)> {
    let data = read_input(Year(2023), Day(16))?;
    Ok((
        u64::try_from(part_one(&data)?)?.into(),
        u64::try_from(part_two(&data)?)?.into(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_example;

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test1_part_one() -> Result<()> {
        let data = read_example(Year(2023), Day(16), 1)?;
        assert_eq!(part_one(&data)?, 46);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test1_part_two() -> Result<()> {
        let data = read_example(Year(2023), Day(16), 1)?;
        assert_eq!(part_two(&data)?, 51);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_solve() -> Result<()> {
        let (p1, p2) = solve()?;
        assert_eq!(p1, 7477.into());
        assert_eq!(p2, 7853.into());
        Ok(())
    }
}
