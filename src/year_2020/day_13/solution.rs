use crate::utils::{Answer, Day, Year, read_input};
use anyhow::Result;

/// Solves Year 2020, Day 13: Shuttle Search.
///
/// # Errors
/// Returns an error if the input cannot be read or parsed.
pub fn solve() -> Result<(Answer, Answer)> {
    let content = read_input(Year(2020), Day(13))?;
    let (p1, p2) = calculate_solution(&content)?;
    Ok((p1.into(), p2.into()))
}

fn calculate_solution(input: &str) -> Result<(u64, u64)> {
    let mut lines = input.lines();
    let earliest_time: u64 = lines.next().unwrap_or("0").parse()?;
    let bus_ids_raw = lines.next().unwrap_or("");

    let part1 = solve_part1(earliest_time, bus_ids_raw);
    let part2 = solve_part2(bus_ids_raw);

    Ok((part1, part2))
}

fn solve_part1(earliest_time: u64, bus_ids_raw: &str) -> u64 {
    let bus_ids: Vec<u64> = bus_ids_raw
        .split(',')
        .filter(|&x| x != "x")
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let mut min_wait = u64::MAX;
    let mut best_bus = 0;

    for &id in &bus_ids {
        let wait_time = if earliest_time.is_multiple_of(id) {
            0
        } else {
            id - (earliest_time % id)
        };

        if wait_time < min_wait {
            min_wait = wait_time;
            best_bus = id;
        }
    }

    best_bus * min_wait
}

fn solve_part2(bus_ids_raw: &str) -> u64 {
    let buses: Vec<(u64, u64)> = bus_ids_raw
        .split(',')
        .enumerate()
        .filter(|&(_, x)| x != "x")
        .map(|(i, x)| (i as u64, x.parse::<u64>().unwrap()))
        .collect();

    let mut t = 0;
    let mut step = 1;

    for (offset, id) in buses {
        // Find the first t' of the form t + n * step such that (t' + offset) % id == 0
        while (t + offset) % id != 0 {
            t += step;
        }
        // Update step to be the LCM of current step and the new bus ID.
        // Since bus IDs are prime, we just multiply.
        step *= id;
    }

    t
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_example;

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_example_part1() -> Result<()> {
        let input = &read_example(Year(2020), Day(13), 1)?;
        let (p1, _) = calculate_solution(input).unwrap();
        assert_eq!(p1, 295);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_example_part2_1() -> Result<()> {
        assert_eq!(solve_part2(&read_example(Year(2020), Day(13), 2)?), 1068781);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_example_part2_2() -> Result<()> {
        assert_eq!(solve_part2(&read_example(Year(2020), Day(13), 3)?), 3417);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_example_part2_3() -> Result<()> {
        assert_eq!(solve_part2(&read_example(Year(2020), Day(13), 4)?), 754018);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_example_part2_4() -> Result<()> {
        assert_eq!(solve_part2(&read_example(Year(2020), Day(13), 5)?), 779210);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_example_part2_5() -> Result<()> {
        assert_eq!(solve_part2(&read_example(Year(2020), Day(13), 6)?), 1261476);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_example_part2_6() -> Result<()> {
        assert_eq!(
            solve_part2(&read_example(Year(2020), Day(13), 7)?),
            1202161486
        );
        Ok(())
    }
}
