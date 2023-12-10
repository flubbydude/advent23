use lazy_static::lazy_static;
use num::integer::lcm;
use regex::Regex;
use std::collections::hash_map::Entry;
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

#[derive(Debug)]
struct GhostPathInfo {
    time_steps_at_z: Vec<usize>,
    cycle_start_index: usize, // index of time_steps where the first Z appears in a cycle
    cycle_length: usize,      // in terms of time steps.
}

// returns the vector of time steps of when you find a Z before the cycle
// the index of z_indices where you get to the first Z on the cycle
// and the cycle length
fn follow_directions_part2(
    instructions: &[Direction],
    graph: &Graph,
    start: &str,
) -> GhostPathInfo {
    let mut current = start;

    // state = (index of instructions)
    // this should find
    // find cycle and when it reaches cycle
    let mut seen: HashMap<(usize, &str), usize> = HashMap::new();
    // let cycle_start_state;
    let mut cycle_start_time_step = 0;
    let mut cycle_length = 0;

    let mut time_steps_at_z = Vec::new();

    for (i, &instruction) in instructions.iter().cycle().enumerate() {
        if current.ends_with('Z') {
            time_steps_at_z.push(i);
        }

        // been on this instruction at this time before
        let state = (i % instructions.len(), current);
        match seen.entry(state) {
            Entry::Occupied(e) => {
                // cycle_start_state = state;
                cycle_start_time_step = *e.get();
                cycle_length = i - cycle_start_time_step;
                break;
            }
            Entry::Vacant(e) => {
                e.insert(i);
            }
        }
        current = graph[current][instruction as usize];
    }

    // the first index where the cycle starts is always after the
    let cycle_start_index = time_steps_at_z
        .binary_search(&cycle_start_time_step)
        .unwrap_or_else(|i| i);

    // yuh
    if cycle_start_index == time_steps_at_z.len() {
        eprintln!("No Zs in a cycle, that's odd lol");
    }

    GhostPathInfo {
        time_steps_at_z,
        cycle_start_index,
        cycle_length,
    }
}

fn part2(instructions: &[Direction], graph: &Graph) -> usize {
    let start_nodes = graph
        .keys()
        .filter(|&node| node.ends_with('A'))
        .cloned()
        .collect::<Vec<_>>();

    let ghost_path_infos = start_nodes
        .iter()
        .map(|start| follow_directions_part2(instructions, graph, start))
        .collect::<Vec<_>>();

    println!("{:?}", ghost_path_infos);

    // OKAY bruh they each only go to 1 Z repeatedly in the real data so as a
    // shortcut we simply get the LCM of all the dists
    // otherwise it would have been way harder mafs lol

    // we have this shortcut b/c the printout basically before
    // resorting to the cycles we have to reach all the cycles
    // checking if all are on Z until time step maximum cycle start index

    // then solve a real large system of linear equations lol
    // up to (num As)**(num As) times (once for every possible appearance of Z for every A)
    ghost_path_infos
        .iter()
        .map(|gpi| gpi.cycle_length)
        .reduce(lcm)
        .unwrap()
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
