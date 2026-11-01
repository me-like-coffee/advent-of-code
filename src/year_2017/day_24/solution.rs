use crate::utils::{Answer, Day, Year, read_input};
use anyhow::{Context, Result};
use std::collections::{HashMap, HashSet};

type IntType = u32;

#[derive(Clone, Debug)]
struct Component {
    left: IntType,
    right: IntType,
    used: bool,
    left_used: bool,
    right_used: bool,
}

impl Component {
    fn parse(data: &str) -> Result<Component> {
        let (left, right) = data
            .split_once('/')
            .with_context(|| format!("not a component: {data:?}"))?;
        let left: IntType = left.parse()?;
        let right: IntType = right.parse()?;

        Ok(Component {
            left,
            right,
            used: false,
            left_used: false,
            right_used: false,
        })
    }

    fn strength(&self) -> IntType {
        self.left + self.right
    }
}

#[derive(Debug)]
struct Bridge {
    components: Vec<Component>,
    end_points: HashMap<IntType, Vec<usize>>,
    start_components: Vec<usize>,
}

impl Bridge {
    fn parse(data: &str) -> Result<Bridge> {
        let mut components = data
            .lines()
            .map(Component::parse)
            .collect::<Result<Vec<_>>>()?;

        let mut end_points: HashMap<IntType, Vec<usize>> = HashMap::new();
        components.iter_mut().enumerate().for_each(|(i, c)| {
            if c.left == 0 {
                c.used = true;
                c.left_used = true;
            }
            end_points.entry(c.left).or_default().push(i);
            if c.right == 0 {
                c.used = true;
                c.right_used = true;
            }
            end_points.entry(c.right).or_default().push(i);
        });

        let start_components: Vec<_> = end_points
            .get(&0)
            .context("no component with a 0 port")?
            .clone();

        Ok(Bridge {
            components,
            end_points,
            start_components,
        })
    }

    fn best_overall_bridge(&self) -> IntType {
        let mut best_strength = IntType::MIN;
        for i in &self.start_components {
            let mut bridges = HashSet::new();
            let mut bridge_components = self.components.clone();
            bridge_components[*i].used = true;
            self.dfs(bridge_components, vec![*i], &mut bridges);
            for bridge in &bridges {
                best_strength = best_strength.max(self.bridge_strength(bridge));
            }
        }
        best_strength
    }

    /// The strength of the longest bridge; among equally long bridges, the strongest.
    fn best_longest_bridge(&self) -> IntType {
        let mut best = (0, IntType::MIN);
        for i in &self.start_components {
            let mut bridges = HashSet::new();
            let mut bridge_components = self.components.clone();
            bridge_components[*i].used = true;
            self.dfs(bridge_components, vec![*i], &mut bridges);
            for bridge in &bridges {
                best = best.max((bridge.len(), self.bridge_strength(bridge)));
            }
        }
        best.1
    }

    fn bridge_strength(&self, bridge: &[usize]) -> IntType {
        bridge.iter().map(|i| self.components[*i].strength()).sum()
    }

    /// `curr_bridge` is never empty: it starts with a start component.
    fn dfs(
        &self,
        components: Vec<Component>,
        curr_bridge: Vec<usize>,
        bridges: &mut HashSet<Vec<usize>>,
    ) {
        let end_component = curr_bridge[curr_bridge.len() - 1];
        let end_value = if components[end_component].left_used {
            components[end_component].right
        } else {
            components[end_component].left
        };
        if let Some(indices) = self.end_points.get(&end_value) {
            if indices.iter().any(|i| {
                !components[*i].used
                    && (components[*i].left == end_value && !components[*i].right_used
                        || components[*i].right == end_value && !components[*i].left_used)
            }) {
                let mut next_components = components.clone();
                let mut next_bridge = curr_bridge.clone();
                for &i in indices {
                    if components[i].used {
                        continue;
                    }
                    if components[i].left == end_value && !components[i].left_used {
                        next_components[i].used = true;
                        next_components[i].left_used = true;
                        next_bridge.push(i);
                        self.dfs(next_components.clone(), next_bridge.clone(), bridges);
                        next_bridge.pop();
                        next_components[i].left_used = false;
                        next_components[i].used = false;
                    } else if components[i].right == end_value && !components[i].right_used {
                        next_components[i].used = true;
                        next_components[i].right_used = true;
                        next_bridge.push(i);
                        self.dfs(next_components.clone(), next_bridge.clone(), bridges);
                        next_bridge.pop();
                        next_components[i].right_used = false;
                        next_components[i].used = false;
                    }
                }
            } else {
                bridges.insert(curr_bridge);
            }
        } else {
            bridges.insert(curr_bridge);
        }
    }
}

fn part_one(data: &str) -> Result<IntType> {
    Ok(Bridge::parse(data)?.best_overall_bridge())
}

/// The original's answer check for this part was commented out; 1994 matches it.
fn part_two(data: &str) -> Result<IntType> {
    Ok(Bridge::parse(data)?.best_longest_bridge())
}

/// Solves Year 2017, Day 24: Electromagnetic Moat.
///
/// # Errors
/// Returns an error if the input cannot be read or parsed.
pub fn solve() -> Result<(Answer, Answer)> {
    let data = read_input(Year(2017), Day(24))?;
    Ok((
        u64::from(part_one(&data)?).into(),
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
        assert_eq!(part_one(&read_example(Year(2017), Day(24), 1)?)?, 31);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_part_one_full() -> Result<()> {
        assert_eq!(part_one(&read_input(Year(2017), Day(24))?)?, 2006);
        Ok(())
    }
}
