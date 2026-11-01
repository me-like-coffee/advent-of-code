use crate::utils::{Answer, Day, Year, read_input};
use anyhow::{Context, Result, bail};
use std::collections::{HashMap, VecDeque};

type IntType = usize;
type Grid = Vec<Vec<(IntType, IntType, IntType)>>;

const START: (usize, usize) = (0, 0);
const PADDING: (usize, usize) = (40, 40);
const X_MULTIPLE: IntType = 16807;
const Y_MULTIPLE: IntType = 48271;
const MODULO: IntType = 20183;
const ROCKY: IntType = 0;
const WET: IntType = 1;
const NARROW: IntType = 2;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Tool {
    Climbing,
    Neither,
    Torch,
}

type VisitedState = (usize, usize, Tool);
type Visited = HashMap<VisitedState, usize>;

/// Parses "depth: D" and "target: X,Y".
fn parse(data: &str) -> Result<(usize, (usize, usize))> {
    let mut depth = None;
    let mut target = None;
    for line in data.lines() {
        if let Some(value) = line.strip_prefix("depth: ") {
            depth = Some(value.trim().parse()?);
        } else if let Some(value) = line.strip_prefix("target: ") {
            let (x, y) = value
                .trim()
                .split_once(',')
                .with_context(|| format!("not a target: {line:?}"))?;
            target = Some((x.parse()?, y.parse()?));
        }
    }
    Ok((
        depth.context("missing depth")?,
        target.context("missing target")?,
    ))
}

fn risk_level(grid: &Grid, x: usize, y: usize) -> IntType {
    grid[y][x].2
}

fn build_grid(depth: usize, start: (usize, usize), target: (usize, usize)) -> Grid {
    let mut grid = vec![
        vec![(IntType::MAX, IntType::MAX, IntType::MAX); target.0 + PADDING.0];
        target.1 + PADDING.1
    ];

    let geologic_index = 0;
    let erosion_level = (geologic_index + depth as IntType) % MODULO;
    let risk_level = erosion_level % 3;
    grid[start.1][start.0] = (geologic_index, erosion_level, risk_level);

    for x in 1..grid[0].len() {
        let geologic_index = X_MULTIPLE * x as IntType;
        let erosion_level = (geologic_index + depth as IntType) % MODULO;
        let risk_level = erosion_level % 3;
        grid[0][x] = (geologic_index, erosion_level, risk_level);
    }
    for (y, row) in grid.iter_mut().enumerate().skip(1) {
        let geologic_index = Y_MULTIPLE * y as IntType;
        let erosion_level = (geologic_index + depth as IntType) % MODULO;
        let risk_level = erosion_level % 3;
        row[0] = (geologic_index, erosion_level, risk_level);
    }

    for y in 1..grid.len() {
        for x in 1..grid[y].len() {
            if (x, y) == target {
                let geologic_index = 0;
                let erosion_level = (geologic_index + depth as IntType) % MODULO;
                let risk_level = erosion_level % 3;
                grid[y][x] = (geologic_index, erosion_level, risk_level);
            } else {
                let geologic_index = grid[y - 1][x].1 * grid[y][x - 1].1;
                let erosion_level = (geologic_index + depth as IntType) % MODULO;
                let risk_level = erosion_level % 3;
                grid[y][x] = (geologic_index, erosion_level, risk_level);
            }
        }
    }
    grid
}

fn total_risk_level(grid: &Grid, start: (usize, usize), target: (usize, usize)) -> IntType {
    let mut risk_level = 0;

    for row in &grid[start.1..=target.1] {
        for cell in &row[start.0..=target.0] {
            risk_level += cell.2;
        }
    }

    risk_level
}

fn part_one(depth: usize, start: (usize, usize), target: (usize, usize)) -> IntType {
    let grid = build_grid(depth, start, target);
    total_risk_level(&grid, start, target)
}

fn part_two(depth: usize, start: (usize, usize), target: (usize, usize)) -> Result<usize> {
    let grid = build_grid(depth, start, target);
    let mut best_path = usize::MAX;
    let mut q = VecDeque::new();
    let mut visited: Visited = HashMap::new();
    q.push_back((start.0, start.1, Tool::Torch, 0));
    visited.insert((start.0, start.1, Tool::Torch), 0);

    while let Some(state) = q.pop_front() {
        if (state.0, state.1) == target {
            let mut distance = state.3;
            if state.2 != Tool::Torch {
                distance += 7;
            }
            if distance < best_path {
                best_path = best_path.min(distance);
                visited.insert((state.0, state.1, Tool::Torch), distance);
            }
        } else {
            for delta in [1, 0, -1, 0, 1].windows(2) {
                let new_x = delta[0] + state.0 as i32;
                let new_y = delta[1] + state.1 as i32;

                let mut update = |x, y, tool, d| {
                    if visited.get(&(x, y, tool)).unwrap_or(&usize::MAX) > &d && d < best_path {
                        q.push_back((x, y, tool, d));
                        visited.insert((x, y, tool), d);
                    }
                };

                if (0..grid.len() as i32).contains(&new_y)
                    && (0..grid[state.1].len() as i32).contains(&new_x)
                {
                    let new_x = new_x as usize;
                    let new_y = new_y as usize;
                    let new_distance = state.3 + 1;
                    let curr_risk_level = risk_level(&grid, state.0, state.1);
                    match risk_level(&grid, new_x, new_y) {
                        ROCKY => {
                            if state.2 == Tool::Neither {
                                if curr_risk_level == WET {
                                    update(new_x, new_y, Tool::Climbing, new_distance + 7);
                                } else if curr_risk_level == NARROW {
                                    update(new_x, new_y, Tool::Torch, new_distance + 7);
                                }
                            } else {
                                update(new_x, new_y, state.2, new_distance);
                            }
                        }
                        WET => {
                            if state.2 == Tool::Torch {
                                if curr_risk_level == ROCKY {
                                    update(new_x, new_y, Tool::Climbing, new_distance + 7);
                                }
                                if curr_risk_level == NARROW {
                                    update(new_x, new_y, Tool::Neither, new_distance + 7);
                                }
                            } else {
                                update(new_x, new_y, state.2, new_distance);
                            }
                        }
                        NARROW => {
                            if state.2 == Tool::Climbing {
                                if curr_risk_level == ROCKY {
                                    update(new_x, new_y, Tool::Torch, new_distance + 7);
                                } else if curr_risk_level == WET {
                                    update(new_x, new_y, Tool::Neither, new_distance + 7);
                                }
                            } else {
                                update(new_x, new_y, state.2, new_distance);
                            }
                        }
                        r => bail!("unknown region type {r}"),
                    }
                }
            }
        }
    }
    visited
        .get(&(target.0, target.1, Tool::Torch))
        .copied()
        .context("target not reached")
}

/// Solves Year 2018, Day 22: Mode Maze.
///
/// # Errors
/// Returns an error if the input cannot be read or parsed.
pub fn solve() -> Result<(Answer, Answer)> {
    let (depth, target) = parse(&read_input(Year(2018), Day(22))?)?;
    Ok((
        u64::try_from(part_one(depth, START, target))?.into(),
        u64::try_from(part_two(depth, START, target)?)?.into(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_example;

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test1_part_one() -> Result<()> {
        let (depth, target) = parse(&read_example(Year(2018), Day(22), 1)?)?;
        assert_eq!(part_one(depth, START, target), 114);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test1_part_two() -> Result<()> {
        let (depth, target) = parse(&read_example(Year(2018), Day(22), 1)?)?;
        assert_eq!(part_two(depth, START, target)?, 45);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_solve() -> Result<()> {
        let (p1, p2) = solve()?;
        assert_eq!(p1, 5637.into());
        assert_eq!(p2, 969.into());
        Ok(())
    }
}
