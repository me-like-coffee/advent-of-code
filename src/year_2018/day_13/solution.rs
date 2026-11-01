use crate::utils::{Answer, Day, Year, read_input};
use anyhow::{Result, bail};
use std::collections::{BTreeSet, HashSet};

/// (y, x, heading, intersections passed mod 3), so carts sort in move order.
type Cart = (usize, usize, Direction, usize);

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Direction {
    East,
    North,
    South,
    West,
}

impl Direction {
    fn left(&self) -> Direction {
        match *self {
            Direction::East => Direction::North,
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }
    fn right(&self) -> Direction {
        match *self {
            Direction::East => Direction::South,
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

fn parse(data: &str) -> Vec<Vec<u8>> {
    data.lines().map(|l| l.bytes().collect()).collect()
}

fn get_carts_and_intersections(grid: &[Vec<u8>]) -> (BTreeSet<Cart>, HashSet<(usize, usize)>) {
    let mut carts = BTreeSet::new();
    let mut intersections = HashSet::new();

    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            match *c {
                b'>' => {
                    carts.insert((y, x, Direction::East, 0usize));
                }
                b'^' => {
                    carts.insert((y, x, Direction::North, 0usize));
                }
                b'v' => {
                    carts.insert((y, x, Direction::South, 0usize));
                }
                b'<' => {
                    carts.insert((y, x, Direction::West, 0usize));
                }
                b'+' => {
                    intersections.insert((x, y));
                }
                _ => {}
            }
        }
    }

    (carts, intersections)
}

fn update_grid(carts: &BTreeSet<Cart>, grid: &mut [Vec<u8>]) {
    for (y, x, direction, _) in carts {
        match *direction {
            Direction::East => {
                grid[*y][*x] = b'-';
            }
            Direction::North => {
                grid[*y][*x] = b'|';
            }
            Direction::South => {
                grid[*y][*x] = b'|';
            }
            Direction::West => {
                grid[*y][*x] = b'-';
            }
        }
    }
}

fn part_one(data: &str) -> Result<(usize, usize)> {
    let mut grid = parse(data);
    let (mut carts, _intersections) = get_carts_and_intersections(&grid);
    update_grid(&carts, &mut grid);

    let mut positions: BTreeSet<(usize, usize)> =
        carts.iter().map(|(y, x, _, _)| (*x, *y)).collect();

    loop {
        let mut new_positions = BTreeSet::new();
        let mut new_carts = BTreeSet::new();
        for (y, x, direction, i) in carts {
            match direction {
                Direction::East => {
                    let new_x = x + 1;
                    if positions.contains(&(new_x, y)) || new_positions.contains(&(new_x, y)) {
                        return Ok((new_x, y));
                    }
                    positions.remove(&(x, y));
                    new_positions.insert((new_x, y));
                    match grid[y][new_x] {
                        b'-' => {
                            new_carts.insert((y, new_x, direction, i));
                        }
                        b'+' => match i {
                            0 => {
                                new_carts.insert((y, new_x, direction.left(), (i + 1) % 3));
                            }
                            1 => {
                                new_carts.insert((y, new_x, direction, (i + 1) % 3));
                            }
                            2 => {
                                new_carts.insert((y, new_x, direction.right(), (i + 1) % 3));
                            }
                            _ => {
                                unreachable!()
                            }
                        },
                        b'/' => {
                            new_carts.insert((y, new_x, direction.left(), i));
                        }
                        b'\\' => {
                            new_carts.insert((y, new_x, direction.right(), i));
                        }
                        c => bail!("cart ran onto {:?}", c as char),
                    }
                }
                Direction::North => {
                    let new_y = y - 1;
                    if positions.contains(&(x, new_y)) || new_positions.contains(&(x, new_y)) {
                        return Ok((x, new_y));
                    }
                    positions.remove(&(x, y));
                    new_positions.insert((x, new_y));
                    match grid[new_y][x] {
                        b'|' => {
                            new_carts.insert((new_y, x, direction, i));
                        }
                        b'+' => match i {
                            0 => {
                                new_carts.insert((new_y, x, direction.left(), (i + 1) % 3));
                            }
                            1 => {
                                new_carts.insert((new_y, x, direction, (i + 1) % 3));
                            }
                            2 => {
                                new_carts.insert((new_y, x, direction.right(), (i + 1) % 3));
                            }
                            _ => {
                                unreachable!()
                            }
                        },
                        b'/' => {
                            new_carts.insert((new_y, x, direction.right(), i));
                        }
                        b'\\' => {
                            new_carts.insert((new_y, x, direction.left(), i));
                        }
                        c => bail!("cart ran onto {:?}", c as char),
                    }
                }
                Direction::South => {
                    let new_y = y + 1;
                    if positions.contains(&(x, new_y)) || new_positions.contains(&(x, new_y)) {
                        return Ok((x, new_y));
                    }
                    positions.remove(&(x, y));
                    new_positions.insert((x, new_y));
                    match grid[new_y][x] {
                        b'|' => {
                            new_carts.insert((new_y, x, direction, i));
                        }
                        b'+' => match i {
                            0 => {
                                new_carts.insert((new_y, x, direction.left(), (i + 1) % 3));
                            }
                            1 => {
                                new_carts.insert((new_y, x, direction, (i + 1) % 3));
                            }
                            2 => {
                                new_carts.insert((new_y, x, direction.right(), (i + 1) % 3));
                            }
                            _ => {
                                unreachable!()
                            }
                        },
                        b'/' => {
                            new_carts.insert((new_y, x, direction.right(), i));
                        }
                        b'\\' => {
                            new_carts.insert((new_y, x, direction.left(), i));
                        }
                        c => bail!("cart ran onto {:?}", c as char),
                    }
                }
                Direction::West => {
                    let new_x = x - 1;
                    if positions.contains(&(new_x, y)) || new_positions.contains(&(new_x, y)) {
                        return Ok((new_x, y));
                    }
                    positions.remove(&(x, y));
                    new_positions.insert((new_x, y));
                    match grid[y][new_x] {
                        b'-' => {
                            new_carts.insert((y, new_x, direction, i));
                        }
                        b'+' => match i {
                            0 => {
                                new_carts.insert((y, new_x, direction.left(), (i + 1) % 3));
                            }
                            1 => {
                                new_carts.insert((y, new_x, direction, (i + 1) % 3));
                            }
                            2 => {
                                new_carts.insert((y, new_x, direction.right(), (i + 1) % 3));
                            }
                            _ => {
                                unreachable!()
                            }
                        },
                        b'/' => {
                            new_carts.insert((y, new_x, direction.left(), i));
                        }
                        b'\\' => {
                            new_carts.insert((y, new_x, direction.right(), i));
                        }
                        c => bail!("cart ran onto {:?}", c as char),
                    }
                }
            }
        }

        positions = new_positions;
        carts = new_carts;
    }
}

fn part_two(data: &str) -> Result<(usize, usize)> {
    let mut grid = parse(data);
    let (mut carts, _intersections) = get_carts_and_intersections(&grid);
    update_grid(&carts, &mut grid);

    let mut positions: BTreeSet<(usize, usize)> =
        carts.iter().map(|(y, x, _, _)| (*x, *y)).collect();

    loop {
        if carts.len() == 1
            && let Some(&(y, x, _, _)) = carts.first()
        {
            return Ok((x, y));
        }

        let mut new_positions = BTreeSet::new();
        let mut new_carts = BTreeSet::new();
        for (y, x, direction, i) in carts {
            match direction {
                Direction::East => {
                    let new_x = x + 1;
                    if positions.contains(&(new_x, y)) || new_positions.contains(&(new_x, y)) {
                        positions.remove(&(new_x, y));
                        new_positions.remove(&(new_x, y));

                        for (yy, xx, direction, i) in &new_carts {
                            if new_x == *xx && y == *yy {
                                new_carts.remove(&(y, new_x, *direction, *i));
                                break;
                            }
                        }
                        continue;
                    }
                    positions.remove(&(x, y));
                    new_positions.insert((new_x, y));
                    match grid[y][new_x] {
                        b'-' => {
                            new_carts.insert((y, new_x, direction, i));
                        }
                        b'+' => match i {
                            0 => {
                                new_carts.insert((y, new_x, direction.left(), (i + 1) % 3));
                            }
                            1 => {
                                new_carts.insert((y, new_x, direction, (i + 1) % 3));
                            }
                            2 => {
                                new_carts.insert((y, new_x, direction.right(), (i + 1) % 3));
                            }
                            _ => {
                                unreachable!()
                            }
                        },
                        b'/' => {
                            new_carts.insert((y, new_x, direction.left(), i));
                        }
                        b'\\' => {
                            new_carts.insert((y, new_x, direction.right(), i));
                        }
                        c => bail!("cart ran onto {:?}", c as char),
                    }
                }
                Direction::North => {
                    let new_y = y - 1;
                    if positions.contains(&(x, new_y)) || new_positions.contains(&(x, new_y)) {
                        positions.remove(&(x, new_y));
                        new_positions.remove(&(x, new_y));

                        for (yy, xx, direction, i) in &new_carts {
                            if x == *xx && new_y == *yy {
                                new_carts.remove(&(new_y, x, *direction, *i));
                                break;
                            }
                        }
                        continue;
                    }
                    positions.remove(&(x, y));
                    new_positions.insert((x, new_y));
                    match grid[new_y][x] {
                        b'|' => {
                            new_carts.insert((new_y, x, direction, i));
                        }
                        b'+' => match i {
                            0 => {
                                new_carts.insert((new_y, x, direction.left(), (i + 1) % 3));
                            }
                            1 => {
                                new_carts.insert((new_y, x, direction, (i + 1) % 3));
                            }
                            2 => {
                                new_carts.insert((new_y, x, direction.right(), (i + 1) % 3));
                            }
                            _ => {
                                unreachable!()
                            }
                        },
                        b'/' => {
                            new_carts.insert((new_y, x, direction.right(), i));
                        }
                        b'\\' => {
                            new_carts.insert((new_y, x, direction.left(), i));
                        }
                        c => bail!("cart ran onto {:?}", c as char),
                    }
                }
                Direction::South => {
                    let new_y = y + 1;
                    if positions.contains(&(x, new_y)) || new_positions.contains(&(x, new_y)) {
                        positions.remove(&(x, new_y));
                        new_positions.remove(&(x, new_y));

                        for (yy, xx, direction, i) in &new_carts {
                            if x == *xx && new_y == *yy {
                                new_carts.remove(&(new_y, x, *direction, *i));
                                break;
                            }
                        }
                        continue;
                    }
                    positions.remove(&(x, y));
                    new_positions.insert((x, new_y));
                    match grid[new_y][x] {
                        b'|' => {
                            new_carts.insert((new_y, x, direction, i));
                        }
                        b'+' => match i {
                            0 => {
                                new_carts.insert((new_y, x, direction.left(), (i + 1) % 3));
                            }
                            1 => {
                                new_carts.insert((new_y, x, direction, (i + 1) % 3));
                            }
                            2 => {
                                new_carts.insert((new_y, x, direction.right(), (i + 1) % 3));
                            }
                            _ => {
                                unreachable!()
                            }
                        },
                        b'/' => {
                            new_carts.insert((new_y, x, direction.right(), i));
                        }
                        b'\\' => {
                            new_carts.insert((new_y, x, direction.left(), i));
                        }
                        c => bail!("cart ran onto {:?}", c as char),
                    }
                }
                Direction::West => {
                    let new_x = x - 1;
                    if positions.contains(&(new_x, y)) || new_positions.contains(&(new_x, y)) {
                        positions.remove(&(new_x, y));
                        new_positions.remove(&(new_x, y));

                        for (yy, xx, direction, i) in &new_carts {
                            if new_x == *xx && y == *yy {
                                new_carts.remove(&(y, new_x, *direction, *i));
                                break;
                            }
                        }
                        continue;
                    }
                    positions.remove(&(x, y));
                    new_positions.insert((new_x, y));
                    match grid[y][new_x] {
                        b'-' => {
                            new_carts.insert((y, new_x, direction, i));
                        }
                        b'+' => match i {
                            0 => {
                                new_carts.insert((y, new_x, direction.left(), (i + 1) % 3));
                            }
                            1 => {
                                new_carts.insert((y, new_x, direction, (i + 1) % 3));
                            }
                            2 => {
                                new_carts.insert((y, new_x, direction.right(), (i + 1) % 3));
                            }
                            _ => {
                                unreachable!()
                            }
                        },
                        b'/' => {
                            new_carts.insert((y, new_x, direction.left(), i));
                        }
                        b'\\' => {
                            new_carts.insert((y, new_x, direction.right(), i));
                        }
                        c => bail!("cart ran onto {:?}", c as char),
                    }
                }
            }
        }

        positions = new_positions;
        carts = new_carts.clone();
    }
}

/// Solves Year 2018, Day 13: Mine Cart Madness. Both answers are "x,y" positions.
///
/// # Errors
/// Returns an error if the input cannot be read or a cart leaves the track.
pub fn solve() -> Result<(Answer, Answer)> {
    let data = read_input(Year(2018), Day(13))?;
    let (x1, y1) = part_one(&data)?;
    let (x2, y2) = part_two(&data)?;
    Ok((format!("{x1},{y1}").into(), format!("{x2},{y2}").into()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_example;

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test1_part_one() -> Result<()> {
        assert_eq!(part_one(&read_example(Year(2018), Day(13), 1)?)?, (0, 3));
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test2_part_one() -> Result<()> {
        assert_eq!(part_one(&read_example(Year(2018), Day(13), 2)?)?, (7, 3));
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test3_part_two() -> Result<()> {
        assert_eq!(part_two(&read_example(Year(2018), Day(13), 3)?)?, (6, 4));
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_solve() -> Result<()> {
        let (p1, p2) = solve()?;
        assert_eq!(p1, "8,3".into());
        assert_eq!(p2, "73,121".into());
        Ok(())
    }
}
