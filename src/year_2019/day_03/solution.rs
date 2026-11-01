use crate::utils::{Answer, Day, Year, read_input};
use anyhow::{Result, bail};
use std::collections::HashMap;

type IntType = i32;
type Intersections = HashMap<(IntType, IntType), Vec<usize>>;

#[derive(Debug)]
struct Move {
    delta: (IntType, IntType),
    magnitude: usize,
}

impl Move {
    fn parse(data: &str) -> Result<Move> {
        let delta = match data.bytes().next() {
            Some(b'D') => (0, -1),
            Some(b'L') => (-1, 0),
            Some(b'R') => (1, 0),
            Some(b'U') => (0, 1),
            _ => bail!("unknown move {data:?}"),
        };
        let magnitude = data[1..].parse()?;
        Ok(Move { delta, magnitude })
    }
}

fn parse_line(data: &str) -> Result<Vec<Move>> {
    data.split(',').map(Move::parse).collect()
}

fn manhattan_distance(a: &(IntType, IntType), b: &(IntType, IntType)) -> IntType {
    IntType::abs(a.0 - b.0) + IntType::abs(a.1 - b.1)
}

/// Every point that more than one wire passes, with the wires that pass it in order.
fn intersections(wires: &[Vec<Move>]) -> Intersections {
    let mut points: HashMap<(IntType, IntType), Vec<usize>> = HashMap::new();
    let mut intersections: Intersections = HashMap::new();

    for (i, wire) in wires.iter().enumerate() {
        let (mut x, mut y) = (0, 0);
        for m in wire {
            let (delta_x, delta_y) = m.delta;

            for _ in 0..m.magnitude {
                x += delta_x;
                y += delta_y;
                if let Some(seen) = points.get_mut(&(x, y)) {
                    if let Some(crossing) = intersections.get_mut(&(x, y)) {
                        crossing.push(i);
                    } else {
                        intersections.insert((x, y), vec![seen[0], i]);
                    }
                    seen.push(i);
                } else {
                    points.insert((x, y), vec![i]);
                }
            }
        }
    }

    intersections
}

fn parse(data: &str) -> Result<Vec<Vec<Move>>> {
    data.lines().map(parse_line).collect()
}

fn part_one(data: &str) -> Result<IntType> {
    let wires = parse(data)?;

    let mut min_distance = IntType::MAX;
    for ((x, y), mut v) in intersections(&wires) {
        v.dedup();
        if v.len() == 2 {
            min_distance = min_distance.min(manhattan_distance(&(x, y), &(0, 0)));
        }
    }

    Ok(min_distance)
}

fn path_len(wire: &[Move], pt: &(IntType, IntType)) -> Result<IntType> {
    let (mut x, mut y) = (0, 0);
    let mut path_len = 0;

    for m in wire {
        let (delta_x, delta_y) = m.delta;

        for _ in 0..m.magnitude {
            x += delta_x;
            y += delta_y;
            path_len += 1;

            if x == pt.0 && y == pt.1 {
                return Ok(path_len);
            }
        }
    }

    bail!("wire never reaches {pt:?}")
}

fn part_two(data: &str) -> Result<IntType> {
    let wires = parse(data)?;
    let (wire_zero, wire_one) = match wires.as_slice() {
        [a, b, ..] => (a, b),
        _ => bail!("expected two wires"),
    };

    let mut min_distance = IntType::MAX;
    for ((x, y), v) in intersections(&wires) {
        let mut vv = v.clone();
        vv.dedup();
        if vv.len() == 2 {
            let path_zero = path_len(wire_zero, &(x, y))?;
            let path_one = path_len(wire_one, &(x, y))?;

            min_distance = min_distance.min(path_zero + path_one);
        }
    }

    Ok(min_distance)
}

/// Solves Year 2019, Day 3: Crossed Wires.
///
/// # Errors
/// Returns an error if the input cannot be read or parsed.
pub fn solve() -> Result<(Answer, Answer)> {
    let data = read_input(Year(2019), Day(3))?;
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
    fn test1_part_one() -> Result<()> {
        assert_eq!(part_one(&read_example(Year(2019), Day(3), 1)?)?, 6);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test1_part_two() -> Result<()> {
        assert_eq!(part_two(&read_example(Year(2019), Day(3), 1)?)?, 30);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_solve() -> Result<()> {
        let (p1, p2) = solve()?;
        assert_eq!(p1, 232.into());
        assert_eq!(p2, 6084.into());
        Ok(())
    }
}
