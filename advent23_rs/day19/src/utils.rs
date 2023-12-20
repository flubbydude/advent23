use anyhow::{Context, Result};
use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, ops::Range};

#[derive(Debug, Clone, Copy)]
pub enum Category {
    X,
    M,
    A,
    S,
}

impl From<&str> for Category {
    fn from(value: &str) -> Self {
        match value {
            "x" => Category::X,
            "m" => Category::M,
            "a" => Category::A,
            "s" => Category::S,
            _ => panic!("Cannot convert \"{}\" into Category", value),
        }
    }
}

#[derive(Debug)]
pub struct Part {
    pub x: u64,
    pub m: u64,
    pub a: u64,
    pub s: u64,
}

impl Part {
    pub fn get_category(&self, category: Category) -> u64 {
        match category {
            Category::X => self.x,
            Category::M => self.m,
            Category::A => self.a,
            Category::S => self.s,
        }
    }

    pub fn sum_rating_nums(&self) -> u64 {
        self.x + self.m + self.a + self.s
    }

    pub fn passes_condition(&self, condition: &Condition) -> bool {
        match *condition {
            Condition::LessThan(category, val) => self.get_category(category) < val,
            Condition::GreaterThan(category, val) => self.get_category(category) > val,
            Condition::Always => true,
        }
    }
}

impl TryFrom<&str> for Part {
    type Error = anyhow::Error;

    fn try_from(line: &str) -> Result<Self> {
        lazy_static! {
            static ref PART_REGEX: Regex =
                Regex::new(r"\{x=([0-9]+),m=([0-9]+),a=([0-9]+),s=([0-9]+)\}").unwrap();
        }

        let caps = PART_REGEX
            .captures(line)
            .with_context(|| format!("Unable to parse \"{}\" into Part", line))?;

        let (_, value_strs) = caps.extract();

        let [x, m, a, s] = value_strs.map(|val_str| val_str.parse().unwrap());

        Ok(Part { x, m, a, s })
    }
}

#[derive(Debug)]
pub enum Condition {
    LessThan(Category, u64),
    GreaterThan(Category, u64),
    Always,
}

#[derive(Debug)]
pub enum WorkflowResult {
    Workflow(Box<str>),
    Accept,
    Reject,
}

impl From<&str> for WorkflowResult {
    fn from(value: &str) -> Self {
        match value {
            "A" => WorkflowResult::Accept,
            "R" => WorkflowResult::Reject,
            s => WorkflowResult::Workflow(s.into()),
        }
    }
}

#[derive(Debug)]
pub struct Rule {
    pub condition: Condition,
    pub result: WorkflowResult,
}

impl TryFrom<&str> for Rule {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        lazy_static! {
            static ref RULE_REGEX: Regex =
                Regex::new(r"(?<condition>(?<category>[xmas])(?<type>[<>])(?<value>[0-9]+):)?(?<result>[a-z]+|A|R)").unwrap();
        }

        let caps = RULE_REGEX
            .captures(value)
            .with_context(|| format!("Invalid Rule string: \"{}\"", value))?;

        let condition = if caps.name("condition").is_some() {
            let category = caps["category"].into();
            let value = caps["value"].parse().unwrap();

            if &caps["type"] == "<" {
                Condition::LessThan(category, value)
            } else {
                Condition::GreaterThan(category, value)
            }
        } else {
            Condition::Always
        };

        let result = WorkflowResult::from(&caps["result"]);

        Ok(Rule { condition, result })
    }
}

#[derive(Debug)]
pub struct Workflow {
    pub name: Box<str>,
    pub rules: Box<[Rule]>,
}

impl Workflow {
    pub fn process_part(&self, part: &Part) -> &WorkflowResult {
        for rule in self.rules.iter() {
            if part.passes_condition(&rule.condition) {
                return &rule.result;
            }
        }

        panic!(
            "Workflow does not end in a Rule with condition: Condition::Always\n{:?}",
            self
        );
    }
}

impl TryFrom<&str> for Workflow {
    type Error = anyhow::Error;

    fn try_from(line: &str) -> Result<Self> {
        let (name, rest) = line.split_once('{').with_context(|| {
            format!(
                "When parsing \"{}\" as a Workflow, does not contain a {{",
                line
            )
        })?;

        let rest = rest.strip_suffix('}').with_context(|| {
            format!(
                "When parsing \"{}\" as a Workflow, does not end in }}",
                line
            )
        })?;

        let rules = rest.split(',').map(Rule::try_from).collect::<Result<_>>()?;

        Ok(Workflow {
            name: name.into(),
            rules,
        })
    }
}

pub type WorkflowMap = HashMap<Box<str>, Workflow>;

pub trait WorflowMapExt {
    fn accepts(&self, part: &Part) -> bool;
}

impl WorflowMapExt for WorkflowMap {
    fn accepts(&self, part: &Part) -> bool {
        let mut current_workflow = "in";

        loop {
            match self[current_workflow].process_part(part) {
                WorkflowResult::Workflow(w) => current_workflow = w,
                WorkflowResult::Accept => return true,
                WorkflowResult::Reject => return false,
            }
        }
    }
}

#[derive(Clone)]
pub struct PartRanges {
    pub x: Range<u64>,
    pub m: Range<u64>,
    pub a: Range<u64>,
    pub s: Range<u64>,
}

pub enum PartRangesSplitResult {
    All(PartRanges),
    Some {
        passes: PartRanges,
        fails: PartRanges,
    },
    None(PartRanges),
}

impl PartRanges {
    pub fn get_range(&self, category: Category) -> &Range<u64> {
        match category {
            Category::X => &self.x,
            Category::M => &self.m,
            Category::A => &self.a,
            Category::S => &self.s,
        }
    }

    pub fn get_range_mut(&mut self, category: Category) -> &mut Range<u64> {
        match category {
            Category::X => &mut self.x,
            Category::M => &mut self.m,
            Category::A => &mut self.a,
            Category::S => &mut self.s,
        }
    }

    pub fn num_parts_possible(&self) -> u64 {
        [&self.x, &self.m, &self.a, &self.s]
            .into_iter()
            .map(|range| range.end - range.start)
            .product()
    }

    pub fn split_on_condition(self, condition: &Condition) -> PartRangesSplitResult {
        match *condition {
            Condition::LessThan(category, val) => {
                let range = self.get_range(category);

                if range.end <= val {
                    PartRangesSplitResult::All(self)
                } else if range.start >= val {
                    PartRangesSplitResult::None(self)
                } else {
                    let mut passes = self.clone();
                    passes.get_range_mut(category).end = val;

                    let mut fails = self;
                    fails.get_range_mut(category).start = val;

                    PartRangesSplitResult::Some { passes, fails }
                }
            }
            Condition::GreaterThan(category, val) => {
                let range = self.get_range(category);

                if range.start > val {
                    PartRangesSplitResult::All(self)
                } else if range.end <= val + 1 {
                    PartRangesSplitResult::None(self)
                } else {
                    let mut passes = self.clone();
                    passes.get_range_mut(category).start = val + 1;

                    let mut fails = self;
                    fails.get_range_mut(category).end = val + 1;

                    PartRangesSplitResult::Some { passes, fails }
                }
            }
            Condition::Always => PartRangesSplitResult::All(self),
        }
    }
}
