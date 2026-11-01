use crate::utils::{Answer, Day, Year, read_input};
use anyhow::{Context, Result};
use std::collections::{HashMap, HashSet};

type Graph = HashMap<String, HashSet<String>>;
type Visited = HashSet<String>;

// start-b
// A-c
fn parse(data: &str) -> Result<Graph> {
    let mut graph: Graph = HashMap::new();

    for line in data.lines() {
        let (a, b) = line
            .split_once('-')
            .with_context(|| format!("not an edge: {line:?}"))?;
        graph
            .entry(a.to_string())
            .or_default()
            .insert(b.to_string());
        graph
            .entry(b.to_string())
            .or_default()
            .insert(a.to_string());
    }

    Ok(graph)
}

fn dfs(
    graph: &Graph,
    curr: &str,
    path: Vec<String>,
    visited: Visited,
    paths: &mut HashSet<Vec<String>>,
) {
    if curr == "end" {
        paths.insert(path);
    } else {
        for neighbor in graph.get(curr).into_iter().flatten() {
            let Some(c) = neighbor.chars().next() else {
                continue;
            };
            if c.is_ascii_uppercase() || (c.is_ascii_lowercase() && !visited.contains(neighbor)) {
                let mut new_path = path.clone();
                let mut new_visited = visited.clone();
                new_path.push(neighbor.clone());
                if c.is_ascii_lowercase() {
                    new_visited.insert(neighbor.clone());
                }
                dfs(graph, neighbor, new_path, new_visited, paths);
            }
        }
    }
}

fn part_one(data: &str) -> Result<usize> {
    let graph = parse(data)?;

    let path = vec!["start".to_string()];
    let visited = HashSet::from(["start".to_string()]);
    let mut paths = HashSet::new();
    dfs(&graph, "start", path, visited, &mut paths);

    Ok(paths.len())
}

fn dfs_two(
    graph: &Graph,
    curr: &str,
    path: Vec<String>,
    visited: HashMap<String, usize>,
    paths: &mut HashSet<Vec<String>>,
) {
    if curr == "end" {
        paths.insert(path);
    } else {
        for neighbor in graph.get(curr).into_iter().flatten() {
            if neighbor == "start" {
                continue;
            }

            let Some(c) = neighbor.chars().next() else {
                continue;
            };
            if c.is_ascii_uppercase() {
                let mut new_path = path.clone();
                let new_visited = visited.clone();
                new_path.push(neighbor.clone());
                dfs_two(graph, neighbor, new_path, new_visited, paths);
            } else if visited.iter().all(|(_, n)| n == &1)
                || visited.iter().filter(|(_, n)| n == &&2).count() == 1
            {
                let mut new_path = path.clone();
                let mut new_visited = visited.clone();
                new_path.push(neighbor.clone());
                *new_visited.entry(neighbor.clone()).or_insert(0) += 1;
                dfs_two(graph, neighbor, new_path, new_visited, paths);
            }
        }
    }
}

fn part_two(data: &str) -> Result<usize> {
    let graph = parse(data)?;

    let path = vec!["start".to_string()];
    let visited = HashMap::new();
    let mut paths = HashSet::new();
    dfs_two(&graph, "start", path, visited, &mut paths);

    Ok(paths.len())
}

/// Solves Year 2021, Day 12: Passage Pathing.
///
/// # Errors
/// Returns an error if the input cannot be read or parsed.
pub fn solve() -> Result<(Answer, Answer)> {
    let data = read_input(Year(2021), Day(12))?;
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
    fn test1_part_one() -> Result<()> {
        let data = read_example(Year(2021), Day(12), 1)?;
        assert_eq!(part_one(&data)?, 10);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test2_part_one() -> Result<()> {
        let data = read_example(Year(2021), Day(12), 2)?;
        assert_eq!(part_one(&data)?, 19);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test3_part_one() -> Result<()> {
        let data = read_example(Year(2021), Day(12), 3)?;
        assert_eq!(part_one(&data)?, 226);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test1_part_two() -> Result<()> {
        let data = read_example(Year(2021), Day(12), 1)?;
        assert_eq!(part_two(&data)?, 36);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test2_part_two() -> Result<()> {
        let data = read_example(Year(2021), Day(12), 2)?;
        assert_eq!(part_two(&data)?, 103);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test3_part_two() -> Result<()> {
        let data = read_example(Year(2021), Day(12), 3)?;
        assert_eq!(part_two(&data)?, 3509);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_solve() -> Result<()> {
        let (p1, p2) = solve()?;
        assert_eq!(p1, 4573.into());
        assert_eq!(p2, 117509.into());
        Ok(())
    }
}
