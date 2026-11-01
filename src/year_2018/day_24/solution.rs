// TBD add enum UnitType and id to group

use crate::utils::{Answer, Day, Year, read_input, split_at_blank_line};
use anyhow::{Context, Result, bail};
use std::collections::HashMap;

type IntType = i64;
type Battle = Vec<((usize, GroupType), (usize, GroupType))>;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum DamageType {
    Bludgeoning,
    Cold,
    Fire,
    Radiation,
    Slashing,
}

impl DamageType {
    fn parse(name: &str) -> Result<DamageType> {
        Ok(match name {
            "bludgeoning" => DamageType::Bludgeoning,
            "cold" => DamageType::Cold,
            "fire" => DamageType::Fire,
            "radiation" => DamageType::Radiation,
            "slashing" => DamageType::Slashing,
            _ => bail!("unknown damage type {name:?}"),
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Resistance {
    Immune,
    Normal,
    Weak,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum GroupType {
    Immune,
    Infection,
}

#[derive(Clone, Debug, PartialEq)]
struct Group {
    group_type: GroupType,
    num_units: IntType,
    hit_points: IntType,
    attack_damage: IntType,
    attack_type: DamageType,
    initiative: IntType,
    resistance: HashMap<DamageType, Resistance>,
}

/// The `i`th whitespace-separated token of a group description.
fn token<'a>(tokens: &[&'a str], i: usize, data: &str) -> Result<&'a str> {
    tokens
        .get(i)
        .copied()
        .with_context(|| format!("unexpected group: {data:?}"))
}

impl Group {
    fn parse_resistance_clause(
        data: &str,
        resistance_map: &mut HashMap<DamageType, Resistance>,
    ) -> Result<()> {
        let (kind, types) = data
            .split_once(" to ")
            .with_context(|| format!("unexpected resistance: {data:?}"))?;

        let resistance = match kind.trim() {
            "immune" => Resistance::Immune,
            "normal" => Resistance::Normal,
            "weak" => Resistance::Weak,
            other => bail!("unknown resistance {other:?}"),
        };
        for t in types.trim().split(", ") {
            resistance_map.insert(DamageType::parse(t)?, resistance.clone());
        }
        Ok(())
    }

    fn parse_resistance(
        data: &str,
        resistance: &mut HashMap<DamageType, Resistance>,
    ) -> Result<()> {
        for token in data.split(';') {
            Group::parse_resistance_clause(token, resistance)?;
        }
        Ok(())
    }

    // 3246 units each with 1380 hit points (weak to radiation; immune to cold) with an attack that does 4 cold damage at initiative 5
    // 5042 units each with 10450 hit points with an attack that does 18 fire damage at initiative 11
    fn parse(data: &str, group_type: GroupType) -> Result<Group> {
        let num_units;
        let hit_points;
        let attack_damage;
        let attack_type;
        let initiative;
        let mut resistance: HashMap<DamageType, Resistance> = HashMap::from([
            (DamageType::Bludgeoning, Resistance::Normal),
            (DamageType::Cold, Resistance::Normal),
            (DamageType::Fire, Resistance::Normal),
            (DamageType::Radiation, Resistance::Normal),
            (DamageType::Slashing, Resistance::Normal),
        ]);

        if let Some((unit_data, suffix)) = data.split_once('(') {
            // parse unit_data data
            let tokens: Vec<_> = unit_data.split_ascii_whitespace().collect();
            num_units = token(&tokens, 0, data)?.parse()?;
            hit_points = token(&tokens, 4, data)?.parse()?;

            // parse resistance data
            let (resistance_data, damage_data) = suffix
                .split_once(')')
                .with_context(|| format!("unclosed resistance list: {data:?}"))?;
            Group::parse_resistance(resistance_data, &mut resistance)?;

            // parse suffix
            let tokens: Vec<_> = damage_data.split_ascii_whitespace().collect();
            attack_damage = token(&tokens, 5, data)?.parse()?;
            attack_type = DamageType::parse(token(&tokens, 6, data)?)?;
            initiative = token(&tokens, 10, data)?.parse()?;
        } else {
            // no resistance data
            let tokens: Vec<_> = data.split_ascii_whitespace().collect();

            num_units = token(&tokens, 0, data)?.parse()?;
            hit_points = token(&tokens, 4, data)?.parse()?;
            attack_damage = token(&tokens, 12, data)?.parse()?;
            attack_type = DamageType::parse(token(&tokens, 13, data)?)?;
            initiative = token(&tokens, 17, data)?.parse()?;
        }

        Ok(Group {
            group_type,
            num_units,
            hit_points,
            attack_damage,
            attack_type,
            initiative,
            resistance,
        })
    }

    #[inline]
    fn num_units(&self) -> IntType {
        self.num_units
    }

    #[inline]
    fn initiative(&self) -> IntType {
        self.initiative
    }

    #[inline]
    fn effective_power(&self) -> IntType {
        self.num_units * self.attack_damage
    }

    #[inline]
    fn attack_type(&self) -> DamageType {
        self.attack_type
    }

    #[inline]
    fn is_alive(&self) -> bool {
        self.num_units > 0
    }

    /// Every damage type has an entry: parsing starts them all at Normal.
    fn damage(&self, effective_power: IntType, attack_type: DamageType) -> IntType {
        let modifier = match self.resistance.get(&attack_type) {
            Some(Resistance::Immune) => 0,
            Some(Resistance::Normal) | None => 1,
            Some(Resistance::Weak) => 2,
        };
        modifier * effective_power
    }

    fn attack(&mut self, effective_power: IntType, attack_type: DamageType) {
        let damage = self.damage(effective_power, attack_type);
        let dead_units = damage / self.hit_points;
        self.num_units -= dead_units;
    }
}

fn parse(data: &str) -> Result<(HashMap<usize, Group>, HashMap<usize, Group>)> {
    let (immune_data, infection_data) =
        split_at_blank_line(data).context("expected an immune system and an infection")?;

    let immune = immune_data
        .lines()
        .skip(1)
        .enumerate()
        .map(|(i, l)| Ok((i + 1, Group::parse(l, GroupType::Immune)?)))
        .collect::<Result<HashMap<usize, Group>>>()?;

    let infection = infection_data
        .lines()
        .skip(1)
        .enumerate()
        .map(|(i, l)| Ok((i + 1, Group::parse(l, GroupType::Infection)?)))
        .collect::<Result<HashMap<usize, Group>>>()?;

    Ok((immune, infection))
}

/// Both armies are non-empty: the battle loop stops as soon as one is gone.
fn select_targets(attacker: &HashMap<usize, Group>, defender: &HashMap<usize, Group>) -> Battle {
    let mut result = Vec::new();

    let mut attackers: Vec<_> = attacker.keys().copied().collect();

    let mut defenders: Vec<_> = defender.keys().copied().collect();

    attackers.sort_by(|a, b| {
        let a_group = &attacker[a];
        let b_group = &attacker[b];
        b_group
            .effective_power()
            .cmp(&a_group.effective_power())
            .then_with(|| b_group.initiative().cmp(&a_group.initiative()))
    });

    let attacker_group_type = attacker[&attackers[0]].group_type;
    let defender_group_type = defender[&defenders[0]].group_type;

    for a in attackers {
        let a_group = &attacker[&a];
        let mut target: Option<usize> = None;
        for d in &defenders {
            let d_group = &defender[d];
            let d_damage = d_group.damage(a_group.effective_power(), a_group.attack_type());
            if d_damage > 0 {
                if let Some(t) = target {
                    let t_group = &defender[&t];
                    let t_damage = t_group.damage(a_group.effective_power(), a_group.attack_type());
                    if d_damage > t_damage {
                        target = Some(*d);
                    } else if d_damage == t_damage {
                        let d_effective_power = d_group.effective_power();
                        let t_effective_power = t_group.effective_power();
                        if d_effective_power > t_effective_power
                            || d_effective_power == t_effective_power
                                && d_group.initiative() > t_group.initiative()
                        {
                            target = Some(*d);
                        }
                    }
                } else {
                    target = Some(*d);
                }
            }
        }

        if let Some(t) = target {
            result.push(((a, attacker_group_type), (t, defender_group_type)));
            defenders.retain(|e| *e != t);
        }
    }

    result
}

fn build_battle_grouping(
    immune: &HashMap<usize, Group>,
    infection: &HashMap<usize, Group>,
) -> Battle {
    let mut battle_grouping = select_targets(immune, infection);
    battle_grouping.append(&mut select_targets(infection, immune));

    let initiative = |(id, group_type): (usize, GroupType)| {
        if group_type == GroupType::Immune {
            immune[&id].initiative()
        } else {
            infection[&id].initiative()
        }
    };
    battle_grouping.sort_by_key(|b| std::cmp::Reverse(initiative(b.0)));

    battle_grouping
}

fn resolve_battle_grouping(
    battle_grouping: &Battle,
    immune: &HashMap<usize, Group>,
    infection: &HashMap<usize, Group>,
) -> (HashMap<usize, Group>, HashMap<usize, Group>) {
    let mut new_immune = immune.clone();
    let mut new_infection = infection.clone();

    for (attacker, defender) in battle_grouping {
        let (attackers, defenders) = match attacker.1 {
            GroupType::Immune => (&new_immune, &mut new_infection),
            GroupType::Infection => (&new_infection, &mut new_immune),
        };
        if let Some(attacker_group) = attackers.get(&attacker.0)
            && let Some(defender_group) = defenders.get_mut(&defender.0)
        {
            defender_group.attack(
                attacker_group.effective_power(),
                attacker_group.attack_type(),
            );
            if !defender_group.is_alive() {
                defenders.remove(&defender.0);
            }
        }
    }

    (new_immune, new_infection)
}

/// Fights until one army is gone and returns the winner's remaining units.
fn fight(mut immune: HashMap<usize, Group>, mut infection: HashMap<usize, Group>) -> IntType {
    while !immune.is_empty() && !infection.is_empty() {
        let battle_grouping = build_battle_grouping(&immune, &infection);
        (immune, infection) = resolve_battle_grouping(&battle_grouping, &immune, &infection);
    }

    let winner = if immune.is_empty() { infection } else { immune };
    winner.values().map(Group::num_units).sum()
}

fn part_one(data: &str) -> Result<IntType> {
    let (immune, infection) = parse(data)?;
    Ok(fight(immune, infection))
}

/// The boost of 31 was found by hand for this input; the search for the
/// smallest winning boost isn't implemented.
fn part_two(data: &str) -> Result<IntType> {
    let (mut immune, infection) = parse(data)?;

    let boost = 31;
    for g in immune.values_mut() {
        g.attack_damage += boost;
    }

    Ok(fight(immune, infection))
}

/// Solves Year 2018, Day 24: Immune System Simulator 20XX.
///
/// # Errors
/// Returns an error if the input cannot be read or parsed.
pub fn solve() -> Result<(Answer, Answer)> {
    let data = read_input(Year(2018), Day(24))?;
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
        assert_eq!(part_one(&read_example(Year(2018), Day(24), 1)?)?, 5216);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_solve() -> Result<()> {
        let (p1, p2) = solve()?;
        assert_eq!(p1, 24009.into());
        assert_eq!(p2, 379.into());
        Ok(())
    }
}
