use crate::utils::{Answer, Day, Year, read_input};
use anyhow::{Context, Result, ensure};
use std::collections::HashMap;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Record {
    timestamp: String,
    detail: String,
}

impl Record {
    // [1518-11-01 00:00] Guard #10 begins shift
    fn parse(data: &str) -> Result<Record> {
        let (timestamp, detail) = data
            .split_once("] ")
            .with_context(|| format!("not a record: {data:?}"))?;

        Ok(Record {
            timestamp: timestamp.to_string(),
            detail: detail.to_string(),
        })
    }

    /// Minutes since midnight of "[YYYY-MM-DD HH:MM".
    fn minute_of_day(&self) -> Result<u32> {
        let (_, time) = self
            .timestamp
            .split_once(' ')
            .with_context(|| format!("no time in {:?}", self.timestamp))?;
        let (hour, minute) = time
            .split_once(':')
            .with_context(|| format!("no time in {:?}", self.timestamp))?;
        Ok(hour.parse::<u32>()? * 60 + minute.parse::<u32>()?)
    }
}

/// Guards only sleep during the midnight hour, so a nap is a start minute and a length.
#[derive(Debug)]
struct Sleep {
    start: u32,
    end: u32,
}

fn parse(data: &str) -> Result<Vec<Record>> {
    data.lines().map(Record::parse).collect()
}

fn build_guards(records: &[Record]) -> Result<HashMap<u32, Vec<Sleep>>> {
    let mut guards = HashMap::new();

    let mut curr_guard: Option<u32> = None;
    let mut curr_start: Option<u32> = None;

    for record in records {
        if record.detail.contains("Guard") {
            curr_guard = Some(
                record
                    .detail
                    .replace("Guard #", "")
                    .replace(" begins shift", "")
                    .parse()?,
            );
            curr_start = None;
        } else if record.detail.contains("falls") {
            curr_start = Some(record.minute_of_day()?);
        } else if record.detail.contains("wakes") {
            let end = record.minute_of_day()?;
            let start = curr_start.context("guard wakes without falling asleep")?;
            ensure!(end >= start && end <= 60, "nap outside the midnight hour");
            let sleep = Sleep { start, end };
            guards
                .entry(curr_guard.context("wake-up before any guard's shift")?)
                .or_insert(Vec::new())
                .push(sleep);
            curr_start = None;
        }
    }

    Ok(guards)
}

/// The sleepiest minute of a guard.
fn max_sleep_minute(minutes: &[u32]) -> Result<u32> {
    let max_sleep_by_minute = minutes.iter().max().context("no minutes")?;
    let position = minutes
        .iter()
        .position(|s| s == max_sleep_by_minute)
        .context("no minutes")?;
    Ok(u32::try_from(position)?)
}

fn strategies(data: &str) -> Result<(u32, u32)> {
    let mut records = parse(data)?;
    records.sort();

    let guard_minutes: HashMap<u32, Vec<u32>> = build_guards(&records)?
        .iter()
        .map(|(k, v)| {
            let mut minutes: Vec<u32> = vec![0u32; 60];
            for sleep in v {
                for minute in &mut minutes[sleep.start as usize..sleep.end as usize] {
                    *minute += 1;
                }
            }
            (*k, minutes)
        })
        .collect();

    let mut total_sleep_per_guard: Vec<_> = guard_minutes
        .iter()
        .map(|(k, v)| (v.iter().sum::<u32>(), k))
        .collect();
    total_sleep_per_guard.sort();
    total_sleep_per_guard.reverse();

    let max_sleep_guard = *total_sleep_per_guard.first().context("no guards")?.1;
    let part_one = max_sleep_guard * max_sleep_minute(&guard_minutes[&max_sleep_guard])?;

    let mut max_sleep_per_guard: Vec<_> = guard_minutes
        .iter()
        .map(|(c, v)| (v.iter().max().copied().unwrap_or(0), *c))
        .collect();
    max_sleep_per_guard.sort();
    max_sleep_per_guard.reverse();

    let max_sleep_guard = max_sleep_per_guard.first().context("no guards")?.1;
    let part_two = max_sleep_guard * max_sleep_minute(&guard_minutes[&max_sleep_guard])?;

    Ok((part_one, part_two))
}

/// Solves Year 2018, Day 4: Repose Record.
///
/// # Errors
/// Returns an error if the input cannot be read or parsed.
pub fn solve() -> Result<(Answer, Answer)> {
    let (p1, p2) = strategies(&read_input(Year(2018), Day(4))?)?;
    Ok((u64::from(p1).into(), u64::from(p2).into()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_example;

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_part_one() -> Result<()> {
        let (p1, _) = strategies(&read_example(Year(2018), Day(4), 1)?)?;
        assert_eq!(p1, 240);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_part_two() -> Result<()> {
        let (_, p2) = strategies(&read_example(Year(2018), Day(4), 1)?)?;
        assert_eq!(p2, 4455);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_solve() -> Result<()> {
        let (p1, p2) = solve()?;
        assert_eq!(p1, 84636.into());
        assert_eq!(p2, 91679.into());
        Ok(())
    }
}
