use crate::utils::{Answer, Day, Year, read_input};
use anyhow::{Context, Result};

type IntType = i32;

#[derive(Debug)]
struct Point {
    x: IntType,
    y: IntType,
}

impl Point {
    fn new(x: IntType, y: IntType) -> Point {
        Point { x, y }
    }

    // 462,215
    fn parse(data: &str) -> Result<Point> {
        let (x, y) = data
            .split_once(',')
            .with_context(|| format!("not a point: {data:?}"))?;
        Ok(Point::new(x.parse()?, y.parse()?))
    }
}

#[derive(Debug)]
struct Span {
    a: Point,
    b: Point,
}

impl Span {
    fn is_horizontal(&self) -> bool {
        self.a.y == self.b.y
    }

    fn is_vertical(&self) -> bool {
        self.a.x == self.b.x
    }
}

// 462,215 -> 462,747
fn parse(data: &str) -> Result<Vec<Span>> {
    data.lines()
        .map(|line| {
            let (prefix, suffix) = line
                .split_once(" -> ")
                .with_context(|| format!("not a line segment: {line:?}"))?;
            Ok(Span {
                a: Point::parse(prefix)?,
                b: Point::parse(suffix)?,
            })
        })
        .collect()
}

/// Counts the points covered by more than one line; diagonal lines count only if `diagonals`.
fn overlaps(data: &str, diagonals: bool) -> Result<usize> {
    let spans = parse(data)?;

    let mut grid = vec![vec![0; 1000]; 1000];
    for s in &spans {
        if s.is_horizontal() {
            let start = s.a.x.min(s.b.x) as usize;
            let end = s.a.x.max(s.b.x) as usize;
            for cell in &mut grid[s.a.y as usize][start..=end] {
                *cell += 1;
            }
        } else if s.is_vertical() {
            let start = s.a.y.min(s.b.y) as usize;
            let end = s.a.y.max(s.b.y) as usize;
            for row in &mut grid[start..=end] {
                row[s.a.x as usize] += 1;
            }
        } else if diagonals {
            let delta_x = if s.a.x < s.b.x { 1 } else { -1 };
            let delta_y = if s.a.y < s.b.y { 1 } else { -1 };
            let mut x = s.a.x;
            let mut y = s.a.y;
            let num_steps = if delta_x == 1 {
                s.b.x - s.a.x + 1
            } else {
                s.a.x - s.b.x + 1
            };
            for _ in 0..num_steps {
                grid[y as usize][x as usize] += 1;
                x += delta_x;
                y += delta_y;
            }
        }
    }
    Ok(grid
        .iter()
        .flat_map(|r| r.iter())
        .filter(|n| **n > 1)
        .count())
}

/// Solves Year 2021, Day 5: Hydrothermal Venture.
///
/// # Errors
/// Returns an error if the input cannot be read or parsed.
pub fn solve() -> Result<(Answer, Answer)> {
    let data = read_input(Year(2021), Day(5))?;
    Ok((
        u64::try_from(overlaps(&data, false)?)?.into(),
        u64::try_from(overlaps(&data, true)?)?.into(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_example;

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_examples() -> Result<()> {
        let data = read_example(Year(2021), Day(5), 1)?;
        assert_eq!(overlaps(&data, false)?, 5);
        assert_eq!(overlaps(&data, true)?, 12);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_solve() -> Result<()> {
        let (p1, p2) = solve()?;
        assert_eq!(p1, 3990.into());
        assert_eq!(p2, 21305.into());
        Ok(())
    }
}
