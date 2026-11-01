use crate::utils::{Answer, Day, Year, read_input};
use anyhow::{Context, Result, bail};
use std::collections::{HashMap, VecDeque};

type IntType = i64;

#[derive(Clone, Debug)]
enum Instruction {
    Add { x: String, y: String },
    Jgz { x: String, y: String },
    Mod { x: String, y: String },
    Mul { x: String, y: String },
    Rcv { x: u8 },
    Set { x: String, y: String },
    Snd { x: String },
}

#[derive(Debug, PartialEq)]
enum ProgramState {
    Active,
    Ready,
    Terminated,
    Waiting,
}

#[derive(Debug)]
struct Program {
    program: Vec<Instruction>,
    program_counter: IntType,
    registers: HashMap<u8, IntType>,
    send_buffer: VecDeque<IntType>,
    receive_buffer: VecDeque<IntType>,
    number_of_sends: usize,
    state: ProgramState,
}

impl Program {
    fn new(program: &[Instruction], registers: &HashMap<u8, IntType>) -> Program {
        Program {
            program: program.to_vec(),
            program_counter: 0,
            registers: registers.clone(),
            send_buffer: VecDeque::new(),
            receive_buffer: VecDeque::new(),
            number_of_sends: 0,
            state: ProgramState::Ready,
        }
    }

    fn init(&mut self, registers: &HashMap<u8, IntType>) {
        for (k, v) in registers {
            self.registers.insert(*k, *v);
        }
    }

    // Refactor while loop to single pass per step
    fn step(&mut self) {
        while self.program_counter >= 0 && self.program_counter < self.program.len() as IntType {
            let instruction = &self.program[self.program_counter as usize];
            let program_counter_increment;

            match self.state {
                ProgramState::Ready | ProgramState::Active | ProgramState::Waiting => {
                    self.state = ProgramState::Active;
                    match instruction {
                        Instruction::Add { x, y } => {
                            let x_value = value_or_register(x, &self.registers);
                            let y_value = value_or_register(y, &self.registers);
                            let key = get_register(x);
                            self.registers.insert(key, x_value + y_value);
                            program_counter_increment = 1;
                        }
                        Instruction::Jgz { x, y } => {
                            let x_value = value_or_register(x, &self.registers);
                            let y_value = value_or_register(y, &self.registers);
                            if x_value > 0 {
                                program_counter_increment = y_value;
                            } else {
                                program_counter_increment = 1;
                            }
                        }
                        Instruction::Mod { x, y } => {
                            let x_value = value_or_register(x, &self.registers);
                            let y_value = value_or_register(y, &self.registers);
                            let key = get_register(x);
                            self.registers.insert(key, x_value % y_value);
                            program_counter_increment = 1;
                        }
                        Instruction::Mul { x, y } => {
                            let x_value = value_or_register(x, &self.registers);
                            let y_value = value_or_register(y, &self.registers);
                            let key = get_register(x);
                            self.registers.insert(key, x_value * y_value);
                            program_counter_increment = 1;
                        }
                        Instruction::Rcv { x } => {
                            if let Some(value) = self.receive_buffer.pop_front() {
                                self.registers.insert(*x, value);
                                program_counter_increment = 1;
                            } else {
                                self.state = ProgramState::Waiting;
                                return;
                            }
                        }
                        Instruction::Set { x, y } => {
                            let y_value = value_or_register(y, &self.registers);
                            let key = get_register(x);
                            self.registers.insert(key, y_value);
                            program_counter_increment = 1;
                        }
                        Instruction::Snd { x } => {
                            let value = value_or_register(x, &self.registers);
                            self.send_buffer.push_back(value);
                            self.number_of_sends += 1;
                            program_counter_increment = 1;
                        }
                    }
                }
                ProgramState::Terminated => {
                    return;
                }
            }

            self.program_counter += program_counter_increment;
        }
        self.state = ProgramState::Terminated;
    }

    fn send_messages(&mut self) -> Vec<IntType> {
        self.send_buffer.drain(..).collect()
    }

    fn receive_messages(&mut self, messages: Vec<IntType>) {
        self.receive_buffer.extend(messages);
    }
}

/// Operands are non-empty: they come from whitespace-separated tokens.
fn get_register(string: &str) -> u8 {
    string.as_bytes()[0]
}

/// All registers start at 0, so a register that was never written reads as 0.
fn value_or_register(string: &str, registers: &HashMap<u8, IntType>) -> IntType {
    if let Ok(value) = string.parse::<IntType>() {
        value
    } else {
        registers.get(&get_register(string)).copied().unwrap_or(0)
    }
}

// mul p -1
// add p b
fn parse_instruction(data: &str) -> Result<Instruction> {
    let tokens: Vec<_> = data.split_ascii_whitespace().collect();

    Ok(match tokens.as_slice() {
        ["snd", x] => Instruction::Snd { x: x.to_string() },
        ["rcv", x] => Instruction::Rcv { x: get_register(x) },
        ["set", x, y] => Instruction::Set {
            x: x.to_string(),
            y: y.to_string(),
        },
        ["add", x, y] => Instruction::Add {
            x: x.to_string(),
            y: y.to_string(),
        },
        ["mul", x, y] => Instruction::Mul {
            x: x.to_string(),
            y: y.to_string(),
        },
        ["mod", x, y] => Instruction::Mod {
            x: x.to_string(),
            y: y.to_string(),
        },
        ["jgz", x, y] => Instruction::Jgz {
            x: x.to_string(),
            y: y.to_string(),
        },
        _ => bail!("unknown instruction: {data:?}"),
    })
}

fn parse_program(data: &str) -> Result<Vec<Instruction>> {
    data.lines().map(parse_instruction).collect()
}

fn part_one(data: &str) -> Result<IntType> {
    let program = parse_program(data)?;
    let registers: HashMap<_, _> = (b'a'..=b'z').map(|c| (c, 0)).collect();

    let mut program_zero = Program::new(&program, &registers);
    program_zero.step();

    program_zero
        .registers
        .get(&b'b')
        .copied()
        .context("no register b")
}

fn part_two(data: &str) -> Result<usize> {
    let program = parse_program(data)?;
    let registers: HashMap<_, _> = (b'a'..=b'z').map(|c| (c, 0)).collect();

    let mut program_zero = Program::new(&program, &registers);
    let mut program_one = Program::new(&program, &registers);
    program_one.init(&[(b'p', 1)].into_iter().collect());

    loop {
        program_one.receive_messages(program_zero.send_messages());
        program_zero.receive_messages(program_one.send_messages());

        program_zero.step();
        program_one.step();

        if program_zero.state == ProgramState::Waiting
            && program_one.send_buffer.is_empty()
            && program_one.state == ProgramState::Waiting
            && program_zero.send_buffer.is_empty()
        {
            break;
        }

        if program_zero.state == ProgramState::Terminated
            && program_one.state == ProgramState::Terminated
        {
            break;
        }
    }

    Ok(program_one.number_of_sends)
}

/// Solves Year 2017, Day 18: Duet.
///
/// # Errors
/// Returns an error if the input cannot be read or parsed.
pub fn solve() -> Result<(Answer, Answer)> {
    let data = read_input(Year(2017), Day(18))?;
    Ok((
        u64::try_from(part_one(&data)?)?.into(),
        u64::try_from(part_two(&data)?)?.into(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_example;

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_part_one() -> Result<()> {
        assert_eq!(part_one(&read_example(Year(2017), Day(18), 1)?)?, 0);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_part_two() -> Result<()> {
        assert_eq!(part_two(&read_example(Year(2017), Day(18), 2)?)?, 3);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_solve() -> Result<()> {
        let (p1, p2) = solve()?;
        assert_eq!(p1, 3188.into());
        assert_eq!(p2, 7112.into());
        Ok(())
    }
}
