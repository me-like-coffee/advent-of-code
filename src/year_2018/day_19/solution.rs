use crate::utils::{Answer, Day, Year, read_input};
use crate::year_2018::device::Opcode;
use anyhow::{Context, Result, ensure};

type Operands = [u64; 3];
type Registers = [u64; 6];
type Instruction = (Opcode, Operands);

// eqrr 1 5 1
fn parse_line(data: &str) -> Result<Instruction> {
    let tokens: Vec<_> = data.split(' ').collect();
    ensure!(tokens.len() == 4, "not an instruction: {data:?}");
    let op = Opcode::from_name(tokens[0]).with_context(|| format!("unknown opcode in {data:?}"))?;
    let mut operands: Operands = [0; 3];
    operands[0] = tokens[1].parse()?;
    operands[1] = tokens[2].parse()?;
    operands[2] = tokens[3].parse()?;

    Ok((op, operands))
}

fn parse(data: &str) -> Result<(usize, Vec<Instruction>)> {
    // ip_register
    let mut data_lines = data.lines();
    let ip_line = data_lines.next().context("empty program")?;
    let (_, id) = ip_line
        .split_once(' ')
        .with_context(|| format!("not an #ip line: {ip_line:?}"))?;
    let ip_register = id.trim().parse()?;

    // build instruction vector
    let instructions = data_lines.map(parse_line).collect::<Result<Vec<_>>>()?;

    Ok((ip_register, instructions))
}

fn evaluate(
    ip_register: usize,
    instructions: &[Instruction],
    init_registers: &Registers,
) -> Result<Registers> {
    let mut registers: Registers = *init_registers;
    let mut ip: usize = 0;

    while ip < instructions.len() {
        registers[ip_register] = ip as u64;
        let (op, [a, b, c]) = instructions[ip];
        op.execute(a, b, c, &mut registers)?;
        ip = usize::try_from(registers[ip_register])?;
        ip += 1;
    }

    Ok(registers)
}

pub fn part_one(data: &str) -> Result<u64> {
    let (ip_register, instructions) = parse(data)?;
    ensure!(ip_register < 6, "no register {ip_register}");
    let registers = evaluate(ip_register, &instructions, &[0; 6])?;

    Ok(registers[0])
}

/// Solves Year 2018, Day 19: Go With The Flow.
///
/// Part two is not solved: running the program with register 0 set to 1, as the
/// original did, takes far too long, so it returns 0.
///
/// # Errors
/// Returns an error if the input cannot be read or parsed.
pub fn solve() -> Result<(Answer, Answer)> {
    let data = read_input(Year(2018), Day(19))?;
    Ok((part_one(&data)?.into(), 0.into()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_example;

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_part_one() -> Result<()> {
        assert_eq!(part_one(&read_example(Year(2018), Day(19), 1)?)?, 6);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_solve() -> Result<()> {
        let (p1, _) = solve()?;
        assert_eq!(p1, 1256.into());
        Ok(())
    }
}
