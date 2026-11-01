use crate::utils::{Answer, Day, Year, read_input};
use anyhow::{Context, Result, ensure};
use std::collections::HashSet;

type IntType = i64;

#[derive(Clone, Debug)]
struct Particle {
    x: IntType,
    y: IntType,
    z: IntType,

    x_velocity: IntType,
    y_velocity: IntType,
    z_velocity: IntType,

    x_acceleration: IntType,
    y_acceleration: IntType,
    z_acceleration: IntType,
}

impl Particle {
    #[allow(clippy::too_many_arguments)]
    fn new(
        x: IntType,
        y: IntType,
        z: IntType,
        x_velocity: IntType,
        y_velocity: IntType,
        z_velocity: IntType,
        x_acceleration: IntType,
        y_acceleration: IntType,
        z_acceleration: IntType,
    ) -> Particle {
        Particle {
            x,
            y,
            z,
            x_velocity,
            y_velocity,
            z_velocity,
            x_acceleration,
            y_acceleration,
            z_acceleration,
        }
    }

    fn parse_triad(triad: &str) -> Result<(IntType, IntType, IntType)> {
        // a=<-2,0,0>
        let tokens = triad
            .replace(['<', '>'], "")
            .split(',')
            .map(|t| t.parse())
            .collect::<Result<Vec<IntType>, _>>()?;
        ensure!(tokens.len() == 3, "expected 3 values in {triad:?}");

        Ok((tokens[0], tokens[1], tokens[2]))
    }

    fn parse(data: &str) -> Result<Particle> {
        // p=<4,0,0>, v=<0,0,0>, a=<-2,0,0>
        let filtered_data = data
            .replace("p=<", "")
            .replace(" v=<", "")
            .replace(" a=<", "");

        let tokens: Vec<_> = filtered_data.split(">,").collect();
        ensure!(tokens.len() == 3, "unexpected particle: {data:?}");

        let p = Particle::parse_triad(tokens[0])?;
        let v = Particle::parse_triad(tokens[1])?;
        let a = Particle::parse_triad(tokens[2])?;

        Ok(Particle::new(p.0, p.1, p.2, v.0, v.1, v.2, a.0, a.1, a.2))
    }

    fn update(&mut self) {
        self.x_velocity += self.x_acceleration;
        self.y_velocity += self.y_acceleration;
        self.z_velocity += self.z_acceleration;
        self.x += self.x_velocity;
        self.y += self.y_velocity;
        self.z += self.z_velocity;
    }

    fn acceleration_weight(&self) -> IntType {
        self.x_acceleration.abs() + self.y_acceleration.abs() + self.z_acceleration.abs()
    }

    fn manhattan_distance(&self, other: &Particle) -> IntType {
        IntType::abs(self.x - other.x)
            + IntType::abs(self.y - other.y)
            + IntType::abs(self.z - other.z)
    }
}

#[derive(Clone, Debug)]
struct ParticleSystem {
    particles: Vec<Particle>,
}

impl ParticleSystem {
    fn parse(data: &str) -> Result<ParticleSystem> {
        let particles = data.lines().map(Particle::parse).collect::<Result<_>>()?;

        Ok(ParticleSystem { particles })
    }

    fn update(&mut self) {
        self.particles.iter_mut().for_each(|p| p.update());
    }

    fn min_particle_acceleration(&self) -> Result<usize> {
        let accelerations: Vec<_> = self
            .particles
            .iter()
            .enumerate()
            .map(|(i, p)| (p.acceleration_weight(), i))
            .collect();

        let min_acceleration = accelerations
            .iter()
            .map(|(a, _)| *a)
            .min()
            .context("no particles")?;

        let mut accelerations: Vec<_> = accelerations
            .iter()
            .filter(|(a, _)| *a == min_acceleration)
            .collect();

        let origin = Particle::new(0, 0, 0, 0, 0, 0, 0, 0, 0);
        accelerations.sort_by_key(|a| self.particles[a.1].manhattan_distance(&origin));
        Ok(accelerations[0].1)
    }

    fn final_particle_count(&mut self) -> usize {
        let mut particle_ids: HashSet<_> = (0..self.particles.len()).collect();

        for _ in 0..250 {
            self.update();

            let ids: Vec<_> = particle_ids.iter().cloned().collect();
            for i in &ids {
                for j in &ids {
                    if i == j {
                        continue;
                    }
                    if self.particles[*i].manhattan_distance(&self.particles[*j]) == 0 {
                        particle_ids.remove(i);
                        particle_ids.remove(j);
                    }
                }
            }
        }
        particle_ids.len()
    }
}

fn part_one(data: &str) -> Result<usize> {
    let particle_system = ParticleSystem::parse(data)?;
    particle_system.min_particle_acceleration()
}

fn part_two(data: &str) -> Result<usize> {
    let mut particle_system = ParticleSystem::parse(data)?;
    Ok(particle_system.final_particle_count())
}

/// Solves Year 2017, Day 20: Particle Swarm.
///
/// # Errors
/// Returns an error if the input cannot be read or parsed.
pub fn solve() -> Result<(Answer, Answer)> {
    let data = read_input(Year(2017), Day(20))?;
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
        assert_eq!(part_one(&read_example(Year(2017), Day(20), 1)?)?, 0);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test1_part_two() -> Result<()> {
        assert_eq!(part_two(&read_example(Year(2017), Day(20), 2)?)?, 1);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_solve() -> Result<()> {
        let (p1, p2) = solve()?;
        assert_eq!(p1, 170.into());
        assert_eq!(p2, 571.into());
        Ok(())
    }
}
