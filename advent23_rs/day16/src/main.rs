use std::{collections::HashSet, thread};

use anyhow::{Context, Result};
use array2d::Array2D;
use num::FromPrimitive;
use num_derive::FromPrimitive;

#[derive(Hash, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn try_move(
        &self,
        row: usize,
        column: usize,
        num_rows: usize,
        num_columns: usize,
    ) -> Option<(usize, usize)> {
        // return row, col
        match self {
            Direction::North => {
                if row == 0 {
                    None
                } else {
                    Some((row - 1, column))
                }
            }
            Direction::East => {
                if column == num_columns - 1 {
                    None
                } else {
                    Some((row, column + 1))
                }
            }
            Direction::South => {
                if row == num_rows - 1 {
                    None
                } else {
                    Some((row + 1, column))
                }
            }
            Direction::West => {
                if column == 0 {
                    None
                } else {
                    Some((row, column - 1))
                }
            }
        }
    }
}

#[derive(Debug, FromPrimitive, Clone)]
#[repr(u8)]
enum Tile {
    NorthSouthSplitter = b'|',
    EastWestSplitter = b'-',
    SlashMirror = b'/',
    BackSlashMirror = b'\\',
    Empty = b'.',
}

fn parse_input(input: &str) -> Result<Array2D<Tile>> {
    let rows = input
        .lines()
        .map(|line| {
            line.as_bytes()
                .iter()
                .map(|&c| {
                    Tile::from_u8(c)
                        .with_context(|| format!("Failed to parse tile from b'{}'", c as char))
                })
                .collect::<Result<Vec<Tile>, _>>()
        })
        .collect::<Result<Vec<Vec<Tile>>, _>>()?;

    Array2D::from_rows(&rows).context("Failed to parse input rows of tiles into array2d")
}

enum GetSuccessorsResult<T> {
    Zero,
    One(T),
    Two(T, T),
}

#[derive(Hash, PartialEq, Eq)]
struct State {
    row: usize,
    column: usize,
    direction: Direction,
}

impl State {
    fn new(row: usize, column: usize, direction: Direction) -> Self {
        State {
            row,
            column,
            direction,
        }
    }

    fn get_successors(&self, puzzle_input: &Array2D<Tile>) -> GetSuccessorsResult<Self> {
        let &State {
            row,
            column,
            direction: prev_dir,
        } = self;

        let mut next_dir = prev_dir;
        let mut next_dir2 = None;

        {
            use Direction::*;

            match puzzle_input[(row, column)] {
                Tile::NorthSouthSplitter => match prev_dir {
                    East | West => {
                        next_dir = North;
                        next_dir2 = Some(South);
                    }
                    _ => (),
                },
                Tile::EastWestSplitter => match prev_dir {
                    North | South => {
                        next_dir = East;
                        next_dir2 = Some(West);
                    }
                    _ => (),
                },
                Tile::SlashMirror => {
                    next_dir = match prev_dir {
                        North => East,
                        East => North,
                        South => West,
                        West => South,
                    };
                }
                Tile::BackSlashMirror => {
                    next_dir = match prev_dir {
                        North => West,
                        East => South,
                        South => East,
                        West => North,
                    };
                }
                Tile::Empty => (),
            }
        }

        let maybe_succ1 = if let Some((next_row, next_col)) = next_dir.try_move(
            row,
            column,
            puzzle_input.num_rows(),
            puzzle_input.num_columns(),
        ) {
            Some(State::new(next_row, next_col, next_dir))
        } else {
            None
        };

        let maybe_succ2 = if let Some(dir) = next_dir2 {
            if let Some((next_row, next_col)) = dir.try_move(
                row,
                column,
                puzzle_input.num_rows(),
                puzzle_input.num_columns(),
            ) {
                Some(State::new(next_row, next_col, dir))
            } else {
                None
            }
        } else {
            None
        };

        if let Some(succ1) = maybe_succ1 {
            if let Some(succ2) = maybe_succ2 {
                GetSuccessorsResult::Two(succ1, succ2)
            } else {
                GetSuccessorsResult::One(succ1)
            }
        } else if let Some(succ2) = maybe_succ2 {
            GetSuccessorsResult::One(succ2)
        } else {
            GetSuccessorsResult::Zero
        }
    }
}

fn num_energized(puzzle_input: &Array2D<Tile>, initial_state: State) -> usize {
    let mut energized = HashSet::new();

    let mut explored = HashSet::new();
    let mut frontier = Vec::from([initial_state]);

    while let Some(state) = frontier.pop() {
        if explored.contains(&state) {
            continue;
        }

        match state.get_successors(puzzle_input) {
            GetSuccessorsResult::One(succ) => frontier.push(succ),
            GetSuccessorsResult::Two(succ1, succ2) => {
                frontier.push(succ1);
                frontier.push(succ2);
            }
            GetSuccessorsResult::Zero => (),
        }

        energized.insert((state.row, state.column));
        explored.insert(state);
    }

    energized.len()
}

fn part1(puzzle_input: &Array2D<Tile>) -> usize {
    num_energized(puzzle_input, State::new(0, 0, Direction::East))
}

fn part2(puzzle_input: &Array2D<Tile>) -> usize {
    let from_left = (0..puzzle_input.num_rows()).map(|row| State::new(row, 0, Direction::East));
    let from_right = (0..puzzle_input.num_rows())
        .map(|row| State::new(row, puzzle_input.num_columns() - 1, Direction::West));
    let from_top =
        (0..puzzle_input.num_columns()).map(|column| State::new(0, column, Direction::South));
    let from_bottom = (0..puzzle_input.num_columns())
        .map(|column| State::new(puzzle_input.num_rows() - 1, column, Direction::North));

    thread::scope(|s| {
        let threads = from_left
            .chain(from_right)
            .chain(from_top)
            .chain(from_bottom)
            .map(|state| s.spawn(|| num_energized(puzzle_input, state)))
            .collect::<Vec<_>>();

        threads
            .into_iter()
            .map(|thread| thread.join().unwrap())
            .max()
            .unwrap()
    })
}

fn main() -> Result<()> {
    let file_contents = std::fs::read_to_string("input.txt")?;

    let puzzle_input = parse_input(&file_contents)?;

    println!("{}", part1(&puzzle_input));
    println!("{}", part2(&puzzle_input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(&parse_input(TEST_INPUT)?), 46);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(&parse_input(TEST_INPUT)?), 51);

        Ok(())
    }
}
