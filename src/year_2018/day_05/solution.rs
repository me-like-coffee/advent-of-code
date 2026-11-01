use crate::utils::{Answer, Day, Year, read_input};
use anyhow::Result;

fn opposite_polarity(a: u8, b: u8) -> bool {
    match (a.is_ascii_uppercase(), b.is_ascii_uppercase()) {
        (true, true) | (false, false) => false,
        (true, false) => a - b'A' == b.wrapping_sub(b'a'),
        (false, true) => a.wrapping_sub(b'a') == b - b'A',
    }
}

fn react_polymer(polymer: &[u8]) -> usize {
    let mut polymer = polymer.to_vec();
    loop {
        let polymer_len = polymer.len();
        for i in 0..(polymer.len() - 1) {
            if opposite_polarity(polymer[i], polymer[i + 1]) {
                polymer.remove(i + 1);
                polymer.remove(i);
                break;
            }
        }
        if polymer_len == polymer.len() {
            break;
        }
    }

    polymer.len()
}

fn part_one(data: &str) -> usize {
    react_polymer(data.as_bytes())
}

fn filter_polymer(polymer: &[u8], lower_case: u8) -> Vec<u8> {
    let upper_case = lower_case - b'a' + b'A';
    polymer
        .iter()
        .filter(|&&b| b != lower_case && b != upper_case)
        .copied()
        .collect()
}

fn part_two(data: &str) -> usize {
    let polymer = data.as_bytes();
    let mut min_polymer_len = usize::MAX;

    for c in b'a'..=b'z' {
        let filtered_polymer = filter_polymer(polymer, c);
        min_polymer_len = min_polymer_len.min(react_polymer(&filtered_polymer));
    }

    min_polymer_len
}

/// Solves Year 2018, Day 5: Alchemical Reduction.
///
/// # Errors
/// Returns an error if the input cannot be read.
pub fn solve() -> Result<(Answer, Answer)> {
    let data = read_input(Year(2018), Day(5))?;
    Ok((
        u64::try_from(part_one(&data))?.into(),
        u64::try_from(part_two(&data))?.into(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_example;

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_filter_polymer() -> Result<()> {
        let data = read_example(Year(2018), Day(5), 1)?;
        let polymer = data.as_bytes();

        assert_eq!(b"dbcCCBcCcD".to_vec(), filter_polymer(polymer, b'a'));
        assert_eq!(b"daAcCaCAcCcaDA".to_vec(), filter_polymer(polymer, b'b'));
        assert_eq!(b"dabAaBAaDA".to_vec(), filter_polymer(polymer, b'c'));
        assert_eq!(b"abAcCaCBAcCcaA".to_vec(), filter_polymer(polymer, b'd'));
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_part_one() -> Result<()> {
        assert_eq!(part_one(&read_example(Year(2018), Day(5), 1)?), 10);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_part_two() -> Result<()> {
        assert_eq!(part_two(&read_example(Year(2018), Day(5), 1)?), 4);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_solve() -> Result<()> {
        let (p1, p2) = solve()?;
        assert_eq!(p1, 11364.into());
        assert_eq!(p2, 4212.into());
        Ok(())
    }
}
