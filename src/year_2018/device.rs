//! The 2018 time-travel device's sixteen opcodes, shared by days 16 and 19.

use anyhow::{Context, Result};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Opcode {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

impl Opcode {
    pub const ALL: [Opcode; 16] = [
        Opcode::Addr,
        Opcode::Addi,
        Opcode::Mulr,
        Opcode::Muli,
        Opcode::Banr,
        Opcode::Bani,
        Opcode::Borr,
        Opcode::Bori,
        Opcode::Setr,
        Opcode::Seti,
        Opcode::Gtir,
        Opcode::Gtri,
        Opcode::Gtrr,
        Opcode::Eqir,
        Opcode::Eqri,
        Opcode::Eqrr,
    ];

    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Opcode::Addr => "addr",
            Opcode::Addi => "addi",
            Opcode::Mulr => "mulr",
            Opcode::Muli => "muli",
            Opcode::Banr => "banr",
            Opcode::Bani => "bani",
            Opcode::Borr => "borr",
            Opcode::Bori => "bori",
            Opcode::Setr => "setr",
            Opcode::Seti => "seti",
            Opcode::Gtir => "gtir",
            Opcode::Gtri => "gtri",
            Opcode::Gtrr => "gtrr",
            Opcode::Eqir => "eqir",
            Opcode::Eqri => "eqri",
            Opcode::Eqrr => "eqrr",
        }
    }

    #[must_use]
    pub fn from_name(name: &str) -> Option<Opcode> {
        Opcode::ALL.into_iter().find(|op| op.name() == name)
    }

    /// Runs the opcode with operands `a`, `b` and `c`, writing register `c`.
    ///
    /// # Errors
    /// Returns an error if an operand used as a register is not one of `registers`.
    pub fn execute(self, a: u64, b: u64, c: u64, registers: &mut [u64]) -> Result<()> {
        let reg = |i: u64| -> Result<u64> {
            usize::try_from(i)
                .ok()
                .and_then(|i| registers.get(i).copied())
                .with_context(|| format!("{}: no register {i}", self.name()))
        };
        let value = match self {
            Opcode::Addr => reg(a)? + reg(b)?,
            Opcode::Addi => reg(a)? + b,
            Opcode::Mulr => reg(a)? * reg(b)?,
            Opcode::Muli => reg(a)? * b,
            Opcode::Banr => reg(a)? & reg(b)?,
            Opcode::Bani => reg(a)? & b,
            Opcode::Borr => reg(a)? | reg(b)?,
            Opcode::Bori => reg(a)? | b,
            Opcode::Setr => reg(a)?,
            Opcode::Seti => a,
            Opcode::Gtir => u64::from(a > reg(b)?),
            Opcode::Gtri => u64::from(reg(a)? > b),
            Opcode::Gtrr => u64::from(reg(a)? > reg(b)?),
            Opcode::Eqir => u64::from(a == reg(b)?),
            Opcode::Eqri => u64::from(reg(a)? == b),
            Opcode::Eqrr => u64::from(reg(a)? == reg(b)?),
        };
        let target = usize::try_from(c)
            .ok()
            .and_then(|c| registers.get_mut(c))
            .with_context(|| format!("{}: no register {c}", self.name()))?;
        *target = value;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(op: Opcode, a: u64, b: u64, c: u64) -> Result<[u64; 4]> {
        let mut registers = [5, 3, 12, 0];
        op.execute(a, b, c, &mut registers)?;
        Ok(registers)
    }

    #[test]
    fn test_arithmetic_and_bitwise() -> Result<()> {
        assert_eq!(run(Opcode::Addr, 0, 1, 3)?, [5, 3, 12, 8]);
        assert_eq!(run(Opcode::Addi, 0, 7, 3)?, [5, 3, 12, 12]);
        assert_eq!(run(Opcode::Mulr, 0, 1, 3)?, [5, 3, 12, 15]);
        assert_eq!(run(Opcode::Muli, 1, 4, 3)?, [5, 3, 12, 12]);
        assert_eq!(run(Opcode::Banr, 0, 2, 3)?, [5, 3, 12, 4]);
        assert_eq!(run(Opcode::Bani, 2, 6, 3)?, [5, 3, 12, 4]);
        assert_eq!(run(Opcode::Borr, 0, 1, 3)?, [5, 3, 12, 7]);
        assert_eq!(run(Opcode::Bori, 0, 2, 3)?, [5, 3, 12, 7]);
        Ok(())
    }

    #[test]
    fn test_assignment() -> Result<()> {
        assert_eq!(run(Opcode::Setr, 2, 9, 0)?, [12, 3, 12, 0]);
        assert_eq!(run(Opcode::Seti, 9, 2, 0)?, [9, 3, 12, 0]);
        Ok(())
    }

    #[test]
    fn test_comparisons() -> Result<()> {
        assert_eq!(run(Opcode::Gtir, 4, 1, 3)?, [5, 3, 12, 1]);
        assert_eq!(run(Opcode::Gtri, 1, 4, 3)?, [5, 3, 12, 0]);
        assert_eq!(run(Opcode::Gtrr, 2, 0, 3)?, [5, 3, 12, 1]);
        assert_eq!(run(Opcode::Eqir, 3, 1, 3)?, [5, 3, 12, 1]);
        assert_eq!(run(Opcode::Eqri, 0, 4, 3)?, [5, 3, 12, 0]);
        assert_eq!(run(Opcode::Eqrr, 1, 1, 3)?, [5, 3, 12, 1]);
        Ok(())
    }

    #[test]
    fn test_output_may_be_an_input() -> Result<()> {
        assert_eq!(run(Opcode::Addr, 0, 0, 0)?, [10, 3, 12, 0]);
        Ok(())
    }

    #[test]
    fn test_register_out_of_range() {
        assert!(run(Opcode::Addr, 4, 0, 0).is_err());
        assert!(run(Opcode::Addi, 0, 4, 4).is_err());
        assert!(run(Opcode::Seti, 9, 0, 0).is_ok());
    }

    #[test]
    fn test_names() {
        for op in Opcode::ALL {
            assert_eq!(Opcode::from_name(op.name()), Some(op));
        }
        assert_eq!(Opcode::from_name("nope"), None);
    }
}
