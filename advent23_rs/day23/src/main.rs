use std::{
    collections::{HashMap, HashSet},
    fs,
};

use array2d::Array2D;
use std::hash::Hash;
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
    fn is_opposite(&self, other: Self) -> bool {
        use Direction::*;
        let oppo = match self {
            North => South,
            East => West,
            South => North,
            West => East,
        };
        oppo == other
    }

    fn move_one_step(&self, point: Position) -> Position {
        let (row, column) = point;

        // assumes no underflow/overflow
        match self {
            Direction::North => (row - 1, column),
            Direction::East => (row, column + 1),
            Direction::South => (row + 1, column),
            Direction::West => (row, column - 1),
        }
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

type Position = (usize, usize);

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

trait GridExt {
    fn replace_slopes_with_paths(&mut self);
    fn get_successors(&self, position: Position) -> Vec<(Position, Direction)>;
    fn to_graph(&self) -> Graph<Position>;
    fn longest_path_len(&self) -> usize;
}

impl GridExt for Array2D<Tile> {
    fn replace_slopes_with_paths(&mut self) {
        let mut i = 0;
        while let Some(elem) = self.get_mut_row_major(i) {
            if matches!(*elem, Tile::Slope(_)) {
                *elem = Tile::Path;
            }
            i += 1;
        }
    }

    fn get_successors(&self, position: Position) -> Vec<(Position, Direction)> {
        let try_direction = |direction_to_try: Direction| {
            // only check out of bounds on top and bottom
            if position.0 == 0 && !matches!(direction_to_try, Direction::South) {
                return None;
            }

            if position.0 == self.num_rows() - 1 && !matches!(direction_to_try, Direction::North) {
                return None;
            }

            let new_position = direction_to_try.move_one_step(position);

            if matches!(self[new_position], Tile::Forest) {
                None
            } else {
                Some((new_position, direction_to_try))
            }
        };

        match self[position] {
            Tile::Path => Direction::iter().filter_map(try_direction).collect(),
            Tile::Slope(direction_to_try) => {
                if let Some(succ) = try_direction(direction_to_try) {
                    vec![succ]
                } else {
                    vec![]
                }
            }
            Tile::Forest => {
                panic!(
                    "Cannot get the successors of a Forest. get_successors called on ({:?})",
                    position
                )
            }
        }
    }

    #[inline(always)]
    fn to_graph(&self) -> Graph<Position> {
        self.into()
    }

    fn longest_path_len(&self) -> usize {
        self.to_graph()
            .longest_path_len_brute_force(&(0, 1), &(self.num_rows() - 1, self.num_columns() - 2))
            .unwrap()
    }
}

fn _longest_path_len_no_cycles(grid: &Array2D<Tile>) -> usize {
    grid.to_graph()
        ._longest_path_len(&(0, 1), &(grid.num_rows() - 1, grid.num_columns() - 2))
        .unwrap()
}

#[derive(Debug)]
struct Graph<T: PartialEq + Eq + Hash>(HashMap<T, Vec<(T, usize)>>);

impl From<&Array2D<Tile>> for Graph<Position> {
    fn from(grid: &Array2D<Tile>) -> Self {
        // assumes paths on the grid have width 1
        let mut stack: Vec<Position> = vec![(0, 1)];
        let mut graph = Graph::new();

        while let Some(cur_vertex) = stack.pop() {
            // HashMap<(successor (vertex): Position, distance: usize)>
            let mut cur_vertex_successors: HashMap<(usize, usize), usize> = HashMap::new();

            for (mut cur_pos, mut prev_direction) in grid.get_successors(cur_vertex) {
                for distance in 1.. {
                    let cur_pos_successors = grid.get_successors(cur_pos);

                    if cur_pos.0 == 0
                        || cur_pos.0 == grid.num_rows() - 1
                        || cur_pos_successors.len() >= 3
                    {
                        // cur_pos is a new vertex, with an edge from cur_vertex that has weight dist
                        if !graph.0.contains_key(&cur_pos) {
                            stack.push(cur_pos);
                        }

                        cur_vertex_successors
                            .entry(cur_pos)
                            .and_modify(|prev_dist| *prev_dist = (*prev_dist).max(distance))
                            .or_insert(distance);

                        break;
                    }

                    // since not a vertex, must have 0 or 1 or 2 successors
                    // and 1 of them may be where you just came from, so
                    // need to either get the 1 valid new successor
                    // or end this since this path doesn't lead to a new vertex
                    let valid_succ_or_empty = cur_pos_successors
                        .into_iter()
                        .find(|&(_, direction)| !direction.is_opposite(prev_direction));

                    if let Some((valid_succ, valid_direction)) = valid_succ_or_empty {
                        cur_pos = valid_succ;
                        prev_direction = valid_direction;
                    } else {
                        // path ends here and did not reach another vertex
                        break;
                    }
                }
            }

            graph.insert(cur_vertex, cur_vertex_successors.into_iter().collect());
        }

        graph
    }
}

impl<T: PartialEq + Eq + Hash + Clone> Graph<T> {
    fn new() -> Self {
        Graph(HashMap::new())
    }

    fn insert(&mut self, k: T, v: Vec<(T, usize)>) -> Option<Vec<(T, usize)>> {
        self.0.insert(k, v)
    }

    /// Perform a bellman-ford variant to find longest path length
    ///
    /// Only works if the graph has no cycles
    fn _longest_path_len(&self, source: &T, target: &T) -> Option<usize> {
        let mut distances = HashMap::from([(source, 0)]);

        for _ in 0..self.0.len() - 1 {
            for (from, successors) in self.0.iter() {
                if let Some(&distance_from) = distances.get(from) {
                    for (to, weight) in successors {
                        let new_distance_to = distance_from + weight;

                        distances
                            .entry(to)
                            .and_modify(|distance_to| {
                                if new_distance_to > *distance_to {
                                    *distance_to = new_distance_to;
                                }
                            })
                            .or_insert(new_distance_to);
                    }
                }
            }
        }

        // check for cycles
        for (from, successors) in self.0.iter() {
            if let Some(&distance_from) = distances.get(from) {
                for (to, weight) in successors {
                    if let Some(&distance_to) = distances.get(to) {
                        if distance_from + weight > distance_to {
                            return None;
                        }
                    }
                }
            }
        }

        distances.get(target).copied()
    }

    // brute force longest path from s to t
    // without going through a state w/ position in open_set
    fn longest_path_len_brute_force(&self, source: &T, target: &T) -> Option<usize> {
        enum StackItem<'a, T> {
            Todo((&'a T, usize)),
            Done(&'a T),
        }

        let mut open_set = HashSet::new();
        let mut stack = vec![StackItem::Todo((source, 0))];

        let mut result = None;

        while let Some(item) = stack.pop() {
            match item {
                StackItem::Todo((node, distance)) => {
                    if node == target {
                        result = Some(distance.max(result.unwrap_or(0)));
                        continue;
                    }

                    open_set.insert(node);

                    // removes from open_set when pops
                    stack.push(StackItem::Done(node));

                    for &(ref successor, weight) in &self.0[node] {
                        if !open_set.contains(successor) {
                            stack.push(StackItem::Todo((successor, distance + weight)));
                        }
                    }
                }
                StackItem::Done(node) => {
                    open_set.remove(node);
                }
            }
        }

        result
    }
}

fn main() {
    let puzzle_input = fs::read_to_string("input.txt").unwrap();

    let mut grid = parse_input(&puzzle_input);

    println!("Part 1: {}", grid.longest_path_len());

    // println!("Part 1: {}", _longest_path_len_no_cycles(&grid));

    grid.replace_slopes_with_paths();

    println!("Part 2: {}", grid.longest_path_len());
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

    const MY_TEST_INPUT: &str = "#.#####\n\
                                 #.....#\n\
                                 ###.#.#\n\
                                 #.....#\n\
                                 #.###.#\n\
                                 #.....#\n\
                                 #####.#";

    #[test]
    fn test_part1() {
        let grid = parse_input(TEST_INPUT);

        assert_eq!(grid.longest_path_len(), 94);
    }

    #[test]
    fn test_part1_no_brute_force() {
        let grid = parse_input(TEST_INPUT);

        assert_eq!(_longest_path_len_no_cycles(&grid), 94);
    }

    #[test]
    fn test_part2() {
        let mut grid = parse_input(TEST_INPUT);

        grid.replace_slopes_with_paths();

        assert_eq!(grid.longest_path_len(), 154);
    }

    #[test]
    fn my_test() {
        let grid = parse_input(MY_TEST_INPUT);

        let graph = grid.to_graph();

        for (node, succs) in graph.0.iter() {
            println!("{node:?}: {succs:?}");
        }
    }
}
