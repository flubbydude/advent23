use array2d::Array2D;
use std::array;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, EnumIter)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    fn was_right_turn(&self, last_direction: Direction) -> bool {
        use Direction::*;
        match last_direction {
            Right => matches!(self, Down),
            Down => matches!(self, Left),
            Left => matches!(self, Up),
            Up => matches!(self, Right),
        }
    }

    fn turn_right(&self) -> Self {
        use Direction::*;
        match self {
            Right => Down,
            Down => Left,
            Left => Up,
            Up => Right,
        }
    }

    fn turn_left(&self) -> Self {
        use Direction::*;
        match self {
            Right => Up,
            Down => Right,
            Left => Down,
            Up => Left,
        }
    }
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        use Direction::*;
        match value {
            'R' => Right,
            'D' => Down,
            'L' => Left,
            'U' => Up,
            _ => panic!("Unable to parse direction from '{}'", value),
        }
    }
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        assert_eq!(value.len(), 1);
        value.chars().next().unwrap().into()
    }
}

impl From<u32> for Direction {
    fn from(value: u32) -> Self {
        use Direction::*;
        match value {
            0 => Right,
            1 => Down,
            2 => Left,
            3 => Up,
            _ => panic!("Unable to parse direction from {}u32", value),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    length: usize,
}

impl From<&str> for Instruction {
    fn from(line: &str) -> Self {
        let mut iter = line.split_ascii_whitespace();

        let direction = iter.next().unwrap().into();

        let length = iter.next().unwrap().parse().unwrap();

        Instruction { direction, length }
    }
}

impl Instruction {
    fn from_part2(line: &str) -> Self {
        // ex:
        // line = "R 6 (#70c710)""
        // instr = "70c710" => parse into Instruction { direction: Direction::Right, length: 461937 }
        let instr = line.rsplit_once(' ').unwrap().1;

        // remove last
        let mut hex_chars = instr.chars();
        let direction_digit = hex_chars.next_back().unwrap();

        let direction = direction_digit.to_digit(4).unwrap().into();
        let length = usize::from_str_radix(hex_chars.as_str(), 16).unwrap();

        Instruction { direction, length }
    }
}

fn parse_input_part1(file_contents: &str) -> Vec<Instruction> {
    file_contents.lines().map(Instruction::from).collect()
}

fn parse_input_part2(file_contents: &str) -> Vec<Instruction> {
    file_contents.lines().map(Instruction::from_part2).collect()
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
struct Position {
    row: isize,
    column: isize,
}

impl Position {
    fn new(row: isize, column: isize) -> Self {
        Position { row, column }
    }

    fn move_in_direction(&self, direction: Direction, length: usize) -> Self {
        match direction {
            Direction::Right => Position::new(self.row, self.column + length as isize),
            Direction::Down => Position::new(self.row + length as isize, self.column),
            Direction::Left => Position::new(self.row, self.column - length as isize),
            Direction::Up => Position::new(self.row - length as isize, self.column),
        }
    }
}

fn flood_fill_with_pound(array: &mut Array2D<u8>, position: Position) {
    let mut stack = Vec::from([position]);

    while let Some(position) = stack.pop() {
        if position.row < 0
            || position.row as usize >= array.num_rows()
            || position.column < 0
            || position.column as usize >= array.num_columns()
            || array[(position.row as usize, position.column as usize)] == b'#'
        {
            continue;
        }

        array[(position.row as usize, position.column as usize)] = b'#';

        for direction in Direction::iter() {
            stack.push(position.move_in_direction(direction, 1));
        }
    }
}

fn part1(puzzle_input: &[Instruction]) -> usize {
    let mut corners = Vec::with_capacity(puzzle_input.len());
    corners.push(Position::default());

    let mut num_lefts = 0;
    let mut num_rights = 0;

    let mut maybe_last_dir = None;

    for instruction in puzzle_input.split_last().unwrap().1 {
        let next_corner = corners
            .last()
            .unwrap()
            .move_in_direction(instruction.direction, instruction.length);

        corners.push(next_corner);

        if let Some(last_direction) = maybe_last_dir {
            if instruction.direction.was_right_turn(last_direction) {
                num_rights += 1;
            } else {
                num_lefts += 1;
            }
        }

        maybe_last_dir = Some(instruction.direction);
    }

    let minimum_row = corners
        .iter()
        .map(|&Position { row, .. }| row)
        .min()
        .unwrap();

    let minimum_column = corners
        .iter()
        .map(|&Position { column, .. }| column)
        .min()
        .unwrap();

    let maximum_row = corners
        .iter()
        .map(|&Position { row, .. }| row)
        .max()
        .unwrap();

    let maximum_column = corners
        .iter()
        .map(|&Position { column, .. }| column)
        .max()
        .unwrap();

    let num_rows = (maximum_row - minimum_row + 1) as usize;
    let num_columns = (maximum_column - minimum_column + 1) as usize;

    let mut matrix = Array2D::filled_with(b'.', num_rows, num_columns);

    let start_pos = Position {
        row: -minimum_row,
        column: -minimum_column,
    };

    let mut cur_pos = start_pos.clone();

    for instruction in puzzle_input {
        for _ in 0..instruction.length {
            matrix[(cur_pos.row as usize, cur_pos.column as usize)] = b'#';
            cur_pos = cur_pos.move_in_direction(instruction.direction, 1);
        }
    }

    let inside_on_right = num_rights > num_lefts;

    let mut cur_pos = start_pos;

    for instruction in puzzle_input {
        for _ in 0..instruction.length {
            flood_fill_with_pound(
                &mut matrix,
                cur_pos.move_in_direction(
                    if inside_on_right {
                        instruction.direction.turn_right()
                    } else {
                        instruction.direction.turn_left()
                    },
                    1,
                ),
            );

            cur_pos = cur_pos.move_in_direction(instruction.direction, 1);
        }
    }

    let matrix = matrix;

    matrix
        .elements_row_major_iter()
        .filter(|&&c| c == b'#')
        .count()
}

fn main() {
    let file_contents = std::fs::read_to_string("input.txt").unwrap();

    let puzzle_input = parse_input_part1(&file_contents);

    println!("{}", part1(&puzzle_input));
    // println!("{}", part2(&puzzle_input));
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = "R 6 (#70c710)\n\
                              D 5 (#0dc571)\n\
                              L 2 (#5713f0)\n\
                              D 2 (#d2c081)\n\
                              R 2 (#59c680)\n\
                              D 2 (#411b91)\n\
                              L 5 (#8ceee2)\n\
                              U 2 (#caa173)\n\
                              L 1 (#1b58a2)\n\
                              U 2 (#caa171)\n\
                              R 2 (#7807d2)\n\
                              U 3 (#a77fa3)\n\
                              L 2 (#015232)\n\
                              U 2 (#7a21e3)";

    #[test]
    fn test_part_1() {
        let puzzle_input = parse_input_part1(TEST_INPUT);

        assert_eq!(part1(&puzzle_input), 62);
    }

    // #[test]
    // fn test_part_2() {
    //     let puzzle_input = parse_input(TEST_INPUT);

    //     assert_eq!(part2(&puzzle_input), 94);
    // }
}
