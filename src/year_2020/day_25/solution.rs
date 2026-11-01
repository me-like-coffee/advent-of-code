use crate::utils::{Answer, Day, Year, read_input};
use anyhow::{Result, ensure};

const SUBJECT: usize = 7;
const MODULO: usize = 20201227;

fn part_one(card: usize, door: usize) -> usize {
    let mut card_loop_size = 0;
    let mut card_value = 1;

    while card_value != card {
        card_loop_size += 1;
        card_value *= SUBJECT;
        card_value %= MODULO;
    }

    let mut key = 1;
    for _ in 0..card_loop_size {
        key *= door;
        key %= MODULO;
    }

    key
}

/// Parses the card's and the door's public keys, one per line.
fn parse(data: &str) -> Result<(usize, usize)> {
    let keys = data
        .lines()
        .map(|l| l.trim().parse())
        .collect::<Result<Vec<usize>, _>>()?;
    ensure!(
        keys.len() == 2,
        "expected 2 public keys, found {}",
        keys.len()
    );
    Ok((keys[0], keys[1]))
}

/// Solves Year 2020, Day 25: Combo Breaker. There is no second part.
///
/// # Errors
/// Returns an error if the input cannot be read or parsed.
pub fn solve() -> Result<(Answer, Answer)> {
    let (card, door) = parse(&read_input(Year(2020), Day(25))?)?;
    Ok((u64::try_from(part_one(card, door))?.into(), 0.into()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_example;

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test1_part_one() -> Result<()> {
        let (card, door) = parse(&read_example(Year(2020), Day(25), 1)?)?;
        assert_eq!(part_one(card, door), 14897079);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_solve() -> Result<()> {
        let (p1, _) = solve()?;
        assert_eq!(p1, 12227206.into());
        Ok(())
    }
}
