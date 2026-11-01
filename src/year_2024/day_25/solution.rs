use crate::utils::{Answer, Day, Year, read_input, sections};
use anyhow::Result;

/// A schematic represented by the heights of its 5 columns.
#[derive(Debug)]
struct Schematic {
    heights: [u8; 5],
    total_volume: u8,
}

impl Schematic {
    /// Parses a 7-line block into a Schematic.
    fn parse(block: &str) -> Option<(Self, bool)> {
        let lines: Vec<&str> = block.lines().collect();
        if lines.len() < 7 {
            return None;
        }

        let is_lock = lines[0] == "#####";
        let mut heights = [0u8; 5];
        let mut total_volume = 0;

        for (col, height) in heights.iter_mut().enumerate() {
            // Scan middle 5 rows (indices 1 to 5)
            for line in lines.iter().skip(1).take(5) {
                if line.chars().nth(col)? == '#' {
                    *height += 1;
                }
            }
            total_volume += *height;
        }

        Some((
            Self {
                heights,
                total_volume,
            },
            is_lock,
        ))
    }

    /// Checks if this schematic (assumed to be a lock) fits with a key.
    /// Incorporates the volume pruning heuristic: total sum must be <= 25.
    fn fits(&self, key: &Schematic) -> bool {
        // Pruning heuristic
        if self.total_volume + key.total_volume > 25 {
            return false;
        }

        // Full fitting check
        for i in 0..5 {
            if self.heights[i] + key.heights[i] > 5 {
                return false;
            }
        }
        true
    }
}

/// Solves Year 2024, Day 25: Code Chronicle.
pub fn solve() -> Result<(Answer, Answer)> {
    let input = read_input(Year(2024), Day(25))?;
    let blocks: Vec<&str> = sections(&input)
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    let mut locks = Vec::new();
    let mut keys = Vec::new();

    for block in blocks {
        if let Some((schematic, is_lock)) = Schematic::parse(block) {
            if is_lock {
                locks.push(schematic);
            } else {
                keys.push(schematic);
            }
        }
    }

    let mut fit_count = 0;
    for lock in &locks {
        for key in &keys {
            if lock.fits(key) {
                fit_count += 1;
            }
        }
    }

    // Day 25 usually only has one part.
    Ok((fit_count.into(), 0.into()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_example;

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_day_25_example() -> Result<()> {
        let example = &read_example(Year(2024), Day(25), 1)?;

        let blocks: Vec<&str> = sections(example).collect();
        let mut locks = Vec::new();
        let mut keys = Vec::new();

        for block in blocks {
            if let Some((schematic, is_lock)) = Schematic::parse(block) {
                if is_lock {
                    locks.push(schematic);
                } else {
                    keys.push(schematic);
                }
            }
        }

        let mut fit_count = 0;
        for lock in &locks {
            for key in &keys {
                if lock.fits(key) {
                    fit_count += 1;
                }
            }
        }
        assert_eq!(fit_count, 3);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_solve() -> Result<()> {
        let (p1, _) = solve()?;
        assert_eq!(p1, 3525.into());
        Ok(())
    }
}
