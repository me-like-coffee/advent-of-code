use crate::utils::{Answer, Day, Year, read_input};
use anyhow::{Context, Result};

fn part_one(step_size: usize) -> u32 {
    let mut buffer = vec![0u32];
    let mut current_position = 0;
    let mut insert_position = current_position;

    for element in 1..=2017u32 {
        current_position = (current_position + step_size) % buffer.len();
        insert_position = current_position + 1;
        if insert_position == buffer.len() {
            buffer.push(element);
        } else {
            buffer.insert(insert_position, element);
        }
        current_position = insert_position;
    }
    buffer[insert_position + 1]
}

fn part_two(step_size: usize) -> u32 {
    let mut current_position = 0;
    let mut value_after_zero = 0;
    for element in 1..=50000000 {
        current_position = (current_position + step_size) % element;
        if current_position == 0 {
            value_after_zero = element as u32;
        }
        current_position = (current_position + 1) % (element + 1);
    }
    value_after_zero
}

fn parse(data: &str) -> Result<usize> {
    data.trim()
        .parse()
        .with_context(|| format!("not a step size: {data:?}"))
}

/// Solves Year 2017, Day 17: Spinlock.
///
/// # Errors
/// Returns an error if the input cannot be read or isn't a number.
pub fn solve() -> Result<(Answer, Answer)> {
    let step_size = parse(&read_input(Year(2017), Day(17))?)?;
    Ok((
        u64::from(part_one(step_size)).into(),
        u64::from(part_two(step_size)).into(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_example;

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_part_one() -> Result<()> {
        let step_size = parse(&read_example(Year(2017), Day(17), 1)?)?;
        assert_eq!(part_one(step_size), 638);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_solve() -> Result<()> {
        let (p1, p2) = solve()?;
        assert_eq!(p1, 1642.into());
        assert_eq!(p2, 33601318.into());
        Ok(())
    }
}
