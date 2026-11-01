use crate::utils::{Answer, Day, Year, read_input};
use anyhow::{Result, ensure};

fn parse(data: &str) -> Vec<Vec<u8>> {
    data.lines()
        .map(|l| l.bytes().map(|b| b - b'0').collect::<Vec<_>>())
        .collect()
}

fn is_low_point(grid: &[Vec<u8>], x: usize, y: usize) -> bool {
    for delta in [1, 0, -1, 0, 1].windows(2) {
        let new_x = delta[0] + x as i32;
        let new_y = delta[1] + y as i32;

        if (0..grid.len() as i32).contains(&new_y)
            && (0..grid[0].len() as i32).contains(&new_x)
            && grid[new_y as usize][new_x as usize] <= grid[y][x]
        {
            return false;
        }
    }
    true
}

pub fn part_one(data: &str) -> u32 {
    let grid = parse(data);

    let mut risk_levels = Vec::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            if is_low_point(&grid, x, y) {
                risk_levels.push(1 + grid[y][x] as u32);
            }
        }
    }

    risk_levels.iter().sum()
}

fn flood_fill(
    grid: &[Vec<u8>],
    flooded_grid: &mut [Vec<u8>],
    x: usize,
    y: usize,
    value: u8,
    flood_value: u8,
) {
    for delta in [1, 0, -1, 0, 1].windows(2) {
        let new_x = delta[0] + x as i32;
        let new_y = delta[1] + y as i32;

        if (0..grid.len() as i32).contains(&new_y) && (0..grid[0].len() as i32).contains(&new_x) {
            let new_x = new_x as usize;
            let new_y = new_y as usize;
            if grid[new_y][new_x] > value && grid[new_y][new_x] < 9 {
                flooded_grid[new_y][new_x] = flood_value;
                flood_fill(
                    grid,
                    flooded_grid,
                    new_x,
                    new_y,
                    grid[new_y][new_x],
                    flood_value,
                );
            }
        }
    }
}

pub fn part_two(data: &str) -> Result<usize> {
    let grid = parse(data);

    let mut low_points = Vec::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            if is_low_point(&grid, x, y) {
                low_points.push((x, y));
            }
        }
    }

    let mut flooded_grid = vec![vec![u8::MAX; grid[0].len()]; grid.len()];
    for (flood_value, (x, y)) in low_points.iter().enumerate() {
        let flood_value = flood_value as u8;
        flooded_grid[*y][*x] = flood_value;
        flood_fill(&grid, &mut flooded_grid, *x, *y, grid[*y][*x], flood_value);
    }

    let mut flood_sizes = Vec::new();
    for i in 0..low_points.len() as u8 {
        flood_sizes.push(
            flooded_grid
                .iter()
                .flat_map(|r| r.iter())
                .filter(|f| **f == i)
                .count(),
        );
    }

    flood_sizes.sort();
    flood_sizes.reverse();
    ensure!(flood_sizes.len() >= 3, "fewer than 3 basins");

    Ok(flood_sizes[0] * flood_sizes[1] * flood_sizes[2])
}

/// Solves Year 2021, Day 9: Smoke Basin.
///
/// # Errors
/// Returns an error if the input cannot be read or parsed.
pub fn solve() -> Result<(Answer, Answer)> {
    let data = read_input(Year(2021), Day(9))?;
    Ok((
        u64::from(part_one(&data)).into(),
        u64::try_from(part_two(&data)?)?.into(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_example;

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_part_one() -> Result<()> {
        let data = read_example(Year(2021), Day(9), 1)?;
        assert_eq!(part_one(&data), 15);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_part_two() -> Result<()> {
        let data = read_example(Year(2021), Day(9), 1)?;
        assert_eq!(part_two(&data)?, 1134);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_solve() -> Result<()> {
        let (p1, p2) = solve()?;
        assert_eq!(p1, 539.into());
        assert_eq!(p2, 736920.into());
        Ok(())
    }
}
