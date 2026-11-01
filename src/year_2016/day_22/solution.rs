use crate::utils::{Answer, Day, Year, read_input};
use anyhow::{Context, Result, ensure};

#[derive(Clone, Debug, PartialEq)]
struct Node {
    x: usize,
    y: usize,
    used: i32,
    available: i32,
}

impl Node {
    // /dev/grid/node-x1-y2   11T    7T     4T   63%
    fn parse(data: &str) -> Result<(usize, usize, Node)> {
        let data = data.replace("/dev/grid/node-", "").replace(['T', '%'], "");

        let tokens: Vec<_> = data
            .split_ascii_whitespace()
            .filter(|t| !t.is_empty())
            .collect();
        ensure!(tokens.len() == 5, "unexpected node line: {data}");

        let position = tokens[0].replace('x', "").replace("-y", " ");

        let position: Vec<_> = position.split_ascii_whitespace().collect();
        ensure!(position.len() == 2, "unexpected node name: {}", tokens[0]);

        let x = position[0].parse()?;
        let y = position[1].parse()?;
        let used = tokens[2].parse()?;
        let available = tokens[3].parse()?;

        let node = Node {
            x,
            y,
            used,
            available,
        };

        Ok((x, y, node))
    }
}

fn parse(data: &str) -> Result<Vec<Vec<Node>>> {
    let mut grid = vec![Vec::new(); 31];

    for l in data.lines() {
        let (x, y, node) = Node::parse(l)?;
        let row: &mut Vec<Node> = grid.get_mut(y).context("node outside the grid")?;
        ensure!(row.len() == x, "nodes out of order at x{x}-y{y}");
        row.push(node);
    }

    Ok(grid)
}

fn part_one(data: &str) -> Result<u64> {
    let grid = parse(data)?;
    let mut viable_pairs = 0;

    for node_a in grid.iter().flat_map(|r| r.iter()) {
        for node_b in grid.iter().flat_map(|r| r.iter()) {
            if node_a != node_b && node_a.used != 0 && node_b.available >= node_a.used {
                viable_pairs += 1;
            }
        }
    }
    Ok(viable_pairs)
}

fn part_two(data: &str) -> Result<u64> {
    // x steps to move empty space to node to the left of goal data
    // => distance from start to space left of start, avoiding impassable nodes
    // 5 steps to move data one space left * (start - end - 1) => (30 - 0 - 1)
    // 1 to move to final location
    let grid = parse(data)?;

    // find the barrier
    let min_x = grid[15]
        .iter()
        .filter(|n| n.used > 100)
        .map(|n| n.x)
        .min()
        .context("no barrier in row 15")?;

    // find the start
    let x = 13;
    let y = 27;

    let mut distance = 0;
    let (mut xx, mut yy) = (x, y);
    while xx != min_x - 1 {
        xx -= 1;
        distance += 1;
    }
    while yy != 0 {
        yy -= 1;
        distance += 1;
    }
    while xx != 29 {
        xx += 1;
        distance += 1;
    }

    let total_distance = distance + 29 * 5 + 1;
    Ok(total_distance)
}

/// Solves Year 2016, Day 22: Grid Computing.
///
/// Part two uses the grid size, the barrier row and the empty node's position
/// worked out by hand for this input.
///
/// # Errors
/// Returns an error if the input cannot be read or parsed.
pub fn solve() -> Result<(Answer, Answer)> {
    let data = read_input(Year(2016), Day(22))?;
    Ok((part_one(&data)?.into(), part_two(&data)?.into()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_example;

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test1_part_one() -> Result<()> {
        assert_eq!(part_one(&read_example(Year(2016), Day(22), 1)?)?, 7);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_solve() -> Result<()> {
        let (p1, p2) = solve()?;
        assert_eq!(p1, 934.into());
        assert_eq!(p2, 207.into());
        Ok(())
    }
}
