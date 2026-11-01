use crate::utils::{Answer, Day, Year, read_input};
use anyhow::{Context, Result};

type IntType = usize;

fn parse(data: &str) -> Result<Vec<IntType>> {
    let mut adapters = data
        .lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<IntType>, _>>()?;
    adapters.sort_unstable();
    Ok(adapters)
}

fn part_one(data: &str) -> Result<usize> {
    let adapters = parse(data)?;
    let (mut ones, mut threes) = (1, 1);
    for pair in adapters.windows(2) {
        if pair[1] - pair[0] == 1 {
            ones += 1;
        } else {
            threes += 1;
        }
    }

    Ok(ones * threes)
}

fn part_two(data: &str) -> Result<usize> {
    let adapters = parse(data)?;
    let last = adapters.last().context("no adapters")?;
    let mut sum = vec![0; last + 1];
    sum[0] = 1;

    for i in adapters {
        match i {
            1 => sum[i] = sum[i - 1],
            2 => sum[i] = sum[i - 1] + sum[i - 2],
            _ => sum[i] = sum[i - 1] + sum[i - 2] + sum[i - 3],
        }
    }

    sum.last().copied().context("no adapters")
}

/// Solves Year 2020, Day 10: Adapter Array.
///
/// # Errors
/// Returns an error if the input cannot be read or parsed.
pub fn solve() -> Result<(Answer, Answer)> {
    let data = read_input(Year(2020), Day(10))?;
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
    fn test_examples() -> Result<()> {
        let first = read_example(Year(2020), Day(10), 1)?;
        let second = read_example(Year(2020), Day(10), 2)?;
        assert_eq!(part_one(&first)?, 35);
        assert_eq!(part_one(&second)?, 220);
        assert_eq!(part_two(&first)?, 8);
        assert_eq!(part_two(&second)?, 19208);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_solve() -> Result<()> {
        let (p1, p2) = solve()?;
        assert_eq!(p1, 2738.into());
        assert_eq!(p2, 74049191673856.into());
        Ok(())
    }
}
