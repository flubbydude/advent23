#[derive(Debug, Clone, Copy)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
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
        let instr = line.rsplit_once('#').unwrap().1.rsplit_once(')').unwrap().0;

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

#[derive(Debug, Default, Clone)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn new(x: isize, y: isize) -> Self {
        Position { x, y }
    }

    fn move_in_direction(&self, direction: Direction, length: usize) -> Self {
        match direction {
            Direction::Right => Position::new(self.x + length as isize, self.y),
            Direction::Down => Position::new(self.x, self.y + length as isize),
            Direction::Left => Position::new(self.x - length as isize, self.y),
            Direction::Up => Position::new(self.x, self.y - length as isize),
        }
    }
}

fn solve(puzzle_input: &[Instruction]) -> usize {
    // shoelace formula:
    let (double_area, border, _) = puzzle_input.iter().fold(
        (0isize, 0, Position::default()),
        |(double_area, border, vertex), &Instruction { direction, length }| {
            let next_vertex = vertex.move_in_direction(direction, length);

            let double_area = double_area + (vertex.x * next_vertex.y - next_vertex.x * vertex.y);
            let border = border + length;

            (double_area, border, next_vertex)
        },
    );

    // pick's theorem:
    // i = inside squares, b = border squares
    // A = i + b/2 - 1
    // therefore i + b = A + b/2 + 1
    // since we want i + b (number of squares that are #)
    // then we need A + b/2 + 1
    // the way we calculated area above either finds positive or negative area so must abs
    (double_area.unsigned_abs() + border) / 2 + 1
}

fn main() {
    let file_contents = std::fs::read_to_string("input.txt").unwrap();

    println!("{}", solve(&parse_input_part1(&file_contents)));
    println!("{}", solve(&parse_input_part2(&file_contents)));
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
    fn test_part1() {
        let puzzle_input = parse_input_part1(TEST_INPUT);

        assert_eq!(solve(&puzzle_input), 62);
    }

    #[test]
    fn test_part2() {
        let puzzle_input = parse_input_part2(TEST_INPUT);

        assert_eq!(solve(&puzzle_input), 952408144115);
    }
}
