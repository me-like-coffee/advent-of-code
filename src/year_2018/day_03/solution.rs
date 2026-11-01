use crate::utils::{Answer, Day, Year, read_input};
use anyhow::{Context, Result, ensure};
use std::collections::HashSet;

type IntType = i64;

#[derive(Debug)]
struct Fabric {
    id: IntType,
    ul_x: IntType,
    ul_y: IntType,
    lr_x: IntType,
    lr_y: IntType,
}

impl Fabric {
    // #1 @ 1,3: 4x4
    fn parse(data: &str) -> Result<Fabric> {
        let (prefix, suffix) = data
            .split_once(" @ ")
            .with_context(|| format!("not a claim: {data:?}"))?;
        let id = prefix.replace('#', "").trim().parse()?;

        let (prefix, suffix) = suffix
            .split_once(": ")
            .with_context(|| format!("not a claim: {data:?}"))?;
        let tokens: Vec<_> = prefix.split(',').collect();
        ensure!(tokens.len() == 2, "not a claim: {data:?}");
        let ul_x = tokens[0].trim().parse()?;
        let ul_y = tokens[1].trim().parse()?;

        let tokens: Vec<_> = suffix.split('x').collect();
        ensure!(tokens.len() == 2, "not a claim: {data:?}");
        let lr_x = ul_x + tokens[0].trim().parse::<IntType>()? - 1;
        let lr_y: IntType = ul_y + tokens[1].trim().parse::<IntType>()? - 1;

        Ok(Fabric {
            id,
            ul_x,
            ul_y,
            lr_x,
            lr_y,
        })
    }
}

fn get_coverage(fabrics: &[Fabric]) -> Vec<Vec<Vec<IntType>>> {
    let mut coverage = vec![vec![Vec::new(); 1000]; 1000];

    for fabric in fabrics {
        for x in fabric.ul_x..=fabric.lr_x {
            for y in fabric.ul_y..=fabric.lr_y {
                coverage[y as usize][x as usize].push(fabric.id);
            }
        }
    }

    coverage
}

fn part_one(data: &str) -> Result<usize> {
    let fabrics = data
        .lines()
        .map(Fabric::parse)
        .collect::<Result<Vec<_>>>()?;
    let coverage = get_coverage(&fabrics);

    Ok(coverage
        .iter()
        .flat_map(|r| r.iter())
        .filter(|v| v.len() > 1)
        .count())
}

fn part_two(data: &str) -> Result<IntType> {
    let fabrics = data
        .lines()
        .map(Fabric::parse)
        .collect::<Result<Vec<_>>>()?;

    let mut fabric_ids: HashSet<_> = fabrics.iter().map(|f| f.id).collect();

    let coverage = get_coverage(&fabrics);

    let filtered_coverage: Vec<_> = coverage
        .iter()
        .flat_map(|r| r.iter())
        .filter(|v| v.len() > 1)
        .collect();

    filtered_coverage
        .iter()
        .flat_map(|r| r.iter())
        .for_each(|c| {
            fabric_ids.remove(c);
        });

    fabric_ids
        .iter()
        .next()
        .copied()
        .context("every claim overlaps another")
}

/// Solves Year 2018, Day 3: No Matter How You Slice It.
///
/// # Errors
/// Returns an error if the input cannot be read or parsed.
pub fn solve() -> Result<(Answer, Answer)> {
    let data = read_input(Year(2018), Day(3))?;
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
        assert_eq!(part_one(&read_example(Year(2018), Day(3), 1)?)?, 4);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_part_two() -> Result<()> {
        assert_eq!(part_two(&read_example(Year(2018), Day(3), 1)?)?, 3);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_solve() -> Result<()> {
        let (p1, p2) = solve()?;
        assert_eq!(p1, 111266.into());
        assert_eq!(p2, 266.into());
        Ok(())
    }
}
