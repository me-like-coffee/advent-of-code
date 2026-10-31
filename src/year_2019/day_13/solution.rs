use crate::utils::{Answer, read_input, Day, Year};
use crate::year_2019::intcode::{Intcode, Status};
use anyhow::{Context, Result};

/// Solves Year 2019, Day 13: Care Package.
///
/// # Errors
/// Returns an error if the input cannot be read or program execution fails.
pub fn solve() -> Result<(Answer, Answer)> {
    let input = read_input(Year(2019), Day(13))?;
    let program: Vec<i64> = input
        .trim()
        .split(',')
        .map(|s| {
            s.parse::<i64>()
                .with_context(|| format!("Failed to parse program value: '{}'", s))
        })
        .collect::<Result<Vec<i64>>>()?;

    // Part 1: Count block tiles (id 2)
    let part1 = solve_part1(&program)?;

    // Part 2: Play the game and get the final score
    let part2 = solve_part2(&program)?;

    Ok((part1.into(), part2.into()))
}

fn solve_part1(program: &[i64]) -> Result<u64> {
    let mut vm = Intcode::new(program.to_vec());
    let outputs = vm.run_to_end()?;
    let mut block_count = 0;

    for chunk in outputs.as_chunks::<3>().0 {
        if chunk[2] == 2 {
            block_count += 1;
        }
    }

    Ok(block_count)
}

fn solve_part2(program: &[i64]) -> Result<u64> {
    let mut mem = program.to_vec();
    // Set memory address 0 to 2 to play for free
    if !mem.is_empty() {
        mem[0] = 2;
    }
    let mut vm = Intcode::new(mem);

    let mut score = 0;
    let mut paddle_x = 0;
    let mut ball_x = 0;
    let mut output_buffer = Vec::new();

    loop {
        match vm.step()? {
            Status::Halted => break,
            Status::Output(val) => {
                output_buffer.push(val);
                if output_buffer.len() == 3 {
                    let x = output_buffer[0];
                    let y = output_buffer[1];
                    let tile_id = output_buffer[2];

                    if x == -1 && y == 0 {
                        score = tile_id;
                    } else {
                        match tile_id {
                            3 => paddle_x = x,
                            4 => ball_x = x,
                            _ => {}
                        }
                    }
                    output_buffer.clear();
                }
            }
            Status::NeedsInput => {
                // Simple AI: move paddle towards ball
                let joystick = (ball_x - paddle_x).signum();
                vm.add_input(joystick);
            }
        }
    }

    Ok(score as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_day_13_part1() -> Result<()> {
        let (p1, _) = solve()?;
        assert_eq!(p1, 270.into());
        Ok(())
    }
}
