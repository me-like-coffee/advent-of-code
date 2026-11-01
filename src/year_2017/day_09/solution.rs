use crate::utils::{Answer, Day, Year, read_input};
use anyhow::Result;

fn stream_scores(data: &str) -> (usize, usize) {
    let mut iter = data.bytes();
    let mut groups = 0;
    let mut depth = 1;
    let mut characters = 0;

    while let Some(b) = iter.next() {
        match b {
            b'<' => {
                while let Some(b) = iter.next() {
                    match b {
                        b'!' => {
                            iter.next();
                        }
                        b'>' => {
                            break;
                        }
                        _ => {
                            characters += 1;
                        }
                    }
                }
            }
            b'{' => {
                groups += depth;
                depth += 1;
            }
            b'}' => {
                depth -= 1;
            }
            _ => {}
        }
    }

    (groups, characters)
}

/// Solves Year 2017, Day 9: Stream Processing.
///
/// # Errors
/// Returns an error if the input cannot be read.
pub fn solve() -> Result<(Answer, Answer)> {
    let data = read_input(Year(2017), Day(9))?;
    let (groups, characters) = stream_scores(&data);
    Ok((
        u64::try_from(groups)?.into(),
        u64::try_from(characters)?.into(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_solve() -> Result<()> {
        let (p1, p2) = solve()?;
        assert_eq!(p1, 10820.into());
        assert_eq!(p2, 5547.into());
        Ok(())
    }
}
