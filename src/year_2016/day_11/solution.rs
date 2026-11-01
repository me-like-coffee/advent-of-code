// TBD Cache invalid states and remove valid state check

use crate::utils::{Answer, Day, Year, read_input};
use anyhow::{Context, Result, bail, ensure};
use std::collections::{HashSet, VecDeque};

/// Room for the elevator and seven elements: five in the input, two more in part two.
const ELEMENTS: usize = 7;
const SLOTS: usize = 1 + 2 * ELEMENTS;

type Floor = [bool; SLOTS];
type Building = [Floor; 4];
type States = HashSet<Building>;
type NextStates = VecDeque<Building>;

const FLOOR_ONE: usize = 0;
const FLOOR_TWO: usize = 1;
const FLOOR_THREE: usize = 2;
const FLOOR_FOUR: usize = 3;

const E: usize = 0;

/// Slot of element `k`'s generator; its microchip is the next slot.
const fn generator(k: usize) -> usize {
    2 * k + 1
}

const fn microchip(k: usize) -> usize {
    2 * k + 2
}

/// Parses the four floor descriptions. Elements are numbered in order of first appearance.
fn parse(data: &str) -> Result<(Building, usize)> {
    let lines: Vec<&str> = data.lines().collect();
    ensure!(lines.len() == 4, "expected 4 floors, found {}", lines.len());

    let mut building = [[false; SLOTS]; 4];
    building[FLOOR_ONE][E] = true;
    let mut elements: Vec<&str> = Vec::new();

    for (floor, line) in lines.iter().enumerate() {
        let words: Vec<&str> = line.split_whitespace().collect();
        for pair in words.windows(2) {
            let (name, is_generator) = if pair[1].starts_with("generator") {
                (pair[0], true)
            } else if pair[1].starts_with("microchip") {
                let name = pair[0]
                    .strip_suffix("-compatible")
                    .with_context(|| format!("unexpected microchip name: {}", pair[0]))?;
                (name, false)
            } else {
                continue;
            };
            let k = match elements.iter().position(|e| *e == name) {
                Some(k) => k,
                None => {
                    elements.push(name);
                    elements.len() - 1
                }
            };
            ensure!(k + 2 < ELEMENTS, "more than {} elements", ELEMENTS - 2);
            let slot = if is_generator {
                generator(k)
            } else {
                microchip(k)
            };
            building[floor][slot] = true;
        }
    }

    Ok((building, elements.len()))
}

fn goal_state_one(floor: &Floor, elements: usize) -> bool {
    let true_count = floor.iter().filter(|s| **s).count();

    true_count == 1 + 2 * elements
}

/// A microchip is fried if it shares a floor with another element's generator
/// and its own generator isn't there.
fn valid_floor(floor: &Floor) -> bool {
    let true_count = floor.iter().filter(|s| **s).count();
    if floor[E] && true_count == 1 {
        return false;
    }

    for g in 0..ELEMENTS {
        if floor[generator(g)]
            && (0..ELEMENTS).any(|m| m != g && !floor[generator(m)] && floor[microchip(m)])
        {
            return false;
        }
    }

    true
}

fn valid_state(building: &Building) -> bool {
    building.iter().all(valid_floor)
}

fn move_to_floor(
    elevator_floor: usize,
    new_floor: usize,
    state: &Building,
    states: &mut States,
    q: &mut NextStates,
) {
    let elevator_floor_objects: Vec<_> = state[elevator_floor]
        .iter()
        .enumerate()
        .filter(|(i, v)| *i != E && **v)
        .map(|(i, _)| i)
        .collect();

    for &i in &elevator_floor_objects {
        let mut new_state = *state;
        new_state[elevator_floor][i] = false;
        new_state[new_floor][i] = true;
        new_state[elevator_floor][E] = false;
        new_state[new_floor][E] = true;
        if valid_state(&new_state) && !states.contains(&new_state) {
            states.insert(new_state);
            q.push_back(new_state);
        }
    }

    // Every ordered pair of distinct objects.
    for (a, &first) in elevator_floor_objects.iter().enumerate() {
        for (b, &second) in elevator_floor_objects.iter().enumerate() {
            if a == b {
                continue;
            }
            let mut new_state = *state;
            for i in [first, second] {
                new_state[elevator_floor][i] = false;
                new_state[new_floor][i] = true;
            }
            new_state[elevator_floor][E] = false;
            new_state[new_floor][E] = true;

            if valid_state(&new_state) && !states.contains(&new_state) {
                states.insert(new_state);
                q.push_back(new_state);
            }
        }
    }
}

fn part_one(building: Building, elements: usize) -> Result<u64> {
    let mut states: HashSet<Building> = HashSet::new();
    let mut q: VecDeque<Building> = VecDeque::new();
    let mut steps = -1;

    states.insert(building);
    q.push_back(building);

    while !q.is_empty() {
        steps += 1;
        for _ in 0..q.len() {
            let Some(state) = q.pop_front() else {
                break;
            };

            if goal_state_one(&state[FLOOR_FOUR], elements) {
                return Ok(u64::try_from(steps)?);
            }

            let elevator_floor = state
                .iter()
                .position(|floor| floor[E])
                .context("no floor has the elevator")?;

            match elevator_floor {
                FLOOR_FOUR => {
                    // move to floor three
                    move_to_floor(FLOOR_FOUR, FLOOR_THREE, &state, &mut states, &mut q);
                }
                FLOOR_THREE => {
                    // move to floor four
                    move_to_floor(FLOOR_THREE, FLOOR_FOUR, &state, &mut states, &mut q);

                    // move to floor two
                    move_to_floor(FLOOR_THREE, FLOOR_TWO, &state, &mut states, &mut q);
                }
                FLOOR_TWO => {
                    // move to floor three
                    move_to_floor(FLOOR_TWO, FLOOR_THREE, &state, &mut states, &mut q);

                    // move to floor one
                    move_to_floor(FLOOR_TWO, FLOOR_ONE, &state, &mut states, &mut q);
                }
                FLOOR_ONE => {
                    // move to floor two
                    move_to_floor(FLOOR_ONE, FLOOR_TWO, &state, &mut states, &mut q);
                }
                _ => {
                    unreachable!()
                }
            }
        }
    }
    bail!("no sequence of moves reaches the goal")
}

fn goal_state_two(floor: &Floor, elements: usize) -> bool {
    floor.iter().filter(|s| **s).count() == 1 + 2 * (elements + 2)
}

/// Part two adds two elements, each with a generator and a microchip, on the first floor.
fn part_two(building: Building, elements: usize) -> Result<u64> {
    let mut building = building;
    for k in [elements, elements + 1] {
        building[FLOOR_ONE][generator(k)] = true;
        building[FLOOR_ONE][microchip(k)] = true;
    }

    let mut states: HashSet<Building> = HashSet::new();
    let mut q: VecDeque<Building> = VecDeque::new();
    let mut steps = -1;

    states.insert(building);
    q.push_back(building);

    while !q.is_empty() {
        steps += 1;
        for _ in 0..q.len() {
            let Some(state) = q.pop_front() else {
                break;
            };

            if goal_state_two(&state[FLOOR_FOUR], elements) {
                return Ok(u64::try_from(steps)?);
            }

            let elevator_floor = state
                .iter()
                .position(|floor| floor[E])
                .context("no floor has the elevator")?;

            match elevator_floor {
                FLOOR_FOUR => {
                    // move to floor three
                    move_to_floor(FLOOR_FOUR, FLOOR_THREE, &state, &mut states, &mut q);
                }
                FLOOR_THREE => {
                    // move to floor four
                    move_to_floor(FLOOR_THREE, FLOOR_FOUR, &state, &mut states, &mut q);

                    // move to floor two
                    move_to_floor(FLOOR_THREE, FLOOR_TWO, &state, &mut states, &mut q);
                }
                FLOOR_TWO => {
                    // move to floor three
                    move_to_floor(FLOOR_TWO, FLOOR_THREE, &state, &mut states, &mut q);

                    // move to floor one
                    move_to_floor(FLOOR_TWO, FLOOR_ONE, &state, &mut states, &mut q);
                }
                FLOOR_ONE => {
                    // move to floor two
                    move_to_floor(FLOOR_ONE, FLOOR_TWO, &state, &mut states, &mut q);
                }
                _ => {
                    unreachable!()
                }
            }
        }
    }
    bail!("no sequence of moves reaches the goal")
}

/// Solves Year 2016, Day 11: Radioisotope Thermoelectric Generators.
///
/// # Errors
/// Returns an error if the input cannot be read or parsed, or has no solution.
pub fn solve() -> Result<(Answer, Answer)> {
    let data = read_input(Year(2016), Day(11))?;
    let (building, elements) = parse(&data)?;
    Ok((
        part_one(building, elements)?.into(),
        part_two(building, elements)?.into(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_example;

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test1_part_one() -> Result<()> {
        let (building, elements) = parse(&read_example(Year(2016), Day(11), 1)?)?;
        assert_eq!(part_one(building, elements)?, 11);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_solve() -> Result<()> {
        let (p1, p2) = solve()?;
        assert_eq!(p1, 33.into());
        assert_eq!(p2, 57.into());
        Ok(())
    }
}
