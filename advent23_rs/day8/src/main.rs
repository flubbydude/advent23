use lazy_static::lazy_static;
use priority_queue::PriorityQueue;
use regex::Regex;
use std::collections::HashMap;

enum Direction {
    Left,
    Right,
}

type Graph<'a> = HashMap<&'a str, [&'a str; 2]>;

fn part1(instructions: &[Direction], graph: &Graph) -> usize {
    // follow instructions repeatedly on graph until you get to ZZZ
    // return the number of instructions followed
    let mut current = "AAA";
    for (i, instruction) in instructions.iter().cycle().enumerate() {
        if current == "ZZZ" {
            return i;
        }

        current = graph[current][*instruction as usize];
    }

    unreachable!()
}

trait GraphExt {
    fn djikstras(&self, start: &str) -> HashMap<&str, i32>;
    fn all_source_djikstras(&self) -> HashMap<&str, HashMap<&str, i32>>;
}

impl GraphExt for Graph<'_> {
    fn djikstras(&self, start: &str) -> HashMap<&str, i32> {
        let mut queue = PriorityQueue::new();
        let mut dists = HashMap::new();
        for &node in self.keys() {
            dists.insert(node, if node == start { 0 } else { i32::MAX });
            queue.push(node, if node == start { 0 } else { i32::MAX });
        }

        while !queue.is_empty() {
            let (node, _) = queue.pop().unwrap();

            for &neighbor in self.get(node).unwrap() {
                let alt = dists[node] + 1;
                if alt < dists[neighbor] {
                    dists[neighbor] = alt;
                    queue.push_decrease(neighbor, alt);
                }
            }
        }

        dists
    }

    fn all_source_djikstras(&self) -> HashMap<&str, HashMap<&str, i32>> {
        self.keys()
            .map(|&node| (node, self.djikstras(node)))
            .collect()
    }
}

fn part2(graph: &Graph) -> i32 {
    let ends_in_a = graph
        .keys()
        .filter(|&node| node.ends_with('a'))
        .cloned()
        .collect::<Vec<_>>();

    let ends_in_z = graph
        .keys()
        .filter(|&node| node.ends_with('z'))
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
    println!("{}", part2(&graph));
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

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(TEST_INPUT), 5905);
    // }
}
