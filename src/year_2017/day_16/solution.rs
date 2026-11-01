use crate::utils::{Answer, Day, Year, read_input};
use anyhow::{Context, Result, bail};

#[derive(Debug)]
enum DanceMove {
    Exchange { a: usize, b: usize },
    Partner { a: u8, b: u8 },
    Spin { x: usize },
}

fn parse(data: &str) -> Result<DanceMove> {
    let data = data.as_bytes();
    match data.first() {
        Some(b'x') => {
            let (mut a, mut b) = (0, 0);
            let mut first_program = true;
            for &c in &data[1..] {
                if c.is_ascii_digit() {
                    if first_program {
                        a = a * 10 + (c - b'0') as usize;
                    } else {
                        b = b * 10 + (c - b'0') as usize;
                    }
                } else {
                    first_program = false;
                }
            }
            Ok(DanceMove::Exchange { a, b })
        }
        Some(b'p') => {
            let (mut a, mut b) = (b'#', b'#');
            let mut first_program = true;
            for &c in &data[1..] {
                if c.is_ascii_lowercase() {
                    if first_program {
                        a = c;
                    } else {
                        b = c;
                    }
                } else {
                    first_program = false;
                }
            }
            Ok(DanceMove::Partner { a, b })
        }
        Some(b's') => {
            let mut x = 0;
            for &c in &data[1..] {
                x = x * 10 + (c - b'0') as usize;
            }
            Ok(DanceMove::Spin { x })
        }
        _ => bail!("unknown dance move: {:?}", String::from_utf8_lossy(data)),
    }
}

fn dance(programs: &mut [u8], dance_moves: &[DanceMove]) -> Result<()> {
    for dance_move in dance_moves {
        match dance_move {
            DanceMove::Exchange { a, b } => {
                programs.swap(*a, *b);
            }
            DanceMove::Partner { a, b } => {
                let a_index = programs
                    .iter()
                    .position(|c| c == a)
                    .with_context(|| format!("no program {}", *a as char))?;
                let b_index = programs
                    .iter()
                    .position(|c| c == b)
                    .with_context(|| format!("no program {}", *b as char))?;
                programs.swap(a_index, b_index);
            }
            DanceMove::Spin { x } => {
                programs.rotate_right(*x);
            }
        }
    }
    Ok(())
}

fn part_one(data: &str, size: u8) -> Result<String> {
    let dance_moves = data.split(',').map(parse).collect::<Result<Vec<_>>>()?;
    let mut programs: Vec<u8> = (0..size).map(|i| b'a' + i).collect();

    dance(&mut programs, &dance_moves)?;

    Ok(programs.iter().map(|c| *c as char).collect::<String>())
}

/// The dance repeats every 48 rounds for this input, so a billion rounds
/// equal 1_000_000_000 % 48 == 16 rounds.
fn part_two(data: &str, size: u8) -> Result<String> {
    let dance_moves = data.split(',').map(parse).collect::<Result<Vec<_>>>()?;
    let mut programs: Vec<u8> = (0..size).map(|i| b'a' + i).collect();

    for _ in 0..16 {
        dance(&mut programs, &dance_moves)?;
    }
    Ok(programs.iter().map(|c| *c as char).collect::<String>())
}

/// Solves Year 2017, Day 16: Permutation Promenade.
///
/// # Errors
/// Returns an error if the input cannot be read or parsed.
pub fn solve() -> Result<(Answer, Answer)> {
    let data = read_input(Year(2017), Day(16))?;
    Ok((part_one(&data, 16)?.into(), part_two(&data, 16)?.into()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_example;

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_part_one() -> Result<()> {
        assert_eq!(
            part_one(&read_example(Year(2017), Day(16), 1)?, 5)?,
            "baedc"
        );
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_solve() -> Result<()> {
        let (p1, p2) = solve()?;
        assert_eq!(p1, "eojfmbpkldghncia".into());
        assert_eq!(p2, "iecopnahgdflmkjb".into());
        Ok(())
    }
}
