use crate::utils::{Answer, Day, Year, read_input, sections};
use anyhow::{Context, Result, ensure};
use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Action {
    write_value: i32,
    move_value: i32,
    next_state: u8,
}

impl Action {
    pub fn new(write_value: i32, move_value: i32, next_state: u8) -> Action {
        Action {
            write_value,
            move_value,
            next_state,
        }
    }

    /// Parses the three lines "Write the value N.", "Move one slot to the …" and
    /// "Continue with state X.".
    fn parse(write: &str, direction: &str, next: &str) -> Result<Action> {
        let write_value = i32::from(second_last_byte(write)? - b'0');
        let move_value = if direction.contains("right") { 1 } else { -1 };
        let next_state = second_last_byte(next)?;
        Ok(Action::new(write_value, move_value, next_state))
    }
}

/// The byte before the final `.` or `:` of a line.
fn second_last_byte(line: &str) -> Result<u8> {
    line.bytes()
        .rev()
        .nth(1)
        .with_context(|| format!("line too short: {line:?}"))
}

#[derive(Debug)]
struct State {
    actions: [Action; 2],
}

impl State {
    fn new(zero_action: &Action, one_action: &Action) -> State {
        State {
            actions: [zero_action.clone(), one_action.clone()],
        }
    }
}

#[derive(Debug)]
struct TuringMachine {
    tape: HashMap<i32, i32>,
    cursor: i32,
    states: HashMap<u8, State>,
    curr_state: u8,
    checksum_steps: usize,
}

impl TuringMachine {
    pub fn parse(data: &str) -> Result<TuringMachine> {
        let tape = HashMap::new();
        let cursor = 0;
        let mut states = HashMap::new();

        let tokens: Vec<_> = sections(data).collect();

        let (initial_state_token, checksum_token) =
            tokens[0].split_once('.').context("missing initial state")?;
        let initial_state_tokens: Vec<_> = initial_state_token.split_ascii_whitespace().collect();
        let curr_state = *initial_state_tokens
            .get(3)
            .and_then(|t| t.trim().as_bytes().first())
            .context("missing initial state name")?;
        let checksum_tokens: Vec<_> = checksum_token.split_ascii_whitespace().collect();
        let checksum_steps = checksum_tokens
            .get(5)
            .context("missing checksum step count")?
            .parse()?;

        for token in tokens.iter().skip(1) {
            let state_tokens: Vec<_> = token.split('\n').collect();
            ensure!(state_tokens.len() >= 9, "incomplete state: {token:?}");
            let state_id = second_last_byte(state_tokens[0])?;

            let zero_action = Action::parse(state_tokens[2], state_tokens[3], state_tokens[4])?;
            let one_action = Action::parse(state_tokens[6], state_tokens[7], state_tokens[8])?;

            let state = State::new(&zero_action, &one_action);
            states.insert(state_id, state);
        }

        Ok(TuringMachine {
            tape,
            cursor,
            states,
            curr_state,
            checksum_steps,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        for _ in 0..self.checksum_steps {
            let state = self
                .states
                .get(&self.curr_state)
                .with_context(|| format!("no state {}", self.curr_state as char))?;
            let value = *self.tape.get(&self.cursor).unwrap_or(&0);
            let action = &state.actions[value as usize];
            self.tape.insert(self.cursor, action.write_value);
            self.cursor += action.move_value;
            self.curr_state = action.next_state;
        }
        Ok(())
    }

    pub fn checksum(&self) -> usize {
        self.tape.iter().filter(|(_, v)| **v == 1).count()
    }
}

pub fn part_one(data: &str) -> Result<usize> {
    let mut turing_machine = TuringMachine::parse(data)?;

    turing_machine.run()?;

    Ok(turing_machine.checksum())
}

/// Solves Year 2017, Day 25: The Halting Problem. There is no second part.
///
/// # Errors
/// Returns an error if the input cannot be read or parsed.
pub fn solve() -> Result<(Answer, Answer)> {
    let data = read_input(Year(2017), Day(25))?;
    Ok((u64::try_from(part_one(&data)?)?.into(), 0.into()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_example;

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_part_one() -> Result<()> {
        assert_eq!(part_one(&read_example(Year(2017), Day(25), 1)?)?, 3);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_solve() -> Result<()> {
        let (p1, _) = solve()?;
        assert_eq!(p1, 2526.into());
        Ok(())
    }
}
