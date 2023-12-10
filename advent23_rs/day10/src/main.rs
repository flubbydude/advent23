use std::mem;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter, PartialEq, Eq, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn as_vec2(&self) -> (isize, isize) {
        use Direction::*;
        match *self {
            North => (-1, 0),
            South => (1, 0),
            East => (0, 1),
            West => (0, -1),
        }
    }

    fn opposite(&self) -> Direction {
        use Direction::*;
        match *self {
            North => South,
            South => North,
            East => West,
            West => East,
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
            Some(dirs) => dirs[0] == dir || dirs[1] == dir,
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

fn part1(puzzle_input: &[&[Tile]], num_cols: usize) -> i32 {
    // first find the position of the S
    let (s_row, s_col) = puzzle_input
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().position(|&c| c == Tile::Start).map(|j| (i, j)))
        .next()
        .unwrap();

    for start_dir in Direction::iter() {
        let (d_row, d_col) = start_dir.as_vec2();
        let start_row = s_row as isize + d_row;
        let start_col = s_col as isize + d_col;

        if start_row < 0 || start_col < 0 {
            continue;
        }

        let start_row = start_row as usize;
        let start_col = start_col as usize;

        if start_row >= puzzle_input.len() || start_col >= num_cols {
            continue;
        }

        if !puzzle_input[start_row][start_row].connects_to(start_dir.opposite()) {
            continue;
        }

        println!(
            "{:?} connects to {:?} = {}",
            puzzle_input[start_row][start_col],
            start_dir.opposite(),
            puzzle_input[start_row][start_row].connects_to(start_dir.opposite())
        );

        println!(
            "{:?} connects to {:?} = {}",
            Tile::SouthWest,
            Direction::East,
            Tile::SouthWest.connects_to(Direction::East)
        );

        let mut row = start_row;
        let mut col = start_col;

        let mut i = 0;
        let mut prev_dir = start_dir.opposite();
        // go until reaching the S again
        while row != s_row && col != s_col {
            let tile = puzzle_input[row][col];
            let dirs = tile.dirs().unwrap();

            let dir = if dirs[0] == prev_dir {
                dirs[1]
            } else {
                dirs[0]
            };

            let (d_row, d_col) = dir.as_vec2();
            row = (row as isize + d_row) as usize;
            col = (col as isize + d_col) as usize;
            prev_dir = dir.opposite();
            i += 1;
        }

        return i / 2;
    }

    // there was no valid spot to start lol
    panic!()
}

fn main() {
    let file_contents = std::fs::read("input.txt").unwrap();
    let file_contents_as_str = std::str::from_utf8(&file_contents).unwrap();

    let puzzle_input: Vec<&[Tile]>;

    unsafe {
        puzzle_input = file_contents_as_str
            .lines()
            .map(|line| mem::transmute(line.as_bytes()))
            .collect();
    }

    let num_cols = puzzle_input[0].len();

    println!("{}", part1(&puzzle_input, num_cols));
}
