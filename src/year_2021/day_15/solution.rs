use crate::utils::{Answer, Day, Year, read_input};
use anyhow::{Context, Result, ensure};
use std::collections::VecDeque;

/// The input is a grid of risk levels, at most 100x100.
type RiskLevels = [[u8; 100]; 100];

fn parse(data: &str) -> Result<(Vec<&str>, RiskLevels)> {
    let lines: Vec<_> = data.lines().collect();
    ensure!(
        lines.len() <= 100 && lines.iter().all(|l| l.len() <= 100),
        "grid larger than 100x100"
    );

    let mut risk_level = [[u8::MAX; 100]; 100];
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            risk_level[y][x] = c as u8 - b'0';
        }
    }
    Ok((lines, risk_level))
}

/// The risk at (x, y) of the map tiled five times; `width` and `height` are the grid's size.
fn lookup_risk_value(
    risk_level: &RiskLevels,
    width: usize,
    height: usize,
    x: usize,
    y: usize,
) -> u8 {
    let x_multiple = x / width;
    let y_multiple = y / height;
    let x_index = x % width;
    let y_index = y % height;
    let value = risk_level[y_index][x_index] + x_multiple as u8 + y_multiple as u8;

    if value >= 10 { value - 9 } else { value }
}

fn part_one(data: &str) -> Result<u64> {
    let (lines, risk_level) = parse(data)?;

    let (y_goal, x_goal) = (
        lines.len() - 1,
        lines.last().context("empty grid")?.len() - 1,
    );

    let mut total_risk = [[u64::MAX; 100]; 100];
    total_risk[0][0] = 0;

    let mut q = VecDeque::new();
    q.push_back((0, 0));

    while let Some((x, y)) = q.pop_front() {
        for delta in [1, 0, -1, 0, 1].windows(2) {
            let new_x = x + delta[0];
            let new_y = y + delta[1];

            if (0..=y_goal as i32).contains(&new_y) && (0..=x_goal as i32).contains(&new_x) {
                let distance = total_risk[y as usize][x as usize]
                    + risk_level[new_y as usize][new_x as usize] as u64;
                if distance < total_risk[new_y as usize][new_x as usize] {
                    total_risk[new_y as usize][new_x as usize] = distance;
                    q.push_back((new_x, new_y));
                }
            }
        }
    }

    Ok(total_risk[y_goal][x_goal])
}

fn part_two(data: &str) -> Result<u64> {
    let (lines, risk_level) = parse(data)?;

    let (height, width) = (lines.len(), lines.last().context("empty grid")?.len());
    let (y_goal, x_goal) = (5 * height - 1, 5 * width - 1);

    // 2 MB, so on the heap: that is a test thread's whole stack.
    let mut total_risk = vec![[u64::MAX; 500]; 500];
    total_risk[0][0] = 0;

    let mut q = VecDeque::new();
    q.push_back((0, 0));

    while let Some((x, y)) = q.pop_front() {
        for delta in [1, 0, -1, 0, 1].windows(2) {
            let new_x = x + delta[0];
            let new_y = y + delta[1];

            if (0..=y_goal as i32).contains(&new_y) && (0..=x_goal as i32).contains(&new_x) {
                let risk_value =
                    lookup_risk_value(&risk_level, width, height, new_x as usize, new_y as usize)
                        as u64;
                let distance = total_risk[y as usize][x as usize] + risk_value;
                if distance < total_risk[new_y as usize][new_x as usize] {
                    total_risk[new_y as usize][new_x as usize] = distance;
                    q.push_back((new_x, new_y));
                }
            }
        }
    }

    Ok(total_risk[y_goal][x_goal])
}

/// Solves Year 2021, Day 15: Chiton.
///
/// # Errors
/// Returns an error if the input cannot be read or parsed.
pub fn solve() -> Result<(Answer, Answer)> {
    let data = read_input(Year(2021), Day(15))?;
    Ok((part_one(&data)?.into(), part_two(&data)?.into()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_example;

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_part_one() -> Result<()> {
        let data = read_example(Year(2021), Day(15), 1)?;
        assert_eq!(part_one(&data)?, 40);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_part_two() -> Result<()> {
        let data = read_example(Year(2021), Day(15), 1)?;
        assert_eq!(part_two(&data)?, 315);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_solve() -> Result<()> {
        let (p1, p2) = solve()?;
        assert_eq!(p1, 702.into());
        assert_eq!(p2, 2955.into());
        Ok(())
    }
}
