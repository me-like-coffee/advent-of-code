use crate::utils::{Answer, Day, Year, read_input};
use anyhow::Result;
use std::collections::HashMap;

/// Solves Year 2023, Day 12: Hot Springs.
///
/// # Errors
/// Returns an error if the input cannot be read or parsed.
pub fn solve() -> Result<(Answer, Answer)> {
    let input = read_input(Year(2023), Day(12))?;
    let part1 = part_1(&input);
    let part2 = part_2(&input);
    Ok((part1.into(), part2.into()))
}

fn part_1(input: &str) -> u64 {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut parts = line.split_whitespace();
            let springs = parts.next().expect("Missing springs part");
            let groups: Vec<usize> = parts
                .next()
                .expect("Missing groups part")
                .split(',')
                .map(|s| s.parse().expect("Invalid group size"))
                .collect();
            count_arrangements(springs, &groups)
        })
        .sum()
}

fn part_2(input: &str) -> u64 {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut parts = line.split_whitespace();
            let springs_raw = parts.next().expect("Missing springs part");
            let groups_raw = parts.next().expect("Missing groups part");

            let mut springs = springs_raw.to_string();
            for _ in 0..4 {
                springs.push('?');
                springs.push_str(springs_raw);
            }

            let groups_one: Vec<usize> = groups_raw
                .split(',')
                .map(|s| s.parse().expect("Invalid group size"))
                .collect();
            let mut groups = Vec::with_capacity(groups_one.len() * 5);
            for _ in 0..5 {
                groups.extend(&groups_one);
            }

            count_arrangements(&springs, &groups)
        })
        .sum()
}

fn count_arrangements(springs: &str, groups: &[usize]) -> u64 {
    let mut memo = HashMap::new();
    count_recursive(springs.as_bytes(), groups, 0, 0, 0, &mut memo)
}

fn count_recursive(
    springs: &[u8],
    groups: &[usize],
    spring_idx: usize,
    group_idx: usize,
    current_group_len: usize,
    memo: &mut HashMap<(usize, usize, usize), u64>,
) -> u64 {
    let key = (spring_idx, group_idx, current_group_len);
    if let Some(&res) = memo.get(&key) {
        return res;
    }

    if spring_idx == springs.len() {
        // End of springs
        return if (group_idx == groups.len() && current_group_len == 0)
            || (group_idx == groups.len() - 1 && current_group_len == groups[group_idx])
        {
            1
        } else {
            0
        };
    }

    let mut res = 0;
    let char_at = springs[spring_idx];

    // Try treating as '.'
    if char_at == b'.' || char_at == b'?' {
        if current_group_len == 0 {
            res += count_recursive(springs, groups, spring_idx + 1, group_idx, 0, memo);
        } else if group_idx < groups.len() && current_group_len == groups[group_idx] {
            res += count_recursive(springs, groups, spring_idx + 1, group_idx + 1, 0, memo);
        }
    }

    // Try treating as '#'
    if (char_at == b'#' || char_at == b'?')
        && group_idx < groups.len()
        && current_group_len < groups[group_idx]
    {
        res += count_recursive(
            springs,
            groups,
            spring_idx + 1,
            group_idx,
            current_group_len + 1,
            memo,
        );
    }

    memo.insert(key, res);
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_example;
    use anyhow::Context;

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_example() -> Result<()> {
        let input = &read_example(Year(2023), Day(12), 1)?;
        assert_eq!(part_1(input), 21);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_part2_example() -> Result<()> {
        let input = &read_example(Year(2023), Day(12), 1)?;
        assert_eq!(part_2(input), 525152);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_individual_rows() -> Result<()> {
        let example = read_example(Year(2023), Day(12), 1)?;
        let expected = [1, 4, 1, 1, 4, 10];
        assert_eq!(example.lines().count(), expected.len());
        for (line, want) in example.lines().zip(expected) {
            let (springs, groups) = line.split_once(' ').context("missing groups")?;
            let groups = groups
                .split(',')
                .map(str::parse)
                .collect::<Result<Vec<usize>, _>>()?;
            assert_eq!(count_arrangements(springs, &groups), want, "{line}");
        }
        Ok(())
    }
}
