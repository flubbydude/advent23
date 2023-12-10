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

#[derive(FromPrimitive, Clone, Copy)]
enum Turn {
    Right,
    Left,
    Straight,
    Backwards,
}

impl Direction {
    fn get_successor(&self, i: usize, j: usize) -> (usize, usize) {
        use Direction::*;
        match *self {
            North => (i - 1, j),
            South => (i + 1, j),
            East => (i, j + 1),
            West => (i, j - 1),
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

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
    NotEnclosed,
}

fn find_s_and_start_dir(puzzle_input: &[&[Tile]], num_cols: usize) -> (usize, usize, Direction) {
    let (s_row, s_col) = puzzle_input
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().position(|&c| c == Tile::Start).map(|j| (i, j)))
        .next()
        .unwrap();

    let start_dir = Direction::iter()
        .filter_map(|dir| {
            let (start_row, start_col) = dir.get_successor(s_row, s_col);

            if start_row < puzzle_input.len()
                && start_col < num_cols
                && puzzle_input[start_row][start_col].connects_to(dir.opposite())
            {
                Some(dir)
            } else {
                None
            }
        })
        .next()
        .unwrap();

    (s_row, s_col, start_dir)
}

fn part1(puzzle_input: &[&[Tile]], num_cols: usize) -> usize {
    // first find the position of the S
    let (s_row, s_col, start_dir) = find_s_and_start_dir(puzzle_input, num_cols);

    let (mut row, mut col) = start_dir.get_successor(s_row, s_col);
    let mut prev_dir = start_dir;
    for i in 1.. {
        if row == s_row && col == s_col {
            return i / 2;
        }

        let dirs = puzzle_input[row][col].dirs().unwrap();

        let dir = if dirs[0] == prev_dir.opposite() {
            dirs[1]
        } else {
            dirs[0]
        };

        (row, col) = dir.get_successor(row, col);
        prev_dir = dir;
    }

    unreachable!()
}

// assume the flood fill never reachest the edges of the array (0 or num rows or num cols)
fn flood_fill(
    tile_info_arr: &mut [Vec<Option<TileInfo>>],
    i: usize,
    j: usize,
    prev_dir: Option<Direction>,
) {
    if tile_info_arr[i][j].is_some() {
        return;
    }

    tile_info_arr[i][j] = Some(TileInfo::Enclosed);

    for dir in Direction::iter() {
        if prev_dir.is_some() && dir == prev_dir.unwrap().opposite() {
            continue;
        }
        let (new_i, new_j) = dir.get_successor(i, j);
        flood_fill(tile_info_arr, new_i, new_j, Some(dir));
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

    let (s_row, s_col, start_dir) = find_s_and_start_dir(puzzle_input, num_cols);
    let (mut row, mut col) = start_dir.get_successor(s_row, s_col);
    let mut prev_dir = start_dir;

    // follow main loop and count lefts vs rights
    let mut num_lefts = 0;
    let mut num_rights = 0;
    loop {
        tile_info_arr[row][col] = Some(TileInfo::InLoop);

        if row == s_row && col == s_col {
            break;
        }

        let dirs = puzzle_input[row][col].dirs().unwrap();

        let dir = if dirs[0] == prev_dir.opposite() {
            dirs[1]
        } else {
            dirs[0]
        };

        match dir.get_turn_since_last(prev_dir) {
            Turn::Right => num_rights += 1,
            Turn::Left => num_lefts += 1,
            _ => (),
        }

        (row, col) = dir.get_successor(row, col);
        prev_dir = dir;
    }

    let inside_turn = if num_rights > num_lefts {
        Turn::Right
    } else {
        Turn::Left
    };

    let (mut row, mut col) = start_dir.get_successor(s_row, s_col);
    let mut prev_dir = start_dir;

    // follow main loop again with same starts, but this time
    // turn inside_turn every step and set it in the array
    // tile_info_arr
    loop {
        let (in_row, in_col) = prev_dir.turn(inside_turn).get_successor(row, col);
        if tile_info_arr[in_row][in_col].is_none() {
            tile_info_arr[in_row][in_col] = Some(TileInfo::Enclosed);
        }

        let dirs = puzzle_input[row][col].dirs().unwrap();

        let dir = if dirs[0] == prev_dir.opposite() {
            dirs[1]
        } else {
            dirs[0]
        };

        let (in_row, in_col) = dir.turn(inside_turn).get_successor(row, col);
        if tile_info_arr[in_row][in_col].is_none() {
            tile_info_arr[in_row][in_col] = Some(TileInfo::Enclosed);
        }

        if row == s_row && col == s_col {
            break;
        }

        (row, col) = dir.get_successor(row, col);
        prev_dir = dir;
    }

    for i in 0..tile_info_arr.len() {
        for j in 0..num_cols {
            flood_fill(&mut tile_info_arr, i, j, None);
        }
    }

    tile_info_arr
        .iter()
        .flat_map(|row| row.iter())
        .filter(|cell| match cell {
            Some(TileInfo::Enclosed) => true,
            _ => false,
        })
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
