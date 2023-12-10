use num::PrimInt;

trait History<T: PrimInt> {
    fn find_next_value(&self) -> T;
    fn find_prev_value(&self) -> T;
}

impl<T: PrimInt> History<T> for Vec<T> {
    fn find_next_value(&self) -> T {
        if self.iter().all(|x| x.is_zero()) {
            return T::zero();
        }

        let next_layer_val = self
            .windows(2)
            .map(|arr| arr[1] - arr[0])
            .collect::<Vec<_>>()
            .find_next_value();

        self[self.len() - 1] + next_layer_val
    }

    fn find_prev_value(&self) -> T {
        if self.iter().all(|x| x.is_zero()) {
            return T::zero();
        }

        let next_layer_val = self
            .windows(2)
            .map(|arr| arr[1] - arr[0])
            .collect::<Vec<_>>()
            .find_prev_value();

        self[0] - next_layer_val
    }
}

fn parse_history(line: &str) -> Vec<i32> {
    line.split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn part1(puzzle_input: &str) -> i32 {
    puzzle_input
        .lines()
        .map(parse_history)
        .map(|history| history.find_next_value())
        .sum()
}

fn part2(puzzle_input: &str) -> i32 {
    puzzle_input
        .lines()
        .map(parse_history)
        .map(|history| history.find_prev_value())
        .sum()
}

fn main() {
    let file_contents = std::fs::read("input.txt").unwrap();
    let puzzle_input = std::str::from_utf8(&file_contents).unwrap();

    println!("{}", part1(puzzle_input));
    println!("{}", part2(puzzle_input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "0 3 6 9 12 15\n\
                              1 3 6 10 15 21\n\
                              10 13 16 21 30 45";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 114);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 2);
    }
}
