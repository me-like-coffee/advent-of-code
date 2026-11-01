use crate::utils::grid::byte_grid;
use crate::utils::{Answer, Day, Year, read_input};
use anyhow::{Result, bail};
use std::collections::HashMap;

const CLEAN: u8 = b'.';
const INFECTED: u8 = b'#';
const WEAKENED: u8 = b'W';
const FLAGGED: u8 = b'F';

type IntType = i32;
type InfectionMap = HashMap<Point, u8>;

#[derive(Debug)]
enum Direction {
    East,
    North,
    South,
    West,
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Point {
    x: IntType,
    y: IntType,
}

impl Point {
    fn new(x: IntType, y: IntType) -> Point {
        Point { x, y }
    }
}

#[derive(Debug)]
struct Virus {
    heading: Direction,
    position: Point,
}

impl Virus {
    fn new(x: IntType, y: IntType, heading: Direction) -> Virus {
        Virus {
            heading,
            position: Point::new(x, y),
        }
    }

    fn turn_left(&mut self) {
        match self.heading {
            Direction::East => {
                self.heading = Direction::North;
            }
            Direction::North => {
                self.heading = Direction::West;
            }
            Direction::South => {
                self.heading = Direction::East;
            }
            Direction::West => {
                self.heading = Direction::South;
            }
        }
    }

    fn turn_right(&mut self) {
        match self.heading {
            Direction::East => {
                self.heading = Direction::South;
            }
            Direction::North => {
                self.heading = Direction::East;
            }
            Direction::South => {
                self.heading = Direction::West;
            }
            Direction::West => {
                self.heading = Direction::North;
            }
        }
    }

    fn advance(&mut self) {
        match self.heading {
            Direction::East => {
                self.position.x += 1;
            }
            Direction::North => {
                self.position.y -= 1;
            }
            Direction::South => {
                self.position.y += 1;
            }
            Direction::West => {
                self.position.x -= 1;
            }
        }
    }

    fn position(&self) -> Point {
        Point::new(self.position.x, self.position.y)
    }
}

/// Maps the grid onto coordinates centred on its middle node.
fn parse_infection_map(data: &str) -> InfectionMap {
    let mut infection_map = InfectionMap::new();
    let initial_state: Vec<Vec<_>> = byte_grid(data);

    let (x_mid, y_mid) = (
        initial_state[0].len() as IntType / 2,
        initial_state.len() as IntType / 2,
    );

    for (y, row) in initial_state.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            let x = x as IntType - x_mid;
            let y = y as IntType - y_mid;

            infection_map.insert(Point::new(x, y), c);
        }
    }
    infection_map
}

fn part_one(data: &str) -> Result<IntType> {
    let mut infection_map = parse_infection_map(data);
    let mut virus = Virus::new(0, 0, Direction::North);
    let mut new_infection_count = 0;

    for _ in 0..10000 {
        match infection_map.get(&virus.position()) {
            Some(&INFECTED) => {
                virus.turn_right();
                infection_map.insert(virus.position(), CLEAN);
            }
            Some(&CLEAN) | None => {
                virus.turn_left();
                infection_map.insert(virus.position(), INFECTED);
                new_infection_count += 1;
            }
            Some(&c) => bail!("unexpected node state {:?}", c as char),
        }
        virus.advance();
    }
    Ok(new_infection_count)
}

fn part_two(data: &str) -> Result<IntType> {
    let mut infection_map = parse_infection_map(data);
    let mut virus = Virus::new(0, 0, Direction::North);
    let mut new_infection_count = 0;

    for _ in 0..10000000 {
        match infection_map.get(&virus.position()) {
            Some(&INFECTED) => {
                virus.turn_right();
                infection_map.insert(virus.position(), FLAGGED);
            }
            Some(&CLEAN) | None => {
                virus.turn_left();
                infection_map.insert(virus.position(), WEAKENED);
            }
            Some(&WEAKENED) => {
                // Continue in current direction
                infection_map.insert(virus.position(), INFECTED);
                new_infection_count += 1;
            }
            Some(&FLAGGED) => {
                virus.turn_left();
                virus.turn_left();
                infection_map.insert(virus.position(), CLEAN);
            }
            Some(&c) => bail!("unexpected node state {:?}", c as char),
        }
        virus.advance();
    }
    Ok(new_infection_count)
}

/// Solves Year 2017, Day 22: Sporifica Virus.
///
/// # Errors
/// Returns an error if the input cannot be read or holds an unknown node state.
pub fn solve() -> Result<(Answer, Answer)> {
    let data = read_input(Year(2017), Day(22))?;
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
    fn test_part_one() -> Result<()> {
        assert_eq!(part_one(&read_example(Year(2017), Day(22), 1)?)?, 5587);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_part_two() -> Result<()> {
        assert_eq!(part_two(&read_example(Year(2017), Day(22), 1)?)?, 2511944);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_solve() -> Result<()> {
        let (p1, p2) = solve()?;
        assert_eq!(p1, 5182.into());
        assert_eq!(p2, 2512008.into());
        Ok(())
    }
}
