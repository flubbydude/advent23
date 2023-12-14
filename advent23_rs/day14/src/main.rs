use std::collections::HashMap;

use anyhow::{Context, Result};
use array2d::Array2D;

use num::FromPrimitive;
use num_derive::FromPrimitive;

use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{EnumCount, EnumIter};

#[derive(Debug, FromPrimitive, Clone, Hash, PartialEq, Eq)]
#[repr(u8)]
enum Cell {
    Round = b'O',
    Cube = b'#',
    Empty = b'.',
}

#[derive(Debug, EnumIter, EnumCount, Hash, PartialEq, Eq, Clone, Copy)]
enum Direction {
    North,
    West,
    South,
    East,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Platform(Array2D<Cell>);

impl Platform {
    fn tilt(&mut self, direction: Direction) {
        match direction {
            Direction::North | Direction::South => {
                let num_rows = self.0.num_rows();
                for column in 0..self.0.num_columns() {
                    let mut num_round = 0;

                    let process_cell = |row| {
                        match self.0[(row, column)] {
                            Cell::Round => {
                                num_round += 1;
                                self.0[(row, column)] = Cell::Empty;
                            }
                            Cell::Cube => {
                                // set previous num_round elems to Os
                                let range = if matches!(direction, Direction::North) {
                                    row + 1..row + 1 + num_round
                                } else {
                                    row - num_round..row
                                };
                                for prev_row in range {
                                    self.0[(prev_row, column)] = Cell::Round;
                                }
                                num_round = 0;
                            }
                            Cell::Empty => (),
                        }
                    };

                    if matches!(direction, Direction::North) {
                        (0..num_rows).rev().for_each(process_cell)
                    } else {
                        (0..num_rows).for_each(process_cell)
                    }

                    let range = if matches!(direction, Direction::North) {
                        0..num_round
                    } else {
                        num_rows - num_round..num_rows
                    };

                    for prev_row in range {
                        self.0[(prev_row, column)] = Cell::Round;
                    }
                }
            }
            Direction::East | Direction::West => {
                let num_columns = self.0.num_columns();

                for row in 0..self.0.num_rows() {
                    let mut num_round = 0;

                    // mutates self
                    let process_cell = |column| {
                        match self.0[(row, column)] {
                            Cell::Round => {
                                num_round += 1;
                                self.0[(row, column)] = Cell::Empty;
                            }
                            Cell::Cube => {
                                // set previous num_round elems to Os
                                let range = if matches!(direction, Direction::West) {
                                    column + 1..column + 1 + num_round
                                } else {
                                    column - num_round..column
                                };
                                for prev_column in range {
                                    self.0[(row, prev_column)] = Cell::Round;
                                }
                                num_round = 0;
                            }
                            Cell::Empty => (),
                        }
                    };

                    if matches!(direction, Direction::West) {
                        (0..num_columns).rev().for_each(process_cell)
                    } else {
                        (0..num_columns).for_each(process_cell)
                    }

                    let range = if matches!(direction, Direction::West) {
                        0..num_round
                    } else {
                        num_columns - num_round..num_columns
                    };

                    for prev_column in range {
                        self.0[(row, prev_column)] = Cell::Round;
                    }
                }
            }
        }
    }

    fn total_load(&self) -> usize {
        self.0
            .columns_iter()
            .flat_map(|column| column.rev().enumerate())
            .filter_map(|(i, cell)| {
                if matches!(cell, Cell::Round) {
                    Some(i + 1)
                } else {
                    None
                }
            })
            .sum()
    }
}

impl TryFrom<&str> for Platform {
    type Error = anyhow::Error;

    fn try_from(input: &str) -> Result<Self> {
        let rows = input
            .lines()
            .map(str::as_bytes)
            .map(|line| {
                line.iter()
                    .cloned()
                    .map(|c| {
                        Cell::from_u8(c)
                            .with_context(|| format!("Cannot parse b'{}' into Cell", c as char))
                    })
                    .collect()
            })
            .collect::<Result<Vec<Vec<Cell>>>>()?;

        let matrix = Array2D::from_rows(&rows)?;

        Ok(Platform(matrix))
    }
}

fn part1(mut platform: Platform) -> usize {
    platform.tilt(Direction::North);

    platform.total_load()
}

fn part2(mut platform: Platform) -> usize {
    const NUM_CYCLES: usize = 1000000000;
    const CYCLE_LENGTH: usize = Direction::COUNT;
    const NUM_TILTS: usize = NUM_CYCLES * CYCLE_LENGTH;

    let mut prev_states = HashMap::new();

    let mut remaining_tilts = None;

    for (i, direction) in Direction::iter().cycle().take(NUM_TILTS).enumerate() {
        if let Some(val) = remaining_tilts {
            if val == 0 {
                break;
            }
            remaining_tilts = Some(val - 1);
        } else {
            let key = (platform.clone(), direction);
            if let Some(&state_cycle_start) = prev_states.get(&key) {
                let state_cycle_length = i - state_cycle_start;
                let total_remaining_tilts = NUM_TILTS - i;
                let shortcut_remaining_tilts = total_remaining_tilts % state_cycle_length;
                if shortcut_remaining_tilts == 0 {
                    break;
                }

                remaining_tilts = Some(shortcut_remaining_tilts - 1);
            } else {
                prev_states.insert(key, i);
            }
        }

        platform.tilt(direction);
    }

    platform.total_load()
}

fn main() -> Result<()> {
    let file_contents = std::fs::read("input.txt")?;
    let file_contents_str = std::str::from_utf8(&file_contents)?;

    let platform: Platform = file_contents_str
        .try_into()
        .context("Unable to parse Platform from file \"input.txt\"")?;

    println!("{}", part1(platform.clone()));
    println!("{}", part2(platform));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "O....#....\n\
                              O.OO#....#\n\
                              .....##...\n\
                              OO.#O....O\n\
                              .O.....O#.\n\
                              O.#..O.#.#\n\
                              ..O..#O..O\n\
                              .......O..\n\
                              #....###..\n\
                              #OO..#....";

    #[test]
    fn test_part_1() -> Result<()> {
        let puzzle_input = TEST_INPUT.try_into().context("Unable to parse platform")?;

        assert_eq!(part1(puzzle_input), 136);

        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let puzzle_input = TEST_INPUT.try_into().context("Unable to parse platform")?;

        assert_eq!(part2(puzzle_input), 64);

        Ok(())
    }
}
