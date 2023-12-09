use lazy_static::lazy_static;
use priority_queue::PriorityQueue;
use regex::Regex;
use std::collections::HashMap;

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
}

type Graph<'a> = HashMap<&'a str, [&'a str; 2]>;

fn follow_directions(instructions: &[Direction], graph: &Graph, start: &str, goal: &str) -> usize {
    let mut current = start;
    for (i, &instruction) in instructions.iter().cycle().enumerate() {
        if current == goal {
            return i;
        }

        current = graph[current][instruction as usize];
    }

    unreachable!()
}

fn part1(instructions: &[Direction], graph: &Graph) -> usize {
    // follow instructions repeatedly on graph until you get to ZZZ
    // return the number of instructions followed
    follow_directions(instructions, graph, "AAA", "ZZZ")
}

// TODO: i was too high
// should take instructions for each 'A'
// until finding cycle, noting the linear equations
// for turns that 'A' into a 'Z'
// where there a cycle => tells which indices of time
// take 'A' to 'Z' => take LCM of power set of this and choose the min
// power set in this is size 6^6=46656 so not that bad
// power set through iteration and zips? zip crate? maybe
fn part2(instructions: &[Direction], graph: &Graph) -> usize {
    let start_nodes = graph
        .keys()
        .filter(|&node| node.ends_with('A'))
        .cloned()
        .collect::<Vec<_>>();
}

fn parse_input(puzzle_input: &str) -> (Vec<Direction>, Graph) {
    let mut iter = puzzle_input.lines();
    let instructions = iter
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!(
                "Instructions characters must be 'L' or 'R', received: {}",
                c
            ),
        })
        .collect();

    // empty line
    iter.next();

    let mut graph = HashMap::new();
    for line in iter {
        // ex line: "NQT = (TXC, RVJ)"
        lazy_static! {
            static ref LINE_REGEX: Regex = Regex::new(r#"(\w+) = \((\w+), (\w+)\)"#).unwrap();
        }

        let (_, [node, neighbor1, neighbor2]) = LINE_REGEX.captures(line).unwrap().extract();

        graph.insert(node, [neighbor1, neighbor2]);
    }

    (instructions, graph)
}

fn main() {
    let file_contents = std::fs::read("input.txt").unwrap();
    let puzzle_input = std::str::from_utf8(&file_contents).unwrap();

    let (instructions, graph) = parse_input(puzzle_input);

    println!("{}", part1(&instructions, &graph));
    println!("{}", part2(&instructions, &graph));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_A: &str = "RL\n\n\
                                AAA = (BBB, CCC)\n\
                                BBB = (DDD, EEE)\n\
                                CCC = (ZZZ, GGG)\n\
                                DDD = (DDD, DDD)\n\
                                EEE = (EEE, EEE)\n\
                                GGG = (GGG, GGG)\n\
                                ZZZ = (ZZZ, ZZZ)";

    const TEST_INPUT_B: &str = "LLR\n\n\
                                AAA = (BBB, BBB)\n\
                                BBB = (AAA, ZZZ)\n\
                                ZZZ = (ZZZ, ZZZ)";

    const TEST_INPUT_C: &str = "LR\n\n\
                                11A = (11B, XXX)
                                11B = (XXX, 11Z)
                                11Z = (11B, XXX)
                                22A = (22B, XXX)
                                22B = (22C, 22C)
                                22C = (22Z, 22Z)
                                22Z = (22B, 22B)
                                XXX = (XXX, XXX)";

    #[test]
    fn test_part1_a() {
        let (instructions, graph) = parse_input(TEST_INPUT_A);

        assert_eq!(part1(&instructions, &graph), 2);
    }

    #[test]
    fn test_part1_b() {
        let (instructions, graph) = parse_input(TEST_INPUT_B);

        assert_eq!(part1(&instructions, &graph), 6);
    }

    #[test]
    fn test_part2() {
        let (instructions, graph) = parse_input(TEST_INPUT_C);

        assert_eq!(part2(&instructions, &graph), 6);
    }
}
