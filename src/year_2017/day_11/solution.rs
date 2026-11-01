use crate::utils::{Answer, Day, Year, read_input};
use anyhow::Result;

fn hex_distances(data: &str) -> (i32, i32) {
    let mut iter = data.bytes();
    let mut q: i32 = 0;
    let mut r: i32 = 0;
    let mut center_distance = 0;
    let mut max_distance = 0;

    while let Some(first) = iter.next() {
        match first {
            b'n' => match iter.next().unwrap_or(0) {
                b'e' => {
                    q += 1;
                    r -= 1;
                }
                b'w' => q -= 1,
                _ => r -= 1,
            },
            b's' => match iter.next().unwrap_or(0) {
                b'e' => q += 1,
                b'w' => {
                    q -= 1;
                    r += 1;
                }
                _ => r += 1,
            },
            _ => {}
        }

        let s = q + r;
        center_distance = (q.abs() + r.abs() + s.abs()) / 2;
        max_distance = max_distance.max(center_distance);
    }

    (center_distance, max_distance)
}

/// Solves Year 2017, Day 11: Hex Ed.
///
/// # Errors
/// Returns an error if the input cannot be read.
pub fn solve() -> Result<(Answer, Answer)> {
    let data = read_input(Year(2017), Day(11))?;
    let (part_one, part_two) = hex_distances(&data);
    Ok((
        u64::try_from(part_one)?.into(),
        u64::try_from(part_two)?.into(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_solve() -> Result<()> {
        let (p1, p2) = solve()?;
        assert_eq!(p1, 675.into());
        assert_eq!(p2, 1424.into());
        Ok(())
    }
}
