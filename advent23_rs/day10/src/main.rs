use std::mem;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use num::FromPrimitive;
use num_derive::FromPrimitive;

#[derive(Debug, EnumIter, PartialEq, Eq, Clone, Copy, FromPrimitive)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, FromPrimitive, Clone, Copy)]
enum Turn {
    Straight,
    Right,
    Backwards,
    Left,
}

impl Direction {
    fn get_successor(&self, i: usize, j: usize) -> (isize, isize) {
        use Direction::*;
        match *self {
            North => (i as isize - 1, j as isize),
            South => (i as isize + 1, j as isize),
            East => (i as isize, j as isize + 1),
            West => (i as isize, j as isize - 1),
        }
    }

    fn opposite(&self) -> Direction {
        // Direction::from_i32((*self as i32 + 2) % 4).unwrap()
        use Direction::*;
        match *self {
            North => South,
            South => North,
            East => West,
            West => East,
        }
    }

    fn get_turn_since_last(&self, last_dir: Direction) -> Turn {
        Turn::from_i32((*self as i32 + 4 - last_dir as i32) % 4).unwrap()
    }

    fn turn(&self, turn: Turn) -> Direction {
        match turn {
            Turn::Right => Direction::from_i32((*self as i32 + 1) % 4).unwrap(),
            Turn::Left => Direction::from_i32((*self as i32 + 3) % 4).unwrap(),
            Turn::Straight => *self,
            Turn::Backwards => self.opposite(),
        }
    }
}

// since i never construct the tile variants but start
#[allow(dead_code)]
#[repr(u8)]
#[derive(PartialEq, Eq, Clone, Copy)]
enum Tile {
    NorthSouth = b'|',
    EastWest = b'-',
    NorthEast = b'L',
    NorthWest = b'J',
    SouthWest = b'7',
    SouthEast = b'F',
    Ground = b'.',
    Start = b'S',
}

impl Tile {
    fn connects_to(&self, dir: Direction) -> bool {
        match self.dirs() {
            Some([d1, d2]) => dir == d1 || dir == d2,
            None => false,
        }
    }

    fn dirs(&self) -> Option<[Direction; 2]> {
        use Direction::*;

        match *self {
            Tile::NorthSouth => Some([North, South]),
            Tile::EastWest => Some([East, West]),
            Tile::NorthEast => Some([North, East]),
            Tile::NorthWest => Some([North, West]),
            Tile::SouthWest => Some([South, West]),
            Tile::SouthEast => Some([South, East]),
            Tile::Ground => None,
            Tile::Start => None,
        }
    }
}

#[repr(u8)]
enum TileInfo {
    Enclosed,
    InLoop,
    // NotEnclosed,
}

fn get_intitial_pipe_state(puzzle_input: &[&[Tile]], num_cols: usize) -> PipeState {
    let (s_row, s_col) = puzzle_input
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().position(|&c| c == Tile::Start).map(|j| (i, j)))
        .next()
        .unwrap();

    Direction::iter()
        .find_map(|dir| {
            let (start_row, start_col) = dir.get_successor(s_row, s_col);

            if start_row < 0 || start_col < 0 {
                return None;
            }

            let start_row = start_row as usize;
            let start_col = start_col as usize;

            if start_row < puzzle_input.len()
                && start_col < num_cols
                && puzzle_input[start_row][start_col].connects_to(dir.opposite())
            {
                Some(PipeState {
                    row: start_row,
                    col: start_col,
                    dir,
                })
            } else {
                None
            }
        })
        .unwrap()
}

struct PipeIterator<'a> {
    puzzle_input: &'a [&'a [Tile]],
    num_cols: usize,
    state: PipeState,
    done: bool,
}

#[derive(Clone, Copy, Debug)]
struct PipeState {
    row: usize,
    col: usize,
    dir: Direction,
}

fn get_pipe_iterator<'a>(
    puzzle_input: &'a [&'a [Tile]],
    num_cols: usize,
    initial_state: PipeState,
) -> PipeIterator<'a> {
    PipeIterator {
        puzzle_input,
        num_cols,
        state: initial_state,
        done: false,
    }
}

impl Iterator for PipeIterator<'_> {
    type Item = PipeState;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let state = &mut self.state;

        if self.puzzle_input[state.row][state.col] == Tile::Start {
            self.done = true;
            return Some(*state);
        }

        // get next direction
        let [d1, d2] = self.puzzle_input[state.row][state.col].dirs().unwrap();

        state.dir = if d1 == state.dir.opposite() { d2 } else { d1 };

        let result = *state;

        let (next_row, next_col) = state.dir.get_successor(state.row, state.col);

        assert!(next_row >= 0 && next_col >= 0);

        state.row = next_row as usize;
        state.col = next_col as usize;

        assert!(state.row < self.puzzle_input.len());
        assert!(state.col < self.num_cols);

        Some(result)
    }
}

fn part1(puzzle_input: &[&[Tile]], num_cols: usize) -> usize {
    // first find the position of the S
    // go 1 step in a direction
    // follow the pipe until reaching S
    // count the number of time taken to reach S
    // divide by 2
    let num_steps = get_pipe_iterator(
        puzzle_input,
        num_cols,
        get_intitial_pipe_state(puzzle_input, num_cols),
    )
    .count();

    num_steps / 2
}

fn flood_fill(tile_info_arr: &mut [Vec<Option<TileInfo>>], num_cols: usize, i: usize, j: usize) {
    tile_info_arr[i][j] = Some(TileInfo::Enclosed);

    for dir in Direction::iter() {
        let (new_i, new_j) = dir.get_successor(i, j);
        if new_i < 0 || new_j < 0 {
            continue;
        }

        let new_i = new_i as usize;
        let new_j = new_j as usize;

        if new_i >= tile_info_arr.len() || new_j >= num_cols {
            continue;
        }

        if tile_info_arr[new_i][new_j].is_some() {
            continue;
        }

        flood_fill(tile_info_arr, num_cols, new_i, new_j);
    }
}

fn part2(puzzle_input: &[&[Tile]], num_cols: usize) -> usize {
    let mut tile_info_arr = Vec::with_capacity(puzzle_input.len());
    for _ in 0..puzzle_input.len() {
        let mut v = Vec::with_capacity(num_cols);
        for _ in 0..num_cols {
            v.push(None);
        }

        tile_info_arr.push(v);
    }

    // follow main loop and count lefts vs rights
    let initial_state = get_intitial_pipe_state(puzzle_input, num_cols);

    let mut prev_dir = initial_state.dir;

    let mut num_lefts = 0;
    let mut num_rights = 0;

    for pipe_state in get_pipe_iterator(puzzle_input, num_cols, initial_state) {
        match pipe_state.dir.get_turn_since_last(prev_dir) {
            Turn::Right => num_rights += 1,
            Turn::Left => num_lefts += 1,
            _ => (),
        }

        tile_info_arr[pipe_state.row][pipe_state.col] = Some(TileInfo::InLoop);

        prev_dir = pipe_state.dir;
    }

    let inside_turn = if num_rights > num_lefts {
        Turn::Right
    } else {
        Turn::Left
    };

    let mut check_next_to_inside = |row, col, dir: Direction| {
        let (irow, icol) = dir.turn(inside_turn).get_successor(row, col);

        assert!(irow >= 0 && icol >= 0);
        let irow = irow as usize;
        let icol = icol as usize;
        assert!(irow < puzzle_input.len() && icol < num_cols);

        if tile_info_arr[irow][icol].is_none() {
            tile_info_arr[irow][icol] = Some(TileInfo::Enclosed);
        }
    };

    // follow main loop again with same starts, but this time
    // turn inside_turn every step and set it in the array
    // tile_info_arr
    prev_dir = initial_state.dir;
    for pipe_state in get_pipe_iterator(puzzle_input, num_cols, initial_state) {
        // (irow, icol) is on the inside of the loop if not on the loop
        check_next_to_inside(pipe_state.row, pipe_state.col, prev_dir);

        prev_dir = pipe_state.dir;

        // go againe
        check_next_to_inside(pipe_state.row, pipe_state.col, pipe_state.dir);
    }

    for i in 0..tile_info_arr.len() {
        for j in 0..num_cols {
            if matches!(tile_info_arr[i][j], Some(TileInfo::Enclosed)) {
                flood_fill(&mut tile_info_arr, num_cols, i, j);
            }
        }
    }

    tile_info_arr
        .iter()
        .flat_map(|row| row.iter())
        .filter(|cell| matches!(cell, Some(TileInfo::Enclosed)))
        .count()
}

fn main() {
    let file_contents = std::fs::read("input.txt").unwrap();
    let file_contents_as_str = std::str::from_utf8(&file_contents).unwrap();

    let puzzle_input: Vec<&[Tile]>;

    // assuming input is valid:
    unsafe {
        puzzle_input = file_contents_as_str
            .lines()
            .map(|line| mem::transmute(line.as_bytes()))
            .collect();
    }

    let num_cols = puzzle_input[0].len();

    println!("{}", part1(&puzzle_input, num_cols));
    println!("{}", part2(&puzzle_input, num_cols));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_A: &str = ".F----7F7F7F7F-7....\n\
                                .|F--7||||||||FJ....\n\
                                .||.FJ||||||||L7....\n\
                                FJL7L7LJLJ||LJ.L-7..\n\
                                L--J.L7...LJS7F-7L7.\n\
                                ....F-J..F7FJ|L7L7L7\n\
                                ....L7.F7||L7|.L7L7|\n\
                                .....|FJLJ|FJ|F7|.LJ\n\
                                ....FJL-7.||.||||...\n\
                                ....L---J.LJ.LJLJ...";

    fn test_part_2(input_as_str: &str, answer: usize) {
        let puzzle_input: Vec<&[Tile]>;

        // assuming input is valid:
        unsafe {
            puzzle_input = input_as_str
                .lines()
                .map(|line| mem::transmute(line.as_bytes()))
                .collect();
        }

        let num_cols = puzzle_input[0].len();

        assert_eq!(part2(&puzzle_input, num_cols), answer);
    }

    #[test]
    fn stu_test() {
        test_part_2(TEST_INPUT_A, 8);
    }
}
