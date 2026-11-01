use crate::utils::{Answer, Day, Year, read_input};
use anyhow::{Context, Result, bail, ensure};
use std::collections::HashMap;

type IntType = u64;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Category {
    A,
    M,
    S,
    X,
}

impl Category {
    fn parse(name: &str) -> Result<Category> {
        Ok(match name {
            "a" => Category::A,
            "m" => Category::M,
            "s" => Category::S,
            "x" => Category::X,
            _ => bail!("unknown category {name:?}"),
        })
    }
}

#[derive(Debug)]
enum Operation {
    GreaterThan { category: Category, value: IntType },
    LessThan { category: Category, value: IntType },
}

#[derive(Debug)]
struct Rule {
    operation: Operation,
    result: String,
}

impl Rule {
    pub fn parse(data: &str) -> Result<Rule> {
        let (prefix, result) = data
            .split_once(':')
            .with_context(|| format!("not a rule: {data:?}"))?;
        let result = result.to_string();
        if let Some((category, value)) = prefix.split_once('>') {
            let category = Category::parse(category)?;
            let value = value.parse()?;
            let operation = Operation::GreaterThan { category, value };
            Ok(Rule { operation, result })
        } else if let Some((category, value)) = prefix.split_once('<') {
            let category = Category::parse(category)?;
            let value = value.parse()?;
            let operation = Operation::LessThan { category, value };
            Ok(Rule { operation, result })
        } else {
            bail!("rule without a comparison: {data:?}")
        }
    }

    /// Parts always have all four categories: `Part::parse` sets them.
    pub fn evaluate(&self, part: &Part) -> bool {
        match &self.operation {
            Operation::GreaterThan { category, value } => part
                .categories
                .get(category)
                .is_some_and(|other| value < other),
            Operation::LessThan { category, value } => part
                .categories
                .get(category)
                .is_some_and(|other| value > other),
        }
    }
}

#[derive(Debug)]
struct Workflow {
    rules: Vec<Rule>,
    next_workflow: String,
}

impl Workflow {
    pub fn parse(data: &str) -> Result<(String, Workflow)> {
        let data = data.replace('}', "");
        let (name, suffix) = data
            .split_once('{')
            .with_context(|| format!("not a workflow: {data:?}"))?;
        let mut rule_data: Vec<_> = suffix.split(',').collect();
        let next_workflow = rule_data
            .pop()
            .context("workflow without a fallback")?
            .to_string();
        let rules = rule_data
            .iter()
            .map(|rd| Rule::parse(rd))
            .collect::<Result<Vec<_>>>()?;

        Ok((
            name.to_string(),
            Workflow {
                rules,
                next_workflow,
            },
        ))
    }

    pub fn evaluate(&self, part: &Part) -> String {
        for rule in &self.rules {
            if rule.evaluate(part) {
                return rule.result.clone();
            }
        }

        self.next_workflow.clone()
    }
}

#[derive(Debug)]
struct Part {
    categories: HashMap<Category, IntType>,
}

impl Part {
    pub fn parse(data: &str) -> Result<Part> {
        let data = data
            .replace("{x=", "")
            .replace("m=", "")
            .replace("a=", "")
            .replace("s=", "")
            .replace('}', "");

        let tokens: Vec<_> = data.split(',').collect();
        ensure!(tokens.len() == 4, "not a part: {data:?}");

        let categories = HashMap::from([
            (Category::X, tokens[0].parse()?),
            (Category::M, tokens[1].parse()?),
            (Category::A, tokens[2].parse()?),
            (Category::S, tokens[3].parse()?),
        ]);

        Ok(Part { categories })
    }

    pub fn rating(&self) -> IntType {
        self.categories.values().sum()
    }
}

type Workflows = HashMap<String, Workflow>;

fn parse(data: &str) -> Result<(Workflows, Vec<Part>)> {
    let (workflow_data, part_data) = data
        .split_once("\n\n")
        .context("expected workflows, a blank line, then parts")?;

    let workflows = workflow_data
        .lines()
        .map(Workflow::parse)
        .collect::<Result<Workflows>>()?;

    let parts = part_data
        .lines()
        .map(Part::parse)
        .collect::<Result<Vec<_>>>()?;

    Ok((workflows, parts))
}

fn evaluate(workflows: &Workflows, part: &Part) -> Result<bool> {
    let mut state = "in".to_string();

    loop {
        let workflow = workflows
            .get(&state)
            .with_context(|| format!("no workflow {state:?}"))?;
        let new_state = workflow.evaluate(part);
        match new_state.as_str() {
            "A" => return Ok(true),
            "R" => return Ok(false),
            _ => {
                state = new_state;
            }
        }
    }
}

fn part_one(data: &str) -> Result<IntType> {
    let (workflows, parts) = parse(data)?;

    let mut total = 0;
    for part in &parts {
        if evaluate(&workflows, part)? {
            total += part.rating();
        }
    }
    Ok(total)
}

/// Solves Year 2023, Day 19: Aplenty.
///
/// Part two isn't ported: its checked answers come from code that appears
/// adapted from a third-party solution, so it returns 0 until it is rewritten.
///
/// # Errors
/// Returns an error if the input cannot be read or parsed.
pub fn solve() -> Result<(Answer, Answer)> {
    let data = read_input(Year(2023), Day(19))?;
    Ok((part_one(&data)?.into(), 0.into()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_example;

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test1_part_one() -> Result<()> {
        let data = read_example(Year(2023), Day(19), 1)?;
        assert_eq!(part_one(&data)?, 19114);
        Ok(())
    }

    #[test]
    #[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]
    fn test_solve() -> Result<()> {
        let (p1, _) = solve()?;
        assert_eq!(p1, 350678.into());
        Ok(())
    }
}
