use anyhow::{Context, Result};
use lazy_static::lazy_static;
use regex::Regex;
use std::fs;

enum Category {
    X,
    M,
    A,
    S,
}

struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

enum Condition {
    LessThan(Category, u32),
    GreaterThan(Category, u32),
    Always,
}

enum RuleResult {
    Workflow(Box<str>),
    Accept,
    Reject,
}

struct Rule {
    condition: Condition,
    result: RuleResult,
}

impl TryFrom<&str> for Rule {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        lazy_static! {
            static ref RULE_REGEX: Regex =
                Regex::new(r"(([xmas])([<>])([0-9]+):)?([a-z]+|A|R)").unwrap();
        }
    }
}

struct Workflow {
    name: Box<str>,
    rules: Vec<Rule>,
}

impl TryFrom<&str> for Workflow {
    type Error = anyhow::Error;

    fn try_from(line: &str) -> Result<Self> {
        let (name, rest) = line
            .split_once('{')
            .context("Workflow line does not contain a {")?;

        let rest = rest
            .strip_suffix('}')
            .context("Workflow line does not end in }")?;

        let rules = rest.split(',').map(Rule::try_from).collect::<Result<_>>()?;

        Ok(Workflow {
            name: name.to_string().into_boxed_str(),
            rules,
        })
    }
}

fn main() -> Result<()> {
    let file_contents = fs::read_to_string("input.txt")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "";

    #[test]
    fn test_part1() -> Result<()> {
        Ok(())
    }
}
