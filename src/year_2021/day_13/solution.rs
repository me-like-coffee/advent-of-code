use crate::utils::{Answer, Day, Year, read_input};
use anyhow::{Context, Result, bail};
use std::collections::BTreeSet;

// 6,12
fn parse_points(data: &str) -> Result<BTreeSet<(usize, usize)>> {
    data.lines()
        .map(|l| {
            let (x_data, y_data) = l
                .split_once(',')
                .with_context(|| format!("not a point: {l:?}"))?;
            Ok((x_data.parse()?, y_data.parse()?))
        })
        .collect()
}

// fold along y=7
fn parse_folds(data: &str) -> Result<Vec<(u8, usize)>> {
    data.lines()
        .map(|l| {
            let (axis_data, location_data) = l
                .split_once('=')
                .with_context(|| format!("not a fold: {l:?}"))?;
            let axis = axis_data.bytes().last().context("no fold axis")?;
            Ok((axis, location_data.parse()?))
        })
        .collect()
}

type Folds = (BTreeSet<(usize, usize)>, Vec<(u8, usize)>);

fn parse(data: &str) -> Result<Folds> {
    let (point_data, fold_data) = data
        .split_once("\n\n")
        .context("expected points, a blank line, then folds")?;
    Ok((parse_points(point_data)?, parse_folds(fold_data)?))
}

fn fold(
    points: &BTreeSet<(usize, usize)>,
    axis: u8,
    location: usize,
) -> Result<BTreeSet<(usize, usize)>> {
    match axis {
        b'x' => Ok(fold_left(points, location)),
        b'y' => Ok(fold_up(points, location)),
        _ => bail!("unknown fold axis {:?}", axis as char),
    }
}

/// Renders the dots, which spell the part-two answer in capital letters.
fn render(points: &BTreeSet<(usize, usize)>) -> String {
    let x_max = points.iter().map(|(x, _)| *x).max().unwrap_or(0);
    let y_max = points.iter().map(|(_, y)| *y).max().unwrap_or(0);

    let mut grid = vec![vec!['.'; x_max + 1]; y_max + 1];
    for (x, y) in points {
        grid[*y][*x] = '#';
    }

    grid.iter()
        .map(|row| row.iter().collect::<String>() + "\n")
        .collect()
}

fn fold_up(points: &BTreeSet<(usize, usize)>, y_axis: usize) -> BTreeSet<(usize, usize)> {
    let mut folded_points: BTreeSet<(usize, usize)> = points
        .iter()
        .filter(|(_, y)| y < &y_axis)
        .map(|(x, y)| (*x, *y))
        .collect();

    for (x, y) in points.iter().filter(|(_, y)| y > &y_axis) {
        let delta_y = *y - y_axis;
        let new_y = *y - 2 * delta_y;
        folded_points.insert((*x, new_y));
    }

    folded_points
}

fn fold_left(points: &BTreeSet<(usize, usize)>, x_axis: usize) -> BTreeSet<(usize, usize)> {
    let mut folded_points: BTreeSet<(usize, usize)> = points
        .iter()
        .filter(|(x, _)| x < &x_axis)
        .map(|(x, y)| (*x, *y))
        .collect();

    for (x, y) in points.iter().filter(|(x, _)| x > &x_axis) {
        let delta_x = *x - x_axis;
        let new_x = *x - 2 * delta_x;
        folded_points.insert((new_x, *y));
    }

    folded_points
}

fn part_one(data: &str, num_folds: usize) -> Result<usize> {
    let (mut points, folds) = parse(data)?;

    for &(axis, location) in folds.iter().take(num_folds) {
        points = fold(&points, axis, location)?;
    }

    Ok(points.len())
}

/// Applies every fold and returns the remaining dots.
fn fold_all(data: &str) -> Result<BTreeSet<(usize, usize)>> {
    let (mut points, folds) = parse(data)?;

    for (axis, location) in folds {
        points = fold(&points, axis, location)?;
    }

    Ok(points)
}

/// Solves Year 2021, Day 13: Transparent Origami.
///
/// The part-two answer is the capital letters the folded dots spell: they are
/// printed, and 0 is returned, as for 2019 day 11.
///
/// # Errors
/// Returns an error if the input cannot be read or parsed.
pub fn solve() -> Result<(Answer, Answer)> {
    let data = read_input(Year(2021), Day(13))?;
    let p1 = part_one(&data, 1)?;
    println!("{}", render(&fold_all(&data)?));
    Ok((u64::try_from(p1)?.into(), 0.into()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_example;

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test1_part_one() -> Result<()> {
        let data = read_example(Year(2021), Day(13), 1)?;
        assert_eq!(part_one(&data, 1)?, 17);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test1_part_two() -> Result<()> {
        let data = read_example(Year(2021), Day(13), 1)?;
        assert_eq!(fold_all(&data)?.len(), 16);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_solve() -> Result<()> {
        let (p1, _) = solve()?;
        assert_eq!(p1, 695.into());
        Ok(())
    }
}
