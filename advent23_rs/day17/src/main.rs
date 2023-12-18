use array2d::Array2D;

mod search;

use search::a_star_search;

fn parse_input(input: &str) -> Array2D<u8> {
    let rows = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect::<Vec<_>>();

    Array2D::from_rows(&rows).unwrap()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_right(&self) -> Self {
        use Direction::*;
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }

    fn turn_left(&self) -> Self {
        use Direction::*;

        match self {
            North => West,
            East => North,
            South => East,
            West => South,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Position {
    row: usize,
    column: usize,
}

impl Position {
    const ZERO: Position = Position { row: 0, column: 0 };

    fn new(row: usize, column: usize) -> Self {
        Position { row, column }
    }

    fn manhattan_distance(&self, other: &Self) -> usize {
        let di = if self.row > other.row {
            self.row - other.row
        } else {
            other.row - self.row
        };

        let dj = if self.column > other.column {
            self.column - other.column
        } else {
            other.column - self.column
        };

        di + dj
    }

    fn move_in_dir(
        &self,
        direction: Direction,
        num_rows: usize,
        num_columns: usize,
    ) -> Option<Self> {
        use Direction::*;
        match direction {
            North => {
                if self.row == 0 {
                    None
                } else {
                    Some(Position::new(self.row - 1, self.column))
                }
            }
            East => {
                if self.column == num_columns - 1 {
                    None
                } else {
                    Some(Position::new(self.row, self.column + 1))
                }
            }
            South => {
                if self.row == num_rows - 1 {
                    None
                } else {
                    Some(Position::new(self.row + 1, self.column))
                }
            }
            West => {
                if self.column == 0 {
                    None
                } else {
                    Some(Position::new(self.row, self.column - 1))
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    position: Position,
    direction: Direction,

    // how many times you have gone straight in a row
    num_straight: usize,
}

impl State {
    fn get_successors(
        &self,
        puzzle_input: &Array2D<u8>,
        is_ultra_crucible: bool,
    ) -> Vec<(usize, State)> {
        let mut result = Vec::new();

        let can_go_straight = if is_ultra_crucible {
            self.num_straight < 10
        } else {
            self.num_straight < 3
        };

        // try going straight
        if can_go_straight {
            if let Some(position) = self.position.move_in_dir(
                self.direction,
                puzzle_input.num_rows(),
                puzzle_input.num_columns(),
            ) {
                result.push((
                    puzzle_input[(position.row, position.column)] as usize,
                    State {
                        position,
                        direction: self.direction,
                        num_straight: self.num_straight + 1,
                    },
                ))
            }
        }

        let can_turn = if is_ultra_crucible {
            self.num_straight >= 4
        } else {
            true
        };

        if can_turn {
            // try turning right
            let right = self.direction.turn_right();

            if let Some(position) = self.position.move_in_dir(
                right,
                puzzle_input.num_rows(),
                puzzle_input.num_columns(),
            ) {
                result.push((
                    puzzle_input[(position.row, position.column)] as usize,
                    State {
                        position,
                        direction: right,
                        num_straight: 1,
                    },
                ))
            }

            // try turning left
            let left = self.direction.turn_left();

            if let Some(position) =
                self.position
                    .move_in_dir(left, puzzle_input.num_rows(), puzzle_input.num_columns())
            {
                result.push((
                    puzzle_input[(position.row, position.column)] as usize,
                    State {
                        position,
                        direction: left,
                        num_straight: 1,
                    },
                ))
            }
        }

        result
    }
}

fn get_minimum_heat(puzzle_input: &Array2D<u8>, is_ultra_crucible: bool) -> usize {
    let goal_position = Position::new(puzzle_input.num_rows() - 1, puzzle_input.num_columns() - 1);

    // ultra crucible must go straight 4 times before goal.
    a_star_search(
        State {
            position: Position::ZERO,
            direction: Direction::East,
            num_straight: 0,
        },
        |state| state.get_successors(puzzle_input, is_ultra_crucible),
        |state| state.position == goal_position && (!is_ultra_crucible || state.num_straight >= 4),
        |state| state.position.manhattan_distance(&goal_position),
    )
    .unwrap()
}

fn part1(puzzle_input: &Array2D<u8>) -> usize {
    get_minimum_heat(puzzle_input, false)
}

fn part2(puzzle_input: &Array2D<u8>) -> usize {
    get_minimum_heat(puzzle_input, true)
}

fn main() {
    let file_contents = std::fs::read_to_string("input.txt").unwrap();

    let puzzle_input = parse_input(&file_contents);

    println!("{}", part1(&puzzle_input));
    println!("{}", part2(&puzzle_input));
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT_A: &str = "2413432311323\n\
                                3215453535623\n\
                                3255245654254\n\
                                3446585845452\n\
                                4546657867536\n\
                                1438598798454\n\
                                4457876987766\n\
                                3637877979653\n\
                                4654967986887\n\
                                4564679986453\n\
                                1224686865563\n\
                                2546548887735\n\
                                4322674655533";

    const TEST_INPUT_B: &str = "111111111111\n\
                                999999999991\n\
                                999999999991\n\
                                999999999991\n\
                                999999999991";

    #[test]
    fn test_part_1() {
        let puzzle_input = parse_input(TEST_INPUT_A);

        assert_eq!(part1(&puzzle_input), 102);
    }

    #[test]
    fn test_part_2a() {
        let puzzle_input = parse_input(TEST_INPUT_A);

        assert_eq!(part2(&puzzle_input), 94);
    }

    #[test]
    fn test_part_2b() {
        let puzzle_input = parse_input(TEST_INPUT_B);

        assert_eq!(part2(&puzzle_input), 71);
    }
}
