use std::collections::HashMap;

enum Direction {
    Left,
    Right,
}

type Graph<'a> = HashMap<&'a str, (&'a str, &'a str)>;

fn part1(instructions: &[Direction], graph: &Graph) -> i32 {
    // follow instructions repeatedly on graph until you get to ZZZ
    // return the number of instructions followed
    todo!()
}

fn parse_input(puzzle_input: &str) -> (Vec<Direction>, Graph) {
    let mut iter = puzzle_input.lines();
    let instructions = iter.next().unwrap().chars().map(|c| match c {
        'L' => Direction::Left,
        'R' => Direction::Right,
        _ => panic!(
            "Instructions characters must be 'L' or 'R', received: {}",
            c
        ),
    });

    // empty line
    iter.next();

    let mut graph = HashMap::new();
    for line in iter {
        // ex line: "NQT = (TXC, RVJ)"

        // graph.insert(, v)
    }

    (instructions, graph)
}

fn main() {
    let file_contents = std::fs::read("input.txt").unwrap();
    let puzzle_input = std::str::from_utf8(&file_contents).unwrap();

    println!("{}", part1(puzzle_input));
    // println!("{}", part2(puzzle_input));
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
        assert_eq!(part1(TEST_INPUT_A), 2);
    }

    #[test]
    fn test_part1_b() {
        assert_eq!(part1(TEST_INPUT_B), 6);
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(TEST_INPUT), 5905);
    // }
}
