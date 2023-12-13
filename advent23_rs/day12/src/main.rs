#[derive(Clone)]
struct RecordRow {
    damaged_record: Vec<u8>,
    num_contiguous: Vec<usize>,
}

impl From<&str> for RecordRow {
    fn from(line: &str) -> Self {
        let (damaged_record_str, num_contiguous_str) = line.split_once(' ').unwrap();

        let damaged_record_bytes = damaged_record_str.as_bytes();

        let mut damaged_record = Vec::with_capacity(damaged_record_bytes.len());
        damaged_record.extend(damaged_record_bytes);

        let num_contiguous = num_contiguous_str
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();

        RecordRow {
            damaged_record,
            num_contiguous,
        }
    }
}

impl RecordRow {
    fn num_ways_helper(
        &self,
        record_index: usize,
        contiguous_index: usize,
        prev_damaged: usize,
    ) -> usize {
        if record_index == self.damaged_record.len() {
            let is_valid = if contiguous_index == self.num_contiguous.len() {
                prev_damaged == 0
            } else {
                contiguous_index + 1 == self.num_contiguous.len()
                    && self.num_contiguous[contiguous_index] == prev_damaged
            };

            return is_valid as usize;
        }

        let handle_damaged = || {
            if contiguous_index >= self.num_contiguous.len()
                || prev_damaged + 1 > self.num_contiguous[contiguous_index]
            {
                0
            } else {
                self.num_ways_helper(record_index + 1, contiguous_index, prev_damaged + 1)
            }
        };

        let handle_empty = || {
            if prev_damaged == 0 {
                self.num_ways_helper(record_index + 1, contiguous_index, 0)
            } else if contiguous_index == self.num_contiguous.len()
                || self.num_contiguous[contiguous_index] != prev_damaged
            {
                0
            } else {
                // move 1 forward in the contiguous index
                self.num_ways_helper(record_index + 1, contiguous_index + 1, 0)
            }
        };

        match self.damaged_record[record_index] {
            b'#' => handle_damaged(),
            b'.' => handle_empty(),
            b'?' => handle_damaged() + handle_empty(),
            _ => panic!("Input contains an unexpected character."),
        }
    }

    fn num_ways(&self) -> usize {
        // greedy recursive time :(
        // prev_damaged is the number I have seen in a row that are damaged
        // if at the end of the record
        self.num_ways_helper(0, 0, 0)
    }

    fn unfolded(&self) -> Self {
        let mut result = self.clone();
        result
            .damaged_record
            .reserve_exact(result.damaged_record.len() * 4 + 4);
        result
            .num_contiguous
            .reserve_exact(result.num_contiguous.len() * 4);

        for _ in 0..4 {
            result.damaged_record.push(b'?');
            result.damaged_record.extend(&self.damaged_record);
            result.num_contiguous.extend(&self.num_contiguous);
        }

        result
    }
}

fn part1(puzzle_input: &[RecordRow]) -> usize {
    puzzle_input.iter().map(RecordRow::num_ways).sum::<usize>()
}

fn part2(puzzle_input: &[RecordRow]) -> usize {
    puzzle_input
        .iter()
        .map(|row| row.unfolded().num_ways())
        .sum::<usize>()
}

fn parse_input(input: &str) -> Vec<RecordRow> {
    input.lines().map(RecordRow::from).collect()
}

fn main() {
    let file_contents = std::fs::read("input.txt").unwrap();
    let file_contents_str = std::str::from_utf8(&file_contents).unwrap();

    let puzzle_input = parse_input(file_contents_str);

    println!("{}", part1(&puzzle_input));
    println!("{}", part2(&puzzle_input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "???.### 1,1,3\n\
                              .??..??...?##. 1,1,3\n\
                              ?#?#?#?#?#?#?#? 1,3,1,6\n\
                              ????.#...#... 4,1,1\n\
                              ????.######..#####. 1,6,5\n\
                              ?###???????? 3,2,1";

    #[test]
    fn test_part_1() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 21);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 525152);
    }
}
