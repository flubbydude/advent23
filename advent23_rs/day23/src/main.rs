use std::{
    collections::{HashMap, HashSet},
    fs,
};

use array2d::Array2D;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn is_opposite(&self, other: &Self) -> bool {
        use Direction::*;
        let oppo = match self {
            North => South,
            East => West,
            South => North,
            West => East,
        };
        oppo == *other
    }
}

#[derive(Clone, Copy)]
enum Tile {
    Path,
    Forest,
    Slope(Direction),
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::Path,
            '#' => Tile::Forest,
            '^' => Tile::Slope(Direction::North),
            '>' => Tile::Slope(Direction::East),
            'v' => Tile::Slope(Direction::South),
            '<' => Tile::Slope(Direction::West),
            _ => panic!("Cannot convert '{value}' into Tile"),
        }
    }
}

fn parse_input(input: &str) -> Array2D<Tile> {
    let num_columns = input.lines().next().unwrap().len();
    let num_rows = input.lines().count();
    Array2D::from_iter_row_major(
        input.lines().flat_map(|line| line.chars().map(Tile::from)),
        num_rows,
        num_columns,
    )
    .unwrap()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    direction: Direction,
    position: (usize, usize),
}

trait GridExt {
    fn get_successors(&self, state: &State) -> Vec<State>;
    fn reachable_edges(&self) -> Vec<(State, State)>;
    fn longest_path(&self) -> usize;
}

const START_POS: (usize, usize) = (0, 1);

impl GridExt for Array2D<Tile> {
    fn get_successors(&self, state: &State) -> Vec<State> {
        let try_direction = |direction_to_try: Direction| {
            if direction_to_try.is_opposite(&state.direction) {
                return None;
            }

            let (row, column) = state.position;

            // only check out of bounds on top and bottom
            if row == 0 {
                if matches!(state.direction, Direction::South) {
                    return Some((1, column));
                } else {
                    return None;
                }
            }

            if row == self.num_rows() - 1 {
                return None;
            }

            let new_position = match direction_to_try {
                Direction::North => (row - 1, column),
                Direction::East => (row, column + 1),
                Direction::South => (row + 1, column),
                Direction::West => (row, column - 1),
            };

            if matches!(self[new_position], Tile::Forest) {
                None
            } else {
                Some(new_position)
            }
        };

        match self[state.position] {
            Tile::Path => Direction::iter()
                .filter_map(|direction_to_try| {
                    try_direction(direction_to_try).map(|new_position| State {
                        direction: direction_to_try,
                        position: new_position,
                    })
                })
                .collect(),
            Tile::Slope(direction_to_try) => {
                if let Some(new_position) = try_direction(direction_to_try) {
                    vec![State {
                        direction: direction_to_try,
                        position: new_position,
                    }]
                } else {
                    vec![]
                }
            }
            Tile::Forest => panic!("Cannot get the successors of the forest at {state:?}"),
        }
    }

    fn reachable_edges(&self) -> Vec<(State, State)> {
        let start_state = State {
            direction: Direction::South,
            position: START_POS,
        };

        let mut seen = HashSet::from([start_state]);
        let mut stack = vec![start_state];

        let mut edges = Vec::new();

        while let Some(state) = stack.pop() {
            let successors = self.get_successors(&state);
            for successor in successors {
                edges.push((state, successor));
                if !seen.contains(&successor) {
                    seen.insert(successor);
                    stack.push(successor);
                }
            }
        }

        edges
    }

    // use bellman ford to find longest cycle
    // assume the path always is width 1 to avoid cycles
    fn longest_path(&self) -> usize {
        let start_state = State {
            direction: Direction::South,
            position: START_POS,
        };
        let end_state = State {
            direction: Direction::South,
            position: (self.num_rows() - 1, self.num_columns() - 2),
        };

        let edges = self.reachable_edges();

        let mut distances = HashMap::from([(start_state, 0)]);

        for _ in 0..self.num_elements() - 1 {
            for (from, to) in edges.iter() {
                if let Some(distance_from) = distances.get(from) {
                    // all edge weights are 1
                    let new_distance_to = distance_from + 1;
                    distances
                        .entry(*to)
                        .and_modify(|old_distance_to| {
                            if *old_distance_to < new_distance_to {
                                *old_distance_to = new_distance_to;
                            }
                        })
                        .or_insert(new_distance_to);
                }
            }
        }

        // assume no cycles :)

        distances[&end_state]
    }
}

fn main() {
    let puzzle_input = fs::read_to_string("input.txt").unwrap();

    let grid = parse_input(&puzzle_input);

    println!("Part 1: {}", grid.longest_path());

    let grid_part2 = Array2D::from_iter_row_major(
        grid.elements_row_major_iter()
            .copied()
            .map(|tile| match tile {
                Tile::Slope(_) => Tile::Path,
                tile => tile,
            }),
        grid.num_rows(),
        grid.num_columns(),
    )
    .unwrap();

    println!("Part 2: {}", grid_part2.longest_path());
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TEST_INPUT: &str = "#.#####################\n\
                              #.......#########...###\n\
                              #######.#########.#.###\n\
                              ###.....#.>.>.###.#.###\n\
                              ###v#####.#v#.###.#.###\n\
                              ###.>...#.#.#.....#...#\n\
                              ###v###.#.#.#########.#\n\
                              ###...#.#.#.......#...#\n\
                              #####.#.#.#######.#.###\n\
                              #.....#.#.#.......#...#\n\
                              #.#####.#.#.#########v#\n\
                              #.#...#...#...###...>.#\n\
                              #.#.#v#######v###.###v#\n\
                              #...#.>.#...>.>.#.###.#\n\
                              #####v#.#.###v#.#.###.#\n\
                              #.....#...#...#.#.#...#\n\
                              #.#########.###.#.#.###\n\
                              #...###...#...#...#.###\n\
                              ###.###.#.###v#####v###\n\
                              #...#...#.#.>.>.#.>.###\n\
                              #.###.###.#.###.#.#v###\n\
                              #.....###...###...#...#\n\
                              #####################.#";

    #[test]
    fn test_part1() {
        let grid = parse_input(TEST_INPUT);

        assert_eq!(grid.longest_path(), 94);
    }
}
