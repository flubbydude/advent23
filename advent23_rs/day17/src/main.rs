use anyhow::Result;
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

    fn _manhattan_distance(&self, other: &Self) -> usize {
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
    fn get_successors(&self, puzzle_input: &Array2D<u8>) -> Box<[(usize, State)]> {
        let mut result = Vec::new();

        // try going straight
        if self.num_straight != 2 {
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

        // try turning right
        let right = self.direction.turn_right();

        if let Some(position) =
            self.position
                .move_in_dir(right, puzzle_input.num_rows(), puzzle_input.num_columns())
        {
            result.push((
                puzzle_input[(position.row, position.column)] as usize,
                State {
                    position,
                    direction: right,
                    num_straight: 0,
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
                    num_straight: 0,
                },
            ))
        }

        result.into_boxed_slice()
    }
}

fn part1(puzzle_input: &Array2D<u8>) -> usize {
    let initial_states = [
        State {
            position: Position::ZERO,
            direction: Direction::East,
            num_straight: 0,
        },
        State {
            position: Position::ZERO,
            direction: Direction::South,
            num_straight: 0,
        },
    ];

    let goal_position = Position::new(puzzle_input.num_rows() - 1, puzzle_input.num_columns() - 1);

    let (total_cost, _path) = a_star_search(
        &initial_states,
        |state| state.get_successors(puzzle_input),
        |state| state.position == goal_position,
        |state| state.position._manhattan_distance(&goal_position),
    )
    .unwrap();

    // total cost = total heat loss
    total_cost
}

fn main() -> Result<()> {
    let file_contents = std::fs::read_to_string("input.txt")?;

    let puzzle_input = parse_input(&file_contents);

    println!("{}", part1(&puzzle_input));
    // println!("{}", part2(&puzzle_input));

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = "2413432311323\n\
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

    #[test]
    fn test_part_1() -> Result<()> {
        let puzzle_input = parse_input(TEST_INPUT);

        assert_eq!(part1(&puzzle_input), 102);

        Ok(())
    }
}
