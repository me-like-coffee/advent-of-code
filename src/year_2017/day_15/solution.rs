use crate::utils::{Answer, Day, Year, read_input};
use anyhow::{Context, Result, ensure};

fn next_value(seed: u64, factor: u64, modulus: u64) -> u64 {
    let mut curr_value = seed;
    loop {
        curr_value = (curr_value * factor) % 2147483647;
        if curr_value.is_multiple_of(modulus) {
            break;
        }
    }
    curr_value
}

fn are_pair(a: u64, b: u64) -> bool {
    let a_masked = a & 0xffff;
    let b_masked = b & 0xffff;

    a_masked ^ b_masked == 0
}

fn count_pairs(a_seed: u64, b_seed: u64, a_modulus: u64, b_modulus: u64, iterations: u64) -> u64 {
    let mut num_pairs = 0;
    let mut a = a_seed;
    let a_factor = 16807;
    let mut b = b_seed;
    let b_factor = 48271;

    for _ in 0..iterations {
        a = next_value(a, a_factor, a_modulus);
        b = next_value(b, b_factor, b_modulus);
        num_pairs += if are_pair(a, b) { 1 } else { 0 };
    }

    num_pairs
}

/// Parses the two "Generator X starts with N" lines into the seeds.
fn parse(data: &str) -> Result<(u64, u64)> {
    let seeds = data
        .lines()
        .map(|l| {
            l.split_whitespace()
                .last()
                .with_context(|| format!("empty line: {l:?}"))?
                .parse::<u64>()
                .with_context(|| format!("no seed in {l:?}"))
        })
        .collect::<Result<Vec<_>>>()?;
    ensure!(
        seeds.len() == 2,
        "expected 2 generators, found {}",
        seeds.len()
    );
    Ok((seeds[0], seeds[1]))
}

/// Solves Year 2017, Day 15: Dueling Generators.
///
/// # Errors
/// Returns an error if the input cannot be read or parsed.
pub fn solve() -> Result<(Answer, Answer)> {
    let (a, b) = parse(&read_input(Year(2017), Day(15))?)?;
    Ok((
        count_pairs(a, b, 1, 1, 40000000).into(),
        count_pairs(a, b, 4, 8, 5000000).into(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_example;

    #[test]
    fn test_generator() {
        let a_expected = vec![65, 1092455, 1181022009, 245556042, 1744312007, 1352636452];
        let mut a_actual = vec![0; 6];
        let a_factor = 16807;
        a_actual[0] = 65;
        for i in 1..a_actual.len() {
            a_actual[i] = next_value(a_actual[i - 1], a_factor, 1);
        }
        assert_eq!(a_actual, a_expected);
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_part_one_small() -> Result<()> {
        let (a, b) = parse(&read_example(Year(2017), Day(15), 1)?)?;
        assert_eq!(count_pairs(a, b, 1, 1, 5), 1);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_part_two_small() -> Result<()> {
        let (a, b) = parse(&read_example(Year(2017), Day(15), 1)?)?;
        assert_eq!(count_pairs(a, b, 4, 8, 5), 0);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_part_one_large() -> Result<()> {
        let (a, b) = parse(&read_example(Year(2017), Day(15), 1)?)?;
        assert_eq!(count_pairs(a, b, 1, 1, 40000000), 588);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_part_two_large() -> Result<()> {
        let (a, b) = parse(&read_example(Year(2017), Day(15), 1)?)?;
        assert_eq!(count_pairs(a, b, 4, 8, 5000000), 309);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_solve() -> Result<()> {
        let (p1, p2) = solve()?;
        assert_eq!(p1, 573.into());
        assert_eq!(p2, 294.into());
        Ok(())
    }
}
