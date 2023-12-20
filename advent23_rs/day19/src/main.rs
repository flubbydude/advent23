use anyhow::Result;
use std::{collections::HashMap, fs};

mod utils;

use utils::*;

fn parse_input(input: &str) -> Result<(WorkflowMap, Vec<Part>)> {
    let workflow_map = input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| Workflow::try_from(line).map(|workflow| (workflow.name.clone(), workflow)))
        .collect::<Result<HashMap<_, _>>>()?;

    let mut parts = input
        .lines()
        .rev()
        .take_while(|line| !line.is_empty())
        .map(Part::try_from)
        .collect::<Result<Vec<_>>>()?;

    parts.reverse();

    Ok((workflow_map, parts))
}

fn part1(workflow_map: &WorkflowMap, parts: &[Part]) -> u64 {
    parts
        .iter()
        .map(|part| {
            if workflow_map.accepts(part) {
                part.sum_rating_nums()
            } else {
                0
            }
        })
        .sum()
}

fn main() -> Result<()> {
    let file_contents = fs::read_to_string("input.txt")?;

    let (workflow_map, parts) = parse_input(&file_contents)?;

    println!("{}", part1(&workflow_map, &parts));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "px{a<2006:qkq,m>2090:A,rfg}\n\
                              pv{a>1716:R,A}\n\
                              lnx{m>1548:A,A}\n\
                              rfg{s<537:gd,x>2440:R,A}\n\
                              qs{s>3448:A,lnx}\n\
                              qkq{x<1416:A,crn}\n\
                              crn{x>2662:A,R}\n\
                              in{s<1351:px,qqz}\n\
                              qqz{s>2770:qs,m<1801:hdj,R}\n\
                              gd{a>3333:R,R}\n\
                              hdj{m>838:A,pv}\n\n\
                              {x=787,m=2655,a=1222,s=2876}\n\
                              {x=1679,m=44,a=2067,s=496}\n\
                              {x=2036,m=264,a=79,s=2244}\n\
                              {x=2461,m=1339,a=466,s=291}\n\
                              {x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn test_part1() -> Result<()> {
        let (workflow_map, parts) = parse_input(TEST_INPUT)?;

        assert_eq!(part1(&workflow_map, &parts), 19114);

        Ok(())
    }
}
