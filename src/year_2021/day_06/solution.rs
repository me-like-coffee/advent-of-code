use crate::utils::{Answer, Day, Year, read_input};
use anyhow::{Context, Result};

type IntType = u64;

/// Counts the fish after `num_days`, tracking how many have each timer value.
fn count_fish(data: &str, num_days: usize) -> Result<IntType> {
    let mut fish = [0 as IntType; 9];
    for t in data.trim().split(',') {
        let n: usize = t.parse()?;
        *fish
            .get_mut(n)
            .with_context(|| format!("timer {n} out of range"))? += 1;
    }

    for _ in 0..num_days {
        let zero_count = fish[0];
        fish.rotate_left(1);
        fish[6] += zero_count;
    }

    Ok(fish.iter().sum())
}

/// Solves Year 2021, Day 6: Lanternfish.
///
/// # Errors
/// Returns an error if the input cannot be read or parsed.
pub fn solve() -> Result<(Answer, Answer)> {
    let data = read_input(Year(2021), Day(6))?;
    Ok((
        count_fish(&data, 80)?.into(),
        count_fish(&data, 256)?.into(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_example;

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_examples() -> Result<()> {
        let data = read_example(Year(2021), Day(6), 1)?;
        assert_eq!(count_fish(&data, 18)?, 26);
        assert_eq!(count_fish(&data, 80)?, 5934);
        assert_eq!(count_fish(&data, 256)?, 26984457539);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_solve() -> Result<()> {
        let (p1, p2) = solve()?;
        assert_eq!(p1, 380243.into());
        assert_eq!(p2, 1708791884591.into());
        Ok(())
    }
}
