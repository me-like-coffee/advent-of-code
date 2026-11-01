use crate::utils::{Answer, Day, Year, read_input};
use anyhow::{Context, Result};
use std::collections::HashMap;

fn part_one(target: u32) -> u32 {
    let mut a = 3;

    while a * a < target {
        a += 2;
    }
    let b = a - 1;
    let c = a - 2;

    (b / 2) + (c / 2).abs_diff((target - c * c - 1) % b)
}

#[derive(Clone, Debug)]
enum Heading {
    East,
    North,
    South,
    West,
}

#[derive(Clone, Debug)]
struct Direction {
    heading: Heading,
}

impl Direction {
    fn new(heading: Heading) -> Direction {
        Direction { heading }
    }

    fn delta(&self) -> [i32; 2] {
        match self.heading {
            Heading::East => [1, 0],
            Heading::North => [0, -1],
            Heading::South => [0, 1],
            Heading::West => [-1, 0],
        }
    }

    fn counter_clockwise(&mut self) {
        match self.heading {
            Heading::East => self.heading = Heading::North,
            Heading::North => self.heading = Heading::West,
            Heading::South => self.heading = Heading::East,
            Heading::West => self.heading = Heading::South,
        }
    }
}

fn part_two(target: i32) -> i32 {
    let mut size = 2;
    let mut position = [1, 0];
    let mut direction = Direction::new(Heading::North);
    let mut left = Direction::new(Heading::West);

    let mut values = HashMap::from([([0, 0], 1)]);

    'outer: loop {
        for edge in 0..4 {
            for i in 0..size {
                let value = |point| *values.get(&point).unwrap_or(&0);
                let mut tmp = [
                    position[0] - direction.delta()[0],
                    position[1] - direction.delta()[1],
                ];
                let mut next = value(tmp);
                tmp = [
                    position[0] + left.delta()[0] + direction.delta()[0],
                    position[1] + left.delta()[1] + direction.delta()[1],
                ];
                next += value(tmp);
                tmp = [position[0] + left.delta()[0], position[1] + left.delta()[1]];
                next += value(tmp);
                tmp = [
                    position[0] + left.delta()[0] - direction.delta()[0],
                    position[1] + left.delta()[1] - direction.delta()[1],
                ];
                next += value(tmp);

                if next > target {
                    break 'outer next;
                }

                values.insert(position, next);
                if i == size - 1 && edge < 3 {
                    position[0] += left.delta()[0];
                    position[1] += left.delta()[1];
                } else {
                    position[0] += direction.delta()[0];
                    position[1] += direction.delta()[1];
                }
            }
            direction = left.clone();
            left.counter_clockwise();
        }

        size += 2;
    }
}

/// Solves Year 2017, Day 3: Spiral Memory.
///
/// # Errors
/// Returns an error if the input cannot be read or isn't a number.
pub fn solve() -> Result<(Answer, Answer)> {
    let input = read_input(Year(2017), Day(3))?;
    let target: u32 = input
        .trim()
        .parse()
        .with_context(|| format!("not a number: {input:?}"))?;
    let p1 = part_one(target);
    let p2 = part_two(i32::try_from(target)?);
    Ok((u64::from(p1).into(), u64::try_from(p2)?.into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_solve() -> Result<()> {
        let (p1, p2) = solve()?;
        assert_eq!(p1, 438.into());
        assert_eq!(p2, 266330.into());
        Ok(())
    }
}
