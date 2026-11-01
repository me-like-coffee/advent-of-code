use crate::utils::{Answer, Day, Year, read_input};
use anyhow::{Context, Result};
use std::collections::HashSet;

#[derive(Debug)]
struct Coordinate {
    id: usize,
    x: usize,
    y: usize,
}

impl Coordinate {
    fn parse(data: &str, id: usize) -> Result<Coordinate> {
        let (prefix, suffix) = data
            .split_once(", ")
            .with_context(|| format!("not a coordinate: {data:?}"))?;
        let x = prefix.parse()?;
        let y = suffix.parse()?;

        Ok(Coordinate { id, x, y })
    }

    fn manhattan_distance(&self, x: usize, y: usize) -> usize {
        self.x.abs_diff(x) + self.y.abs_diff(y)
    }
}

fn extremes(coordinates: &[Coordinate]) -> Result<(usize, usize, usize, usize)> {
    let min_x = coordinates
        .iter()
        .map(|c| c.x)
        .min()
        .context("no coordinates")?;
    let max_x = coordinates
        .iter()
        .map(|c| c.x)
        .max()
        .context("no coordinates")?;
    let min_y = coordinates
        .iter()
        .map(|c| c.y)
        .min()
        .context("no coordinates")?;
    let max_y = coordinates
        .iter()
        .map(|c| c.y)
        .max()
        .context("no coordinates")?;

    Ok((min_x, max_x, min_y, max_y))
}

fn parse(data: &str) -> Result<Vec<Coordinate>> {
    data.lines()
        .enumerate()
        .map(|(i, l)| Coordinate::parse(l, i))
        .collect()
}

#[allow(clippy::needless_range_loop, reason = "indices mirror the grid layout")]
fn part_one(data: &str) -> Result<usize> {
    let coordinates = parse(data)?;

    let (_, max_x, _, max_y) = extremes(&coordinates)?;
    let x_bound = (max_x / 10) * 10 + 10;
    let y_bound = (max_y / 10) * 10 + 10;

    let mut grid = vec![vec![Vec::<(usize, usize)>::new(); x_bound]; y_bound];
    for coordinate in &coordinates {
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                let distance = coordinate.manhattan_distance(x, y);
                if !grid[y][x].is_empty() {
                    let curr_distance = grid[y][x][0].0;
                    if distance == curr_distance {
                        grid[y][x].push((distance, coordinate.id));
                    } else if distance < curr_distance {
                        grid[y][x].clear();
                        grid[y][x].push((distance, coordinate.id));
                    }
                } else {
                    grid[y][x].push((distance, coordinate.id));
                }
            }
        }
    }

    let mut perimeter = HashSet::new();
    for y in 0..grid.len() {
        if grid[y][0].len() == 1 {
            perimeter.insert(grid[y][0][0].1);
        }
        if grid[y][grid[y].len() - 1].len() == 1 {
            perimeter.insert(grid[y][grid[y].len() - 1][0].1);
        }
    }
    for x in 0..grid[0].len() {
        if grid[0][x].len() == 1 {
            perimeter.insert(grid[0][x][0].1);
        }
        if grid[grid.len() - 1][x].len() == 1 {
            perimeter.insert(grid[grid.len() - 1][x][0].1);
        }
    }

    let mut counts = vec![0; coordinates.len()];
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x].len() == 1 && !perimeter.contains(&grid[y][x][0].1) {
                counts[grid[y][x][0].1] += 1;
            }
        }
    }

    counts.iter().max().copied().context("no coordinates")
}

fn region(coordinates: &[Coordinate], total_distance: usize) -> Result<usize> {
    let (min_x, max_x, min_y, max_y) = extremes(coordinates)?;
    let mut max_region = 0;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let mut distance = 0;
            for coordinate in coordinates {
                distance += coordinate.manhattan_distance(x, y);
            }
            if distance < total_distance {
                max_region += 1;
            }
        }
    }
    Ok(max_region)
}

fn part_two(data: &str, total_distance: usize) -> Result<usize> {
    let coordinates = parse(data)?;
    region(&coordinates, total_distance)
}

/// Solves Year 2018, Day 6: Chronal Coordinates.
///
/// # Errors
/// Returns an error if the input cannot be read or parsed.
pub fn solve() -> Result<(Answer, Answer)> {
    let data = read_input(Year(2018), Day(6))?;
    Ok((
        u64::try_from(part_one(&data)?)?.into(),
        u64::try_from(part_two(&data, 10000)?)?.into(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_example;

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_part_one() -> Result<()> {
        assert_eq!(part_one(&read_example(Year(2018), Day(6), 1)?)?, 17);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_part_two() -> Result<()> {
        assert_eq!(part_two(&read_example(Year(2018), Day(6), 1)?, 32)?, 16);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_solve() -> Result<()> {
        let (p1, p2) = solve()?;
        assert_eq!(p1, 4171.into());
        assert_eq!(p2, 39545.into());
        Ok(())
    }
}
