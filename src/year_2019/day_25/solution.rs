use crate::utils::{Answer, Day, Year, read_extra, read_input};
use crate::year_2019::intcode::{Intcode, Status, parse_program};
use anyhow::Result;

/// Solves Year 2019, Day 25: Cryostasis.
///
/// The route through the ship depends on the input, so it lives in the local
/// file `day_25_walkthrough.txt` (one command per line), next to the input.
/// The items it picks up with `take` are the ones tried at the checkpoint.
///
/// # Errors
/// Returns an error if the input cannot be read or program execution fails.
pub fn solve() -> Result<(Answer, Answer)> {
    let input = read_input(Year(2019), Day(25))?;
    let program = parse_program(&input)?;

    let walkthrough = read_extra(Year(2019), Day(25), "walkthrough")?;
    let password = solve_part1(&program, &walkthrough)?;

    Ok((password.into(), 0.into()))
}

fn solve_part1(program: &[i64], walkthrough: &str) -> Result<u64> {
    let mut vm = Intcode::new(program.to_vec());

    // Phase 1: Walkthrough to collect the safe items and reach the Security Checkpoint.
    // The sequence includes backtracking steps to ensure the VM state stays synchronized.
    let walkthrough: Vec<&str> = walkthrough.lines().filter(|l| !l.is_empty()).collect();

    for cmd in &walkthrough {
        run_command(&mut vm, cmd)?;
    }

    let safe_items: Vec<&str> = walkthrough
        .iter()
        .filter_map(|cmd| cmd.strip_prefix("take "))
        .collect();

    // Phase 2: Brute force the correct weight combination using Differential Inventory Management.
    // We start with all items (mask 255) and XOR the current mask with the next to minimize commands.
    let mut current_mask: u32 = 255;
    let n = safe_items.len();

    for next_mask in 0u32..(1 << n) {
        for (i, item) in safe_items.iter().enumerate() {
            let was_held = (current_mask >> i) & 1 == 1;
            let should_hold = (next_mask >> i) & 1 == 1;

            if should_hold && !was_held {
                run_command(&mut vm, &format!("take {}", item))?;
            } else if !should_hold && was_held {
                run_command(&mut vm, &format!("drop {}", item))?;
            }
        }
        current_mask = next_mask;

        // Moving 'east' from the Security Checkpoint tests the current weight.
        let output = run_command(&mut vm, "east")?;
        if !output.contains("Alert!") {
            // The success message says "…get in by typing <password> on the keypad…".
            if let Some(pos) = output.find("typing ") {
                let start = pos + "typing ".len();
                let end = output[start..]
                    .find(|c: char| !c.is_ascii_digit())
                    .map(|i| start + i)
                    .unwrap_or(output.len());
                return Ok(output[start..end].parse()?);
            }
        }
    }

    Err(anyhow::anyhow!("Could not find airlock password"))
}

/// Helper to execute a command and return the VM's ASCII output.
fn run_command(vm: &mut Intcode, cmd: &str) -> Result<String> {
    for c in cmd.chars() {
        vm.add_input(c as i64);
    }
    vm.add_input(10); // Newline (ASCII 10)

    let mut buffer = String::new();
    while let Status::Output(val) = vm.step()? {
        if val < 256 {
            buffer.push(val as u8 as char);
        }
    }
    Ok(buffer)
}
