use crate::utils::{Answer, Day, Year, read_input};
use anyhow::Result;

fn parse_marker(bytes: &[u8], start: usize) -> (u64, u64, usize) {
    let (mut length, mut multiple) = (0, 0);
    let mut first_number = true;
    let mut i = start;

    loop {
        let c = bytes[i];
        match c {
            b'0'..=b'9' => {
                if first_number {
                    length = length * 10 + (c - b'0') as u64;
                } else {
                    multiple = multiple * 10 + (c - b'0') as u64;
                }
            }
            b'x' => {
                first_number = false;
            }
            b')' => {
                break;
            }
            _ => {}
        }

        i += 1;
    }

    (length, multiple, i + 1)
}

fn decompress_one(bytes: &[u8]) -> u64 {
    let mut i = 0;
    let mut total_length = 0;
    while i < bytes.len() {
        if bytes[i].is_ascii_uppercase() {
            i += 1;
            total_length += 1;
        } else {
            let (length, multiple, new_i) = parse_marker(bytes, i);
            i = new_i + length as usize;
            total_length += length * multiple;
        }
    }

    total_length
}

fn decompress_two(bytes: &[u8], start: usize, end: usize) -> u64 {
    let mut i = start;
    let mut total_length = 0;
    while i < end {
        if bytes[i].is_ascii_uppercase() {
            i += 1;
            total_length += 1;
        } else {
            let (length, multiple, new_i) = parse_marker(bytes, i);
            i = new_i + length as usize;
            total_length += multiple * decompress_two(bytes, new_i, new_i + length as usize);
        }
    }

    total_length
}

fn part_one(data: &str) -> u64 {
    data.lines().map(|l| decompress_one(l.as_bytes())).sum()
}

fn part_two(data: &str) -> u64 {
    data.lines()
        .map(|l| decompress_two(l.as_bytes(), 0, l.len() - 1))
        .sum()
}

/// Solves Year 2016, Day 9: Explosives in Cyberspace.
///
/// # Errors
/// Returns an error if the input cannot be read.
pub fn solve() -> Result<(Answer, Answer)> {
    let data = read_input(Year(2016), Day(9))?;
    Ok((part_one(&data).into(), part_two(&data).into()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_example;

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_part_one() -> Result<()> {
        assert_eq!(part_one(&read_example(Year(2016), Day(9), 1)?), 57);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_part_two() -> Result<()> {
        assert_eq!(part_two(&read_example(Year(2016), Day(9), 2)?), 242393);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_solve() -> Result<()> {
        let (p1, p2) = solve()?;
        assert_eq!(p1, 110346.into());
        assert_eq!(p2, 10774309173.into());
        Ok(())
    }
}
