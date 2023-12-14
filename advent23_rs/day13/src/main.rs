use std::fmt::{self, Write};

use anyhow::{bail, Context, Result};
use array2d::Array2D;

use num::FromPrimitive;
use num_derive::FromPrimitive;

#[derive(Debug, Clone, Copy, FromPrimitive)]
#[repr(u8)]
enum Tile {
    Ash = b'.',
    Rock = b'#',
}

impl Tile {
    fn switch_tile(&mut self) {
        *self = match self {
            Tile::Ash => Tile::Rock,
            Tile::Rock => Tile::Ash,
        };
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char(*self as u8 as char)
    }
}

// maybe could use manacher's alg for a speed up but that alg breaks my brain:

// find the line of reflection that extends to one edge of the slice
// basically finds the first palindrome touching an edge of the slice and returns the index to the right
// of the center
fn get_center_of_symmetry<T: Eq>(l: &[T]) -> Option<usize> {
    // center repr the elem just after the mirroring
    // or equivalently the number left/above the mirroring
    for center in 1..l.len() {
        let mut i = center - 1;
        let mut j = center;
        while l[i] == l[j] {
            if i == 0 || j == l.len() - 1 {
                return Some(center);
            }

            i -= 1;
            j += 1;
        }
    }

    None
}

fn get_new_center_of_symmetry<T: Eq>(l: &[T], old: usize) -> Option<usize> {
    // center repr the elem just after the mirroring
    // or equivalently the number left/above the mirroring
    for center in 1..l.len() {
        if center == old {
            continue;
        }

        let mut i = center - 1;
        let mut j = center;
        while l[i] == l[j] {
            if i == 0 || j == l.len() - 1 {
                return Some(center);
            }

            i -= 1;
            j += 1;
        }
    }

    None
}

#[derive(Debug)]
enum ReflectionLine {
    Vertical(usize),
    Horizontal(usize),
}

impl ReflectionLine {
    fn get_score(&self) -> usize {
        match *self {
            ReflectionLine::Vertical(num_cols) => num_cols,
            ReflectionLine::Horizontal(num_rows) => num_rows * 100,
        }
    }
}

#[derive(Clone)]
struct Pattern(Array2D<Tile>);

impl Pattern {
    // represent the rows as u64s
    // from binary
    // # is 1 and . is 0
    fn rows_as_u64_slice(&self) -> Box<[u64]> {
        let mut result = Vec::with_capacity(self.0.num_rows());
        for row in self.0.rows_iter() {
            let mut row_as_u64 = 0;
            for (i, cell) in row.enumerate() {
                if matches!(cell, Tile::Rock) {
                    row_as_u64 |= 1 << i;
                }
            }
            result.push(row_as_u64);
        }
        result.into_boxed_slice()
    }

    fn cols_as_u64_slice(&self) -> Box<[u64]> {
        let mut result = Vec::with_capacity(self.0.num_columns());
        for col in self.0.columns_iter() {
            let mut col_as_u64 = 0;
            for (i, cell) in col.enumerate() {
                if matches!(cell, Tile::Rock) {
                    col_as_u64 |= 1 << i;
                }
            }
            result.push(col_as_u64);
        }
        result.into_boxed_slice()
    }

    fn get_reflection_line(&self) -> Option<ReflectionLine> {
        if let Some(num_rows) = get_center_of_symmetry(&self.rows_as_u64_slice()) {
            Some(ReflectionLine::Horizontal(num_rows))
        } else if let Some(num_cols) = get_center_of_symmetry(&self.cols_as_u64_slice()) {
            Some(ReflectionLine::Vertical(num_cols))
        } else {
            None
        }
    }

    fn get_new_reflection_line(&self, prev_rl: &ReflectionLine) -> Option<ReflectionLine> {
        match prev_rl {
            &ReflectionLine::Horizontal(prev_num_rows) => {
                // try vertical then horiz.
                if let Some(new_num_cols) = get_center_of_symmetry(&self.cols_as_u64_slice()) {
                    Some(ReflectionLine::Vertical(new_num_cols))
                } else if let Some(new_num_rows) =
                    get_new_center_of_symmetry(&self.rows_as_u64_slice(), prev_num_rows)
                {
                    Some(ReflectionLine::Horizontal(new_num_rows))
                } else {
                    None
                }
            }
            &ReflectionLine::Vertical(prev_num_cols) => {
                // try vertical then horiz.
                if let Some(new_num_rows) = get_center_of_symmetry(&self.rows_as_u64_slice()) {
                    Some(ReflectionLine::Horizontal(new_num_rows))
                } else if let Some(new_num_cols) =
                    get_new_center_of_symmetry(&self.cols_as_u64_slice(), prev_num_cols)
                {
                    Some(ReflectionLine::Vertical(new_num_cols))
                } else {
                    None
                }
            }
        }
    }

    fn get_score(&self) -> Option<usize> {
        self.get_reflection_line().map(|rl| rl.get_score())
    }

    fn from_rows(rows: &[Vec<Tile>]) -> Result<Self, array2d::Error> {
        Array2D::from_rows(rows).map(|array| Pattern(array))
    }

    fn get_smudged_score(&self) -> Result<usize> {
        let prev_rl = self.get_reflection_line().unwrap();
        let mut pattern_cpy = self.clone();
        for i in 0..pattern_cpy.0.column_len() {
            for j in 0..pattern_cpy.0.row_len() {
                pattern_cpy.0[(i, j)].switch_tile();

                if let Some(rl) = pattern_cpy.get_new_reflection_line(&prev_rl) {
                    // assert_ne!(rl, prev_rl);
                    return Ok(rl.get_score());
                }

                pattern_cpy.0[(i, j)].switch_tile();
            }
        }

        // bad input => no smudge
        bail!("no smudge found in the following pattern:\n\n{}", self);
    }
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.0.rows_iter() {
            for tile in row {
                write!(f, "{}", tile)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

fn parse_input(input: &str) -> Result<Box<[Pattern]>> {
    let mut result = Vec::new();
    let mut cur_rows = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            if !cur_rows.is_empty() {
                result.push(Pattern::from_rows(&cur_rows)?);
                cur_rows.clear();
            }
        } else {
            let maybe_row = line
                .as_bytes()
                .iter()
                .copied()
                .map(Tile::from_u8)
                .collect::<Option<Vec<_>>>();

            if let Some(row) = maybe_row {
                cur_rows.push(row);
            } else {
                bail!("Input contains illegal character, line = {}", line);
            }
        }
    }

    if !cur_rows.is_empty() {
        result.push(Pattern::from_rows(&cur_rows)?);
    }

    Ok(result.into_boxed_slice())
}

fn part1(puzzle_input: &[Pattern]) -> Result<usize> {
    puzzle_input
        .iter()
        .map(|pattern| pattern.get_score())
        .sum::<Option<_>>()
        .context("bad input: part1 returned None on puzzle input")
}

fn part2(puzzle_input: &[Pattern]) -> Result<usize> {
    puzzle_input
        .iter()
        .map(|pattern| pattern.get_smudged_score())
        .sum()
}

fn main() -> Result<()> {
    let file_contents = std::fs::read("input.txt")?;
    let file_contents_str = std::str::from_utf8(&file_contents)?;

    let puzzle_input = parse_input(file_contents_str)?;

    println!("{}", part1(&puzzle_input)?);
    println!("{}", part2(&puzzle_input)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "#.##..##.\n\
                              ..#.##.#.\n\
                              ##......#\n\
                              ##......#\n\
                              ..#.##.#.\n\
                              ..##..##.\n\
                              #.#.##.#.\n\
                                       \n\
                              #...##..#\n\
                              #....#..#\n\
                              ..##..###\n\
                              #####.##.\n\
                              #####.##.\n\
                              ..##..###\n\
                              #....#..#";

    const TEST_INPUT_B: &str = "#...###.#..#...\n\
                                #....#...###..#\n\
                                #.##.###..###..\n\
                                #########.#.##.\n\
                                .#..#.##.###...\n\
                                .#..#.##.###...\n\
                                #########.#.##.";

    #[test]
    fn test_part_1() -> Result<()> {
        let puzzle_input = parse_input(TEST_INPUT)?;

        assert_eq!(part1(&puzzle_input).unwrap(), 405);

        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let puzzle_input = parse_input(TEST_INPUT)?;

        assert_eq!(part2(&puzzle_input)?, 400);

        Ok(())
    }

    #[test]
    fn test_part_2b() -> Result<()> {
        let puzzle_input = parse_input(TEST_INPUT_B)?;

        assert_eq!(part2(&puzzle_input)?, 3);

        Ok(())
    }
}
