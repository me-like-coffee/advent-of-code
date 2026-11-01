use crate::utils::{Answer, Day, Year, read_input};
use anyhow::{Context, Result};
use std::collections::HashMap;

type Bags<'a> = HashMap<&'a str, Vec<(&'a str, u32)>>;

// dull lime bags contain 2 posh green bags, 2 muted gold bags, 1 light bronze bag, 2 pale teal bags.
fn parse_line<'a>(data: &'a str, bags: &mut Bags<'a>) -> Result<()> {
    let (prefix, suffix) = data
        .split_once(" bags contain ")
        .with_context(|| format!("not a rule: {data:?}"))?;
    let mut contents = Vec::new();

    if !suffix.contains("no other") {
        for token in suffix.split(", ") {
            let (amount, ssuffix) = token
                .split_once(' ')
                .with_context(|| format!("not a bag count: {token:?}"))?;
            let amount: u32 = amount.parse()?;
            let (bag, _) = ssuffix
                .rsplit_once(' ')
                .with_context(|| format!("not a bag: {token:?}"))?;
            contents.push((bag, amount));
        }
    }
    bags.insert(prefix, contents);
    Ok(())
}

fn parse(data: &str) -> Result<Bags<'_>> {
    let mut bags = HashMap::new();
    for l in data.lines() {
        parse_line(l, &mut bags)?;
    }
    Ok(bags)
}

fn dfs(bags: &Bags, bag: &str, target: &str) -> bool {
    if let Some(contents) = bags.get(bag) {
        for (content, _) in contents {
            if content != &target {
                if dfs(bags, content, target) {
                    return true;
                }
            } else {
                return true;
            }
        }
    }

    false
}

fn part_one(data: &str) -> Result<usize> {
    let bags = parse(data)?;

    let target = "shiny gold";
    Ok(bags.keys().filter(|k| dfs(&bags, k, target)).count())
}

fn bag_count(bags: &Bags, bag: &str) -> u32 {
    let mut num_bags = 0;

    if let Some(contents) = bags.get(bag) {
        for (content_bag, amount) in contents {
            num_bags += amount * (1 + bag_count(bags, content_bag));
        }
    }
    num_bags
}

fn part_two(data: &str) -> Result<u32> {
    let bags = parse(data)?;

    let target = "shiny gold";
    Ok(bag_count(&bags, target))
}

/// Solves Year 2020, Day 7: Handy Haversacks.
///
/// # Errors
/// Returns an error if the input cannot be read or parsed.
pub fn solve() -> Result<(Answer, Answer)> {
    let data = read_input(Year(2020), Day(7))?;
    Ok((
        u64::try_from(part_one(&data)?)?.into(),
        u64::from(part_two(&data)?).into(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_example;

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test1_part_one() -> Result<()> {
        assert_eq!(part_one(&read_example(Year(2020), Day(7), 1)?)?, 4);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test1_part_two() -> Result<()> {
        assert_eq!(part_two(&read_example(Year(2020), Day(7), 1)?)?, 32);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test2_part_two() -> Result<()> {
        assert_eq!(part_two(&read_example(Year(2020), Day(7), 2)?)?, 126);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_solve() -> Result<()> {
        let (p1, p2) = solve()?;
        assert_eq!(p1, 274.into());
        assert_eq!(p2, 158730.into());
        Ok(())
    }
}
