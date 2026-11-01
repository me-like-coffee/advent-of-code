use crate::utils::{Answer, Day, Year, read_input};
use anyhow::{Result, ensure};
use num::integer::Integer;

type IntType = i64;
type Axis = [IntType; 8];

#[derive(Debug)]
struct Moon {
    position: [IntType; 3],
    velocity: [IntType; 3],
}

impl Moon {
    // <x=4, y=-8, z=8>
    fn parse(data: &str) -> Result<Moon> {
        let data = data
            .replace("<x=", "")
            .replace("y=", "")
            .replace("z=", "")
            .replace('>', "");

        let tokens: Vec<_> = data.split(", ").collect();
        ensure!(tokens.len() == 3, "not a moon: {data:?}");

        let x = tokens[0].parse()?;
        let y = tokens[1].parse()?;
        let z = tokens[2].parse()?;

        Ok(Moon {
            position: [x, y, z],
            velocity: [0, 0, 0],
        })
    }
}

fn get_gravity(moons: &[Moon], i: usize) -> [IntType; 3] {
    let mut gravity = [0; 3];

    for (j, other) in moons.iter().enumerate() {
        if i != j {
            let axes = other.position.iter().zip(&moons[i].position);
            for (g, (o, m)) in gravity.iter_mut().zip(axes) {
                *g += (o - m).signum();
            }
        }
    }

    gravity
}

fn apply_gravity(moons: &mut [Moon], gravity: &[[IntType; 3]]) {
    for (moon, pull) in moons.iter_mut().zip(gravity) {
        let axes = moon.velocity.iter_mut().zip(moon.position.iter_mut());
        for ((v, p), g) in axes.zip(pull) {
            *v += g;
            *p += *v;
        }
    }
}

fn parse(data: &str) -> Result<Vec<Moon>> {
    data.lines().map(Moon::parse).collect()
}

fn part_one(data: &str, num_steps: usize) -> Result<IntType> {
    let mut moons = parse(data)?;

    for _ in 0..num_steps {
        let gravity: Vec<_> = (0..moons.len()).map(|i| get_gravity(&moons, i)).collect();

        apply_gravity(&mut moons, &gravity);
    }

    let mut total_energy = 0;
    for moon in moons {
        let potential_energy: IntType = moon.position.iter().map(|p| p.abs()).sum();
        let kinetic_energy: IntType = moon.velocity.iter().map(|p| p.abs()).sum();
        total_energy += potential_energy * kinetic_energy;
    }

    Ok(total_energy)
}

fn step(axis: Axis) -> Axis {
    let [p0, p1, p2, p3, v0, v1, v2, v3] = axis;

    let n0 = v0 + (p1 - p0).signum() + (p2 - p0).signum() + (p3 - p0).signum();
    let n1 = v1 + (p0 - p1).signum() + (p2 - p1).signum() + (p3 - p1).signum();
    let n2 = v2 + (p0 - p2).signum() + (p1 - p2).signum() + (p3 - p2).signum();
    let n3 = v3 + (p0 - p3).signum() + (p1 - p3).signum() + (p2 - p3).signum();

    [p0 + n0, p1 + n1, p2 + n2, p3 + n3, n0, n1, n2, n3]
}

fn stopped(axis: Axis) -> bool {
    axis[4] == 0 && axis[5] == 0 && axis[6] == 0 && axis[7] == 0
}

/// Each axis is independent: find when each first comes to rest, which is
/// half its period, and combine them.
fn part_two(data: &str) -> Result<IntType> {
    let moons = parse(data)?;
    ensure!(moons.len() == 4, "expected 4 moons, found {}", moons.len());

    let (mut x, mut y, mut z) = ([0; 8], [0; 8], [0; 8]);
    for (i, moon) in moons.iter().enumerate() {
        x[i] = moon.position[0];
        x[i + 4] = moon.velocity[0];
        y[i] = moon.position[1];
        y[i + 4] = moon.velocity[1];
        z[i] = moon.position[2];
        z[i + 4] = moon.velocity[2];
    }

    let [mut a, mut b, mut c] = [0i64, 0i64, 0i64];
    let mut count = 0;

    while a == 0 || b == 0 || c == 0 {
        count += 1;

        if a == 0 {
            x = step(x);
            if stopped(x) {
                a = count;
            }
        }

        if b == 0 {
            y = step(y);
            if stopped(y) {
                b = count;
            }
        }

        if c == 0 {
            z = step(z);
            if stopped(z) {
                c = count;
            }
        }
    }

    Ok(2 * a.lcm(&b.lcm(&c)))
}

/// Solves Year 2019, Day 12: The N-Body Problem.
///
/// Part two isn't confirmed for this input: the original's check for it was
/// commented out. The examples confirm the method.
///
/// # Errors
/// Returns an error if the input cannot be read or parsed.
pub fn solve() -> Result<(Answer, Answer)> {
    let data = read_input(Year(2019), Day(12))?;
    Ok((
        u64::try_from(part_one(&data, 1000)?)?.into(),
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
        assert_eq!(part_one(&read_example(Year(2019), Day(12), 1)?, 10)?, 179);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test2_part_one() -> Result<()> {
        assert_eq!(part_one(&read_example(Year(2019), Day(12), 2)?, 100)?, 1940);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test1_part_two() -> Result<()> {
        assert_eq!(part_two(&read_example(Year(2019), Day(12), 1)?)?, 2772);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test2_part_two() -> Result<()> {
        assert_eq!(
            part_two(&read_example(Year(2019), Day(12), 2)?)?,
            4686774924
        );
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_solve() -> Result<()> {
        let (p1, _) = solve()?;
        assert_eq!(p1, 8538.into());
        Ok(())
    }
}
