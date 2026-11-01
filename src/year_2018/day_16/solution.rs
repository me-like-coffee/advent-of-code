use crate::utils::{Answer, Day, Year, read_input, sections};
use crate::year_2018::device::Opcode;
use anyhow::{Context, Result, ensure};
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Eq, Ord, PartialOrd, PartialEq)]
struct Sample {
    before: [u64; 4],
    instruction: [u64; 4],
    expected: [u64; 4],
}

impl Sample {
    /// Whether running the sample's instruction as `op` gives the expected registers.
    fn matches(&self, op: Opcode) -> bool {
        let [_, a, b, c] = self.instruction;
        let mut registers = self.before;
        op.execute(a, b, c, &mut registers).is_ok() && registers == self.expected
    }
}

fn into_array(data: &str) -> Result<[u64; 4]> {
    let data = data
        .replace("Before: [", "")
        .replace("After:  [", "")
        .replace([']', ','], "");
    let values = data
        .split_ascii_whitespace()
        .map(str::parse)
        .collect::<Result<Vec<u64>, _>>()?;

    values
        .try_into()
        .map_err(|v| anyhow::anyhow!("expected 4 values, found {v:?}"))
}

fn parse(data: &str) -> Result<(Vec<Sample>, Vec<[u64; 4]>)> {
    let mut samples = Vec::new();

    let (sample_data, program_data) = data.split_once("\n\n\n\n").unwrap_or((data, ""));
    let sample_data: Vec<_> = sections(sample_data).collect();

    for datum in sample_data {
        let tokens: Vec<_> = datum.split('\n').collect();
        ensure!(tokens.len() >= 3, "incomplete sample: {datum:?}");
        let before = into_array(tokens[0])?;
        let instruction = into_array(tokens[1])?;
        let expected = into_array(tokens[2])?;

        samples.push(Sample {
            before,
            instruction,
            expected,
        });
    }

    let mut program = Vec::new();
    for line in program_data.lines() {
        program.push(into_array(line)?);
    }

    Ok((samples, program))
}

pub fn part_one(data: &str) -> Result<usize> {
    let (samples, _) = parse(data)?;
    let mut three_or_more_opcodes = 0;

    for sample in samples {
        let sample_success = Opcode::ALL
            .into_iter()
            .filter(|op| sample.matches(*op))
            .count();

        if sample_success >= 3 {
            three_or_more_opcodes += 1;
        }
    }

    Ok(three_or_more_opcodes)
}

pub fn part_two(data: &str) -> Result<u64> {
    let (samples, program) = parse(data)?;
    let mut sample_map: HashMap<u64, Vec<&Sample>> = HashMap::new();
    for s in &samples {
        sample_map.entry(s.instruction[0]).or_default().push(s);
    }

    let mut sample_to_opcode: HashMap<u64, HashSet<Opcode>> = HashMap::new();
    for (sample_id, v) in &sample_map {
        for op in Opcode::ALL {
            if v.iter().all(|s| s.matches(op)) {
                sample_to_opcode.entry(*sample_id).or_default().insert(op);
            }
        }
    }

    let mut mappings: Vec<_> = sample_to_opcode.iter().map(|(op, hs)| (*op, hs)).collect();
    mappings.sort_by_key(|a| a.1.len());

    let mut opcode_map: HashMap<u64, Opcode> = HashMap::new();
    let mut mappings: VecDeque<_> = mappings.iter().map(|(id, _)| *id).collect();

    while let Some(id) = mappings.pop_front() {
        let mut hs = sample_to_opcode
            .remove(&id)
            .with_context(|| format!("opcode {id} has no candidates"))?;
        if hs.len() == 1 {
            let op = hs.drain().next().context("empty candidate set")?;
            opcode_map.insert(id, op);
            for hs in sample_to_opcode.values_mut() {
                hs.remove(&op);
            }
        } else {
            sample_to_opcode.insert(id, hs);
            mappings.push_back(id);
        }
    }

    let mut registers = [0u64; 4];
    for [number, a, b, c] in program {
        let op = opcode_map
            .get(&number)
            .with_context(|| format!("unknown opcode number {number}"))?;
        op.execute(a, b, c, &mut registers)?;
    }

    Ok(registers[0])
}

/// Solves Year 2018, Day 16: Chronal Classification.
///
/// # Errors
/// Returns an error if the input cannot be read or parsed, or the opcodes can't be identified.
pub fn solve() -> Result<(Answer, Answer)> {
    let data = read_input(Year(2018), Day(16))?;
    Ok((
        u64::try_from(part_one(&data)?)?.into(),
        part_two(&data)?.into(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_example;

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_part_one() -> Result<()> {
        assert_eq!(part_one(&read_example(Year(2018), Day(16), 1)?)?, 1);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_solve() -> Result<()> {
        let (p1, p2) = solve()?;
        assert_eq!(p1, 493.into());
        assert_eq!(p2, 445.into());
        Ok(())
    }
}
