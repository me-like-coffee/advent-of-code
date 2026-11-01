use crate::utils::parser::unsigned_number;
use crate::utils::{Answer, Day, Year, read_input};
use anyhow::{Context, Result, anyhow};
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, multispace0},
    multi::separated_list1,
    sequence::{separated_pair, terminated},
};
use std::collections::{HashMap, VecDeque};

/// A chemical with a specific quantity.
#[derive(Debug, Clone, PartialEq, Eq)]
struct ChemicalQty {
    amount: u64,
    name: String,
}

/// A chemical reaction.
#[derive(Debug, Clone)]
struct Reaction {
    inputs: Vec<ChemicalQty>,
    output: ChemicalQty,
}

/// Parses a single chemical and its quantity (e.g., "10 ORE").
fn parse_chemical_qty(input: &str) -> IResult<&str, ChemicalQty> {
    separated_pair(unsigned_number, tag(" "), alpha1)
        .map(|(amount, name): (u64, &str)| ChemicalQty {
            amount,
            name: name.to_string(),
        })
        .parse(input)
}

/// Parses a single reaction (e.g., "10 ORE => 10 A").
fn parse_reaction(input: &str) -> IResult<&str, Reaction> {
    separated_pair(
        separated_list1(tag(", "), parse_chemical_qty),
        tag(" => "),
        parse_chemical_qty,
    )
    .map(|(inputs, output)| Reaction { inputs, output })
    .parse(input)
}

/// Parses all reactions from the input string.
fn parse_reactions(input: &str) -> IResult<&str, Vec<Reaction>> {
    separated_list1(terminated(line_ending, multispace0), parse_reaction).parse(input)
}

/// Solves Year 2019, Day 14: Space Stoichiometry.
///
/// # Errors
/// Returns an error if the input cannot be read or parsed.
pub fn solve() -> Result<(Answer, Answer)> {
    let input = read_input(Year(2019), Day(14))?;
    let (p1, p2) = calculate_solution(&input)?;
    Ok((p1.into(), p2.into()))
}

/// Core logic for Year 2019, Day 14.
fn calculate_solution(input: &str) -> Result<(u64, u64)> {
    let (_, reactions) =
        parse_reactions(input.trim()).map_err(|e| anyhow!("Failed to parse reactions: {}", e))?;

    let mut reaction_map = HashMap::new();
    for r in reactions {
        reaction_map.insert(r.output.name.clone(), r);
    }

    let p1 = calculate_ore_needed(&reaction_map, 1)?;

    let total_ore = 1_000_000_000_000u64;
    let mut low = total_ore / p1;
    let mut high = low * 2;

    // Ensure high is actually high enough
    while calculate_ore_needed(&reaction_map, high)? <= total_ore {
        low = high;
        high *= 2;
    }

    let mut p2 = low;
    while low <= high {
        let mid = low + (high - low) / 2;
        if mid == 0 {
            low = 1;
            continue;
        }
        let ore = calculate_ore_needed(&reaction_map, mid)?;
        if ore <= total_ore {
            p2 = mid;
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    Ok((p1, p2))
}

/// Calculates the ORE needed to produce a specific amount of FUEL.
fn calculate_ore_needed(reaction_map: &HashMap<String, Reaction>, fuel_amount: u64) -> Result<u64> {
    let mut in_degree = HashMap::new();
    // Initialize degrees for all chemicals present in the reaction map
    for name in reaction_map.keys() {
        in_degree.entry(name.clone()).or_insert(0);
    }
    in_degree.entry("ORE".to_string()).or_insert(0);

    for r in reaction_map.values() {
        for input in &r.inputs {
            *in_degree.entry(input.name.clone()).or_insert(0) += 1;
        }
    }

    let mut requirements = HashMap::new();
    requirements.insert("FUEL".to_string(), fuel_amount);

    let mut available_to_process: VecDeque<String> = in_degree
        .iter()
        .filter(|&(_, &deg)| deg == 0)
        .map(|(name, _)| name.clone())
        .collect();

    let mut ore_needed = 0;

    while let Some(current) = available_to_process.pop_front() {
        if current == "ORE" {
            continue;
        }

        let req_amount = requirements.get(&current).copied().unwrap_or(0);
        let reaction = reaction_map
            .get(&current)
            .ok_or_else(|| anyhow!("Reaction not found for chemical: {}", current))?;

        let batch_size = reaction.output.amount;
        let num_batches = req_amount.div_ceil(batch_size);

        for input in &reaction.inputs {
            let total_input_needed = num_batches * input.amount;
            if input.name == "ORE" {
                ore_needed += total_input_needed;
            } else {
                *requirements.entry(input.name.clone()).or_insert(0) += total_input_needed;

                let degree = in_degree.get_mut(&input.name).context("Degree not found")?;
                *degree -= 1;
                if *degree == 0 {
                    available_to_process.push_back(input.name.clone());
                }
            }
        }
    }

    Ok(ore_needed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_example;

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_example_1() -> Result<()> {
        let input = &read_example(Year(2019), Day(14), 1)?;
        let (p1, _) = calculate_solution(input)?;
        assert_eq!(p1, 31);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_example_2() -> Result<()> {
        let input = &read_example(Year(2019), Day(14), 2)?;
        let (p1, _) = calculate_solution(input)?;
        assert_eq!(p1, 165);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_example_3() -> Result<()> {
        let input = &read_example(Year(2019), Day(14), 3)?;
        let (p1, p2) = calculate_solution(input)?;
        assert_eq!(p1, 13312);
        assert_eq!(p2, 82892753);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_example_4() -> Result<()> {
        let input = &read_example(Year(2019), Day(14), 4)?;
        let (p1, p2) = calculate_solution(input)?;
        assert_eq!(p1, 180697);
        assert_eq!(p2, 5586022);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_example_5() -> Result<()> {
        let input = &read_example(Year(2019), Day(14), 5)?;
        let (p1, p2) = calculate_solution(input)?;
        assert_eq!(p1, 2210736);
        assert_eq!(p2, 460664);
        Ok(())
    }
}
