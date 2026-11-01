use crate::utils::{Answer, Day, Year, read_extra, read_input};
use anyhow::{Context, Result, bail};
use std::collections::HashMap;

type Grid = Vec<Vec<u8>>;
type TwoGrid = [[u8; 2]; 2];
type ThreeGrid = [[u8; 3]; 3];
type FourGrid = [[u8; 4]; 4];
type TwoMap = HashMap<TwoGrid, ThreeGrid>;
type ThreeMap = HashMap<ThreeGrid, FourGrid>;

enum Rule {
    Two(TwoGrid, ThreeGrid),
    Three(ThreeGrid, FourGrid),
}

// ../.# => ##./#../...
// .#./..#/### => #..#/..../..../#..#
fn parse_rule(data: &str) -> Result<Rule> {
    let (prefix, suffix) = data
        .split_once(" => ")
        .with_context(|| format!("not a rule: {data:?}"))?;

    let prefix_tokens: Vec<_> = prefix.split('/').collect();
    match prefix_tokens.len() {
        2 => {
            let mut rule_prefix: TwoGrid = [[0u8; 2]; 2];
            for (y, row) in prefix_tokens.iter().enumerate() {
                for (x, c) in row.chars().enumerate() {
                    rule_prefix[y][x] = c as u8;
                }
            }

            let suffix_tokens: Vec<_> = suffix.split('/').collect();
            let mut rule_suffix: ThreeGrid = [[0u8; 3]; 3];
            for (y, row) in suffix_tokens.iter().enumerate() {
                for (x, c) in row.chars().enumerate() {
                    rule_suffix[y][x] = c as u8;
                }
            }

            Ok(Rule::Two(rule_prefix, rule_suffix))
        }
        3 => {
            let mut rule_prefix: ThreeGrid = [[0u8; 3]; 3];
            for (y, row) in prefix_tokens.iter().enumerate() {
                for (x, c) in row.chars().enumerate() {
                    rule_prefix[y][x] = c as u8;
                }
            }

            let suffix_tokens: Vec<_> = suffix.split('/').collect();
            let mut rule_suffix: FourGrid = [[0u8; 4]; 4];
            for (y, row) in suffix_tokens.iter().enumerate() {
                for (x, c) in row.chars().enumerate() {
                    rule_suffix[y][x] = c as u8;
                }
            }

            Ok(Rule::Three(rule_prefix, rule_suffix))
        }
        _ => bail!("rule must match a 2x2 or 3x3 block: {data:?}"),
    }
}

fn insert_two_rule(rules: &mut TwoMap, rule_prefix: &TwoGrid, rule_suffix: &ThreeGrid) {
    rules.insert(*rule_prefix, *rule_suffix);

    let mut permuted_block = *rule_prefix;

    // flip horizontal
    permuted_block[0].copy_from_slice(&rule_prefix[1]);
    permuted_block[1].copy_from_slice(&rule_prefix[0]);
    rules.insert(permuted_block, *rule_suffix);

    // flip vertical
    permuted_block[0].copy_from_slice(&rule_prefix[0]);
    permuted_block[0].reverse();
    permuted_block[1].copy_from_slice(&rule_prefix[1]);
    permuted_block[1].reverse();
    rules.insert(permuted_block, *rule_suffix);

    // rotate 90
    for i in 0..permuted_block.len() {
        for j in i..permuted_block.len() {
            permuted_block[i][j] = rule_prefix[j][i];
            permuted_block[j][i] = rule_prefix[i][j];
        }
    }
    permuted_block.swap(0, 1);
    rules.insert(permuted_block, *rule_suffix);

    // rotate 270
    permuted_block[0].reverse();
    permuted_block[1].reverse();
    rules.insert(permuted_block, *rule_suffix);

    // rotate 180
    permuted_block[0].copy_from_slice(&rule_prefix[1]);
    permuted_block[0].reverse();
    permuted_block[1].copy_from_slice(&rule_prefix[0]);
    permuted_block[1].reverse();
    rules.insert(permuted_block, *rule_suffix);
}

fn insert_three_rule(rules: &mut ThreeMap, rule_prefix: &ThreeGrid, rule_suffix: &FourGrid) {
    rules.insert(*rule_prefix, *rule_suffix);

    let mut permuted_block = *rule_prefix;

    // flip horizontal
    permuted_block[0].copy_from_slice(&rule_prefix[2]);
    permuted_block[2].copy_from_slice(&rule_prefix[0]);
    rules.insert(permuted_block, *rule_suffix);

    // flip vertical
    permuted_block[0].copy_from_slice(&rule_prefix[0]);
    permuted_block[0].reverse();
    permuted_block[1].copy_from_slice(&rule_prefix[1]);
    permuted_block[1].reverse();
    permuted_block[2].copy_from_slice(&rule_prefix[2]);
    permuted_block[2].reverse();
    rules.insert(permuted_block, *rule_suffix);

    // rotate 90
    for i in 0..permuted_block.len() {
        for j in i..permuted_block.len() {
            permuted_block[i][j] = rule_prefix[j][i];
            permuted_block[j][i] = rule_prefix[i][j];
        }
    }
    permuted_block.swap(0, 2);
    rules.insert(permuted_block, *rule_suffix);

    // rotate 270
    permuted_block[0].reverse();
    permuted_block[1].reverse();
    permuted_block[2].reverse();
    rules.insert(permuted_block, *rule_suffix);

    // rotate 180
    permuted_block[0].copy_from_slice(&rule_prefix[2]);
    permuted_block[0].reverse();
    permuted_block[1].copy_from_slice(&rule_prefix[1]);
    permuted_block[1].reverse();
    permuted_block[2].copy_from_slice(&rule_prefix[0]);
    permuted_block[2].reverse();
    rules.insert(permuted_block, *rule_suffix);
}

fn parse_rules(data: &str) -> Result<(TwoMap, ThreeMap)> {
    let mut two_map = TwoMap::new();
    let mut three_map = ThreeMap::new();

    for line in data.lines() {
        match parse_rule(line)? {
            Rule::Three(rule_prefix, rule_suffix) => {
                insert_three_rule(&mut three_map, &rule_prefix, &rule_suffix);
            }
            Rule::Two(rule_prefix, rule_suffix) => {
                insert_two_rule(&mut two_map, &rule_prefix, &rule_suffix);
            }
        }
    }

    Ok((two_map, three_map))
}

fn parse_start(data: &str) -> Grid {
    let mut start = Vec::new();

    for line in data.lines() {
        start.push(line.bytes().collect());
    }

    start
}

fn enhance_grid(grid: &Grid, two_map: &TwoMap, three_map: &ThreeMap) -> Result<Grid> {
    if grid.len().is_multiple_of(2) {
        let enhanced_size = (grid.len() / 2) * 3;
        let mut enhanced_grid = vec![Vec::new(); enhanced_size];
        let mut rule_prefix: TwoGrid = [[0; 2]; 2];
        let mut y_dest = 0;
        for y_base in (0..grid.len()).step_by(2) {
            for x in (0..grid[0].len()).step_by(2) {
                for y in y_base..y_base + 2 {
                    rule_prefix[y - y_base].copy_from_slice(&grid[y][x..x + 2]);
                }

                let rule_suffix = two_map
                    .get(&rule_prefix)
                    .with_context(|| format!("rule not found: {rule_prefix:?}"))?;
                enhanced_grid[y_dest].extend_from_slice(&rule_suffix[0]);
                enhanced_grid[y_dest + 1].extend_from_slice(&rule_suffix[1]);
                enhanced_grid[y_dest + 2].extend_from_slice(&rule_suffix[2]);
            }
            y_dest += 3;
        }

        Ok(enhanced_grid)
    } else if grid.len().is_multiple_of(3) {
        let enhanced_size = (grid.len() / 3) * 4;
        let mut enhanced_grid = vec![Vec::new(); enhanced_size];
        let mut rule_prefix: ThreeGrid = [[0; 3]; 3];
        let mut y_dest = 0;
        for y_base in (0..grid.len()).step_by(3) {
            for x in (0..grid[0].len()).step_by(3) {
                for y in y_base..y_base + 3 {
                    rule_prefix[y - y_base].copy_from_slice(&grid[y][x..x + 3]);
                }

                let rule_suffix = three_map
                    .get(&rule_prefix)
                    .with_context(|| format!("rule not found: {rule_prefix:?}"))?;
                enhanced_grid[y_dest].extend_from_slice(&rule_suffix[0]);
                enhanced_grid[y_dest + 1].extend_from_slice(&rule_suffix[1]);
                enhanced_grid[y_dest + 2].extend_from_slice(&rule_suffix[2]);
                enhanced_grid[y_dest + 3].extend_from_slice(&rule_suffix[3]);
            }
            y_dest += 4;
        }

        Ok(enhanced_grid)
    } else {
        bail!("grid size {} is not a multiple of 2 or 3", grid.len());
    }
}

/// `start` is the starting pattern from the puzzle text.
fn count_on(rule_data: &str, start: &str, num_iterations: usize) -> Result<usize> {
    let (two_map, three_map) = parse_rules(rule_data)?;
    let mut grid = parse_start(start);

    for _ in 0..num_iterations {
        grid = enhance_grid(&grid, &two_map, &three_map)?;
    }

    Ok(grid
        .iter()
        .flat_map(|r| r.iter())
        .filter(|&&e| e == b'#')
        .count())
}

/// Solves Year 2017, Day 21: Fractal Art.
///
/// # Errors
/// Returns an error if the input or the starting pattern cannot be read or parsed.
pub fn solve() -> Result<(Answer, Answer)> {
    let rules = read_input(Year(2017), Day(21))?;
    let start = read_extra(Year(2017), Day(21), "start")?;
    Ok((
        u64::try_from(count_on(&rules, &start, 5)?)?.into(),
        u64::try_from(count_on(&rules, &start, 18)?)?.into(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_example;

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_part_one() -> Result<()> {
        let rules = read_example(Year(2017), Day(21), 1)?;
        let start = read_extra(Year(2017), Day(21), "start")?;
        assert_eq!(count_on(&rules, &start, 2)?, 12);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_solve() -> Result<()> {
        let (p1, p2) = solve()?;
        assert_eq!(p1, 110.into());
        assert_eq!(p2, 1277716.into());
        Ok(())
    }
}
