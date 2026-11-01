use crate::utils::{Answer, Day, Year, read_input};
use anyhow::{Context, Result};
use std::collections::HashMap;

type IntType = i32;
type Point = (IntType, IntType);
type Cache = HashMap<Point, IntType>;

fn power_level(cache: &mut Cache, x: IntType, y: IntType, grid_serial_number: IntType) -> IntType {
    if let Some(level) = cache.get(&(x, y)) {
        return *level;
    }

    let rack_id = 10 + x;
    let mut power_level = rack_id * y;
    power_level += grid_serial_number;
    power_level *= rack_id;
    let mut digit = (power_level % 1000) / 100;
    digit -= 5;

    cache.insert((x, y), digit);

    digit
}

fn square_power_level(
    cache: &mut Cache,
    x_start: IntType,
    y_start: IntType,
    width: IntType,
    height: IntType,
    grid_serial_number: IntType,
) -> IntType {
    let mut level = 0;

    for x in x_start..x_start + width {
        for y in y_start..y_start + height {
            level += power_level(cache, x, y, grid_serial_number);
        }
    }

    level
}

fn part_one(grid_serial_number: IntType) -> ((IntType, IntType), IntType) {
    let mut max_square_power_level = IntType::MIN;
    let mut cache = Cache::new();
    let (width, height) = (3, 3);
    let (mut max_x, mut max_y) = (0, 0);
    for x in 1..=298 {
        for y in 1..=298 {
            let level = square_power_level(&mut cache, x, y, width, height, grid_serial_number);
            if level > max_square_power_level {
                max_x = x;
                max_y = y;
                max_square_power_level = level;
            }
        }
    }
    ((max_x, max_y), max_square_power_level)
}

/// Solves Year 2018, Day 11: Chronal Charge. The answer is the "x,y" of the best 3x3 square.
///
/// Part two is not solved: the original's brute force over every square size
/// never finished, so it returns 0.
///
/// # Errors
/// Returns an error if the input cannot be read or isn't a number.
pub fn solve() -> Result<(Answer, Answer)> {
    let input = read_input(Year(2018), Day(11))?;
    let grid_serial_number = input
        .trim()
        .parse()
        .with_context(|| format!("not a serial number: {input:?}"))?;
    let ((x, y), _) = part_one(grid_serial_number);
    Ok((format!("{x},{y}").into(), 0.into()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_example;

    /// Reads "x,y,serial" lines.
    fn read_cases(n: u8) -> Result<Vec<[IntType; 3]>> {
        read_example(Year(2018), Day(11), n)?
            .lines()
            .map(|l| {
                let values = l
                    .split(',')
                    .map(str::parse)
                    .collect::<Result<Vec<IntType>, _>>()?;
                values
                    .try_into()
                    .map_err(|v| anyhow::anyhow!("expected x,y,serial: {v:?}"))
            })
            .collect()
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_power_level() -> Result<()> {
        let mut cache = Cache::new();
        let cases = read_cases(1)?;
        let expected = [4, -5, 0, 4];
        assert_eq!(cases.len(), expected.len());
        for ([x, y, serial], want) in cases.into_iter().zip(expected) {
            cache.clear();
            assert_eq!(power_level(&mut cache, x, y, serial), want);
        }
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_square_power_level() -> Result<()> {
        let mut cache = Cache::new();
        let cases = read_cases(2)?;
        let expected = [29, 30];
        assert_eq!(cases.len(), expected.len());
        for ([x, y, serial], want) in cases.into_iter().zip(expected) {
            cache.clear();
            assert_eq!(square_power_level(&mut cache, x, y, 3, 3, serial), want);
        }
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_solve() -> Result<()> {
        let (p1, _) = solve()?;
        assert_eq!(p1, "243,17".into());
        Ok(())
    }
}
