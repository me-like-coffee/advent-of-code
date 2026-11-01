use crate::utils::{Answer, Day, Year, read_input};
use anyhow::{Result, bail};

fn corrupt(data: &str) -> Result<u32> {
    let mut stack = Vec::new();
    for c in data.bytes() {
        match c {
            b'(' | b'[' | b'{' | b'<' => {
                stack.push(c);
            }
            b')' => {
                if stack.last() == Some(&b'(') {
                    stack.pop();
                } else {
                    return Ok(3);
                }
            }
            b']' => {
                if stack.last() == Some(&b'[') {
                    stack.pop();
                } else {
                    return Ok(57);
                }
            }
            b'}' => {
                if stack.last() == Some(&b'{') {
                    stack.pop();
                } else {
                    return Ok(1197);
                }
            }
            b'>' => {
                if stack.last() == Some(&b'<') {
                    stack.pop();
                } else {
                    return Ok(25137);
                }
            }
            c => bail!("unexpected character {:?}", c as char),
        }
    }

    Ok(0)
}

pub fn part_one(data: &str) -> Result<u32> {
    data.lines().map(corrupt).sum()
}

fn incomplete(data: &str) -> Result<u64> {
    let mut stack = Vec::new();
    for c in data.bytes() {
        match c {
            b'(' | b'[' | b'{' | b'<' => {
                stack.push(c);
            }
            b')' => {
                if stack.last() == Some(&b'(') {
                    stack.pop();
                } else {
                    return Ok(0);
                }
            }
            b']' => {
                if stack.last() == Some(&b'[') {
                    stack.pop();
                } else {
                    return Ok(0);
                }
            }
            b'}' => {
                if stack.last() == Some(&b'{') {
                    stack.pop();
                } else {
                    return Ok(0);
                }
            }
            b'>' => {
                if stack.last() == Some(&b'<') {
                    stack.pop();
                } else {
                    return Ok(0);
                }
            }
            c => bail!("unexpected character {:?}", c as char),
        }
    }

    let mut score: u64 = 0;
    while let Some(c) = stack.pop() {
        match c {
            b'(' => score = score * 5 + 1,
            b'[' => score = score * 5 + 2,
            b'{' => score = score * 5 + 3,
            b'<' => score = score * 5 + 4,
            c => bail!("unexpected character {:?}", c as char),
        }
    }
    Ok(score)
}

pub fn part_two(data: &str) -> Result<u64> {
    let mut scores = data.lines().map(incomplete).collect::<Result<Vec<_>>>()?;
    scores.retain(|s| *s > 0);

    scores.sort();

    let index = scores.len() / 2;
    match scores.get(index) {
        Some(score) => Ok(*score),
        None => bail!("no incomplete lines"),
    }
}

/// Solves Year 2021, Day 10: Syntax Scoring.
///
/// # Errors
/// Returns an error if the input cannot be read or parsed.
pub fn solve() -> Result<(Answer, Answer)> {
    let data = read_input(Year(2021), Day(10))?;
    Ok((u64::from(part_one(&data)?).into(), part_two(&data)?.into()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_example;

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_part_one() -> Result<()> {
        let data = read_example(Year(2021), Day(10), 1)?;
        assert_eq!(part_one(&data)?, 26397);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_part_two() -> Result<()> {
        let data = read_example(Year(2021), Day(10), 1)?;
        assert_eq!(part_two(&data)?, 288957);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_solve() -> Result<()> {
        let (p1, p2) = solve()?;
        assert_eq!(p1, 265527.into());
        assert_eq!(p2, 3969823589.into());
        Ok(())
    }
}
