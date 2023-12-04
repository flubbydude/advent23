use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, error};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

lazy_static! {
    static ref INT_REGEX: Regex = Regex::new("[0-9]+").unwrap();
}

#[derive(Debug)]
struct EngineNumber {
    row_index: usize,
    start: usize,
    end: usize,
    number: i32,
}

fn is_symbol(c: u8) -> bool {
    !c.is_ascii_digit() && c != b'.'
}

impl EngineNumber {
    fn get_neighbors(
        &self,
        num_rows: usize,
        num_cols: usize,
    ) -> impl '_ + Iterator<Item = (usize, usize)> {
        let start_row = if self.row_index > 0 {
            self.row_index - 1
        } else {
            0
        };

        let start_col = if self.start > 0 { self.start - 1 } else { 0 };

        // lmao I could have just returned a vector instead but I'm just known to be silly like that
        (start_row..=self.row_index + 1)
            .filter(move |&i| i < num_rows)
            .flat_map(move |i| {
                (start_col..=self.end)
                    .filter(move |&j| j < num_cols)
                    .map(move |j| (i, j))
            })
            .filter(|&(i, j)| i != self.row_index || j < self.start || j == self.end)
    }

    fn is_part_number(&self, puzzle_input: &[&[u8]]) -> bool {
        let num_rows = puzzle_input.len();
        let num_cols = puzzle_input[0].len();

        return self
            .get_neighbors(num_rows, num_cols)
            .any(|(i, j)| is_symbol(puzzle_input[i][j]));
    }
}

fn find_engine_numbers(puzzle_input: &[&[u8]]) -> Result<Vec<EngineNumber>> {
    let mut result = Vec::new();

    for (i, &line) in puzzle_input.iter().enumerate() {
        let line_as_str = std::str::from_utf8(line)?;
        for num_match in INT_REGEX.find_iter(line_as_str) {
            result.push(EngineNumber {
                row_index: i,
                start: num_match.start(),
                end: num_match.end(),
                number: num_match.as_str().parse()?,
            })
        }
    }

    Ok(result)
}

fn part1(puzzle_input: &[&[u8]], engine_numbers: &[EngineNumber]) -> i32 {
    engine_numbers
        .iter()
        .filter_map(|engine_number| {
            if engine_number.is_part_number(puzzle_input) {
                Some(engine_number.number)
            } else {
                None
            }
        })
        .sum()
}

fn part2(puzzle_input: &[&[u8]], engine_numbers: &[EngineNumber]) -> i32 {
    let num_rows = puzzle_input.len();
    let num_cols = puzzle_input[0].len();

    let mut map = HashMap::new();
    for engine_number in engine_numbers {
        for (i, j) in engine_number.get_neighbors(num_rows, num_cols) {
            if puzzle_input[i][j] == b'*' {
                map.entry((i, j))
                    .and_modify(|val| *val *= -engine_number.number)
                    .or_insert(-engine_number.number);
            }
        }
    }

    map.values().filter(|&&x| x >= 0).sum()
}

fn main() -> Result<()> {
    let file_contents = std::fs::read("input.txt")?;

    let puzzle_input = std::str::from_utf8(&file_contents)?
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();

    let engine_numbers = find_engine_numbers(&puzzle_input)?;

    println!("{}", part1(&puzzle_input, &engine_numbers));

    println!("{}", part2(&puzzle_input, &engine_numbers));

    Ok(())
}
