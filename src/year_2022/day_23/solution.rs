use crate::utils::{Answer, Day, Year, read_input};
use anyhow::{Context, Result};
use std::collections::{HashMap, HashSet};
use std::ops::Add;

type IntType = i32;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: IntType,
    y: IntType,
}

impl Point {
    fn new(x: IntType, y: IntType) -> Point {
        Point { x, y }
    }
}

impl Add for &Point {
    type Output = Point;

    fn add(self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug)]
struct BoundingRect {
    ul: Point,
    lr: Point,
}

impl BoundingRect {
    fn new(positions: &HashSet<Point>) -> Result<BoundingRect> {
        let ul = Point::new(
            positions.iter().map(|pt| pt.x).min().context("no elves")?,
            positions.iter().map(|pt| pt.y).min().context("no elves")?,
        );
        let lr = Point::new(
            positions.iter().map(|pt| pt.x).max().context("no elves")?,
            positions.iter().map(|pt| pt.y).max().context("no elves")?,
        );

        Ok(BoundingRect { ul, lr })
    }

    fn area(&self) -> IntType {
        (self.lr.x - self.ul.x + 1) * (self.lr.y - self.ul.y + 1)
    }
}

#[derive(Debug)]
struct Elves {
    points: HashSet<Point>,
    deltas: [Point; 4],
    frontiers: [[Point; 3]; 4],
    next_direction: usize,
}

impl Elves {
    pub fn parse(data: &str) -> Elves {
        let mut points = HashSet::new();
        for (y, row) in data.lines().enumerate() {
            for (x, c) in row.bytes().enumerate() {
                if c == b'#' {
                    points.insert(Point::new(x as i32, y as i32));
                }
            }
        }

        // North, South, West, East
        let deltas = [
            Point::new(0, -1),
            Point::new(0, 1),
            Point::new(-1, 0),
            Point::new(1, 0),
        ];
        let frontiers = [
            [Point::new(-1, -1), Point::new(0, -1), Point::new(1, -1)],
            [Point::new(-1, 1), Point::new(0, 1), Point::new(1, 1)],
            [Point::new(-1, -1), Point::new(-1, 0), Point::new(-1, 1)],
            [Point::new(1, -1), Point::new(1, 0), Point::new(1, 1)],
        ];
        let next_direction = 0;

        Elves {
            points,
            deltas,
            frontiers,
            next_direction,
        }
    }

    pub fn update(&mut self) -> bool {
        let mut proposed_moves: HashMap<Point, Vec<Point>> = HashMap::new();
        for point in &self.points {
            if !self.has_neighbor(point, 0)
                && !self.has_neighbor(point, 1)
                && !self.has_neighbor(point, 2)
                && !self.has_neighbor(point, 3)
            {
                continue;
            }

            let mut direction = self.next_direction;
            for _ in 0..4 {
                if !self.has_neighbor(point, direction) {
                    let move_point = Point::new(
                        point.x + self.deltas[direction].x,
                        point.y + self.deltas[direction].y,
                    );
                    proposed_moves
                        .entry(move_point.clone())
                        .or_default()
                        .push(point.clone());
                    break;
                }
                direction = (direction + 1) % self.deltas.len();
            }
        }

        if proposed_moves.is_empty() || proposed_moves.iter().all(|(_, v)| v.len() > 1) {
            return false;
        }

        for (point, v) in proposed_moves {
            if v.len() == 1 {
                self.points.remove(&v[0]);
                self.points.insert(point);
            }
        }

        self.next_direction = (self.next_direction + 1) % self.deltas.len();

        true
    }

    pub fn get_bounding_rect(&self) -> Result<BoundingRect> {
        BoundingRect::new(&self.points)
    }

    pub fn empty_tile_count(&self) -> Result<IntType> {
        Ok(self.get_bounding_rect()?.area() - self.points.len() as IntType)
    }

    fn has_neighbor(&self, point: &Point, direction: usize) -> bool {
        for frontier in &self.frontiers[direction] {
            let neighbor = point + frontier;
            if self.points.contains(&neighbor) {
                return true;
            }
        }

        false
    }
}

fn part_one(data: &str, num_rounds: usize) -> Result<IntType> {
    let mut elves = Elves::parse(data);

    for _ in 0..num_rounds {
        elves.update();
    }

    elves.empty_tile_count()
}

fn part_two(data: &str) -> usize {
    let mut elves = Elves::parse(data);
    let mut num_rounds = 1;

    while elves.update() {
        num_rounds += 1;
    }

    num_rounds
}

/// Solves Year 2022, Day 23: Unstable Diffusion.
///
/// # Errors
/// Returns an error if the input cannot be read or parsed.
pub fn solve() -> Result<(Answer, Answer)> {
    let data = read_input(Year(2022), Day(23))?;
    Ok((
        u64::try_from(part_one(&data, 10)?)?.into(),
        u64::try_from(part_two(&data))?.into(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_example;

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_part_one() -> Result<()> {
        let data = read_example(Year(2022), Day(23), 2)?;
        assert_eq!(part_one(&data, 10)?, 110);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_part_two() -> Result<()> {
        let data = read_example(Year(2022), Day(23), 2)?;
        assert_eq!(part_two(&data), 20);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_solve() -> Result<()> {
        let (p1, p2) = solve()?;
        assert_eq!(p1, 4249.into());
        assert_eq!(p2, 980.into());
        Ok(())
    }
}
