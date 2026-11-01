use crate::utils::{Answer, Day, Year, read_input};
use anyhow::{Context, Result, ensure};

type IntType = i64;

#[derive(Clone, Debug, PartialEq)]
pub struct Nanobot {
    x: IntType,
    y: IntType,
    z: IntType,
    r: IntType,
}

impl Nanobot {
    pub fn new(x: IntType, y: IntType, z: IntType, r: IntType) -> Nanobot {
        Nanobot { x, y, z, r }
    }

    // pos=<23253958,-36401441,-4040150>, r=97350345
    pub fn parse(data: &str) -> Result<Nanobot> {
        let (prefix, suffix) = data
            .split_once(">, r=")
            .with_context(|| format!("not a nanobot: {data:?}"))?;
        let prefix = prefix.replace("pos=<", "");
        let tokens: Vec<_> = prefix.split(',').collect();
        ensure!(tokens.len() == 3, "not a nanobot: {data:?}");
        let x = tokens[0].parse()?;
        let y = tokens[1].parse()?;
        let z = tokens[2].parse()?;
        let r = suffix.parse()?;

        Ok(Nanobot::new(x, y, z, r))
    }

    pub fn distance(&self, other: &Nanobot) -> IntType {
        IntType::abs(self.x - other.x)
            + IntType::abs(self.y - other.y)
            + IntType::abs(self.z - other.z)
    }

    pub fn in_range(&self, other: &Nanobot) -> bool {
        self.distance(other) <= self.r
    }
}

fn parse(data: &str) -> Result<Vec<Nanobot>> {
    let nanobots = data
        .lines()
        .map(Nanobot::parse)
        .collect::<Result<Vec<_>>>()?;
    ensure!(!nanobots.is_empty(), "no nanobots");
    Ok(nanobots)
}

fn part_one(data: &str) -> Result<usize> {
    let nanobots = parse(data)?;

    let mut largest_range_nanobot = &nanobots[0];
    for nanobot in &nanobots {
        if nanobot.r > largest_range_nanobot.r {
            largest_range_nanobot = nanobot;
        }
    }

    let in_range = nanobots
        .iter()
        .filter(|n| largest_range_nanobot.in_range(n))
        .count();

    Ok(in_range)
}

fn in_range_count(nanobots: &[Nanobot], point: &Nanobot) -> usize {
    nanobots.iter().filter(|n| n.in_range(point)).count()
}

/// A shrinking local search from the origin toward the point in range of the most nanobots.
fn part_two(data: &str) -> Result<IntType> {
    let nanobots = parse(data)?;

    let origin = Nanobot::new(0, 0, 0, 0);
    let mut position = Nanobot::new(0, 0, 0, 0);
    let mut span = 1000000000;
    let span_modifier = 0.98;
    let mut best_position = position.clone();
    let mut best_distance = IntType::MAX;
    let mut best_in_range_count = in_range_count(&nanobots, &position);
    let mut movement = true;

    while span > 1 || movement {
        movement = false;

        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    let new_x = x * span + position.x;
                    let new_y = y * span + position.y;
                    let new_z = z * span + position.z;
                    let new_position = Nanobot::new(new_x, new_y, new_z, 0);
                    let new_in_range_count = in_range_count(&nanobots, &new_position);
                    let new_distance = new_position.distance(&origin);

                    if new_in_range_count > best_in_range_count
                        || new_in_range_count == best_in_range_count && new_distance < best_distance
                    {
                        best_position = new_position.clone();
                        best_distance = new_distance;
                        best_in_range_count = new_in_range_count;
                        movement = true;
                    }
                }
            }
        }

        if best_position == position {
            span = (span_modifier * span as f64).floor() as i64;
        } else {
            position = best_position.clone();
        }
    }

    Ok(position.distance(&origin))
}

/// Solves Year 2018, Day 23: Experimental Emergency Teleportation.
///
/// # Errors
/// Returns an error if the input cannot be read or parsed.
pub fn solve() -> Result<(Answer, Answer)> {
    let data = read_input(Year(2018), Day(23))?;
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
        assert_eq!(part_one(&read_example(Year(2018), Day(23), 1)?)?, 7);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_part_two() -> Result<()> {
        assert_eq!(part_two(&read_example(Year(2018), Day(23), 1)?)?, 3);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_solve() -> Result<()> {
        let (p1, p2) = solve()?;
        assert_eq!(p1, 499.into());
        assert_eq!(p2, 80162663.into());
        Ok(())
    }
}
