// TBD use 4-tuple to represent points instead of Vec?
use crate::utils::{Answer, Day, Year, read_input};
use anyhow::{Context, Result};

struct PointGroup {
    points: Vec<Vec<i32>>,
}

impl PointGroup {
    pub fn new(point: &[i32]) -> PointGroup {
        PointGroup {
            points: vec![point.to_vec()],
        }
    }

    pub fn add_point(&mut self, point: &[i32]) {
        self.points.push(point.to_vec());
    }

    /// Groups are never empty: each starts with a point.
    pub fn min_distance(&self, point: &[i32]) -> i32 {
        self.points
            .iter()
            .map(|p| manhattan_distance(p, point))
            .min()
            .unwrap_or(i32::MAX)
    }

    pub fn merge(&mut self, other: &mut PointGroup) {
        self.points.append(&mut other.points)
    }
}

fn manhattan_distance(a: &[i32], b: &[i32]) -> i32 {
    a.iter().zip(b.iter()).map(|(i, j)| i32::abs(i - j)).sum()
}

fn parse_line(data: &str) -> Result<Vec<i32>> {
    data.split(',')
        .map(|l| {
            l.trim()
                .parse()
                .with_context(|| format!("not a point: {data:?}"))
        })
        .collect()
}

fn parse(data: &str) -> Result<Vec<Vec<i32>>> {
    data.lines().map(parse_line).collect()
}

fn constellations(data: &str) -> Result<usize> {
    let points = parse(data)?;
    let first = points.first().context("no points")?;

    let mut point_groups = vec![PointGroup::new(first)];
    for point in points.iter().skip(1) {
        let mut neighbors = Vec::new();
        for (i, pg) in point_groups.iter().enumerate() {
            if pg.min_distance(point) <= 3 {
                neighbors.push(i);
            }
        }
        if neighbors.is_empty() {
            point_groups.push(PointGroup::new(point));
        } else if neighbors.len() == 1 {
            point_groups[neighbors[0]].add_point(point);
        } else {
            neighbors.sort();
            let dest = neighbors.remove(0);
            for source in neighbors.iter().rev() {
                let mut other = point_groups.remove(*source);
                point_groups[dest].merge(&mut other);
            }
            point_groups[dest].add_point(point);
        }
    }

    Ok(point_groups.len())
}

/// Solves Year 2018, Day 25: Four-Dimensional Adventure. There is no second part.
///
/// # Errors
/// Returns an error if the input cannot be read or parsed.
pub fn solve() -> Result<(Answer, Answer)> {
    let data = read_input(Year(2018), Day(25))?;
    Ok((u64::try_from(constellations(&data)?)?.into(), 0.into()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_example;

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_examples() -> Result<()> {
        for (n, want) in [(1, 2), (2, 4), (3, 3), (4, 8)] {
            let data = read_example(Year(2018), Day(25), n)?;
            assert_eq!(constellations(&data)?, want, "example {n}");
        }
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_solve() -> Result<()> {
        let (p1, _) = solve()?;
        assert_eq!(p1, 396.into());
        Ok(())
    }
}
