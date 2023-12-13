use std::collections::HashMap;

#[derive(Clone)]
struct RecordRow {
    damaged_record: Vec<u8>,
    num_contiguous: Vec<u32>,
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
    fn _num_ways_recursive_helper(
        &self,
        record_index: usize,
        contiguous_index: usize,
        prev_damaged: u32,
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
                self._num_ways_recursive_helper(
                    record_index + 1,
                    contiguous_index,
                    prev_damaged + 1,
                )
            }
        };

        let handle_operational = || {
            if prev_damaged == 0 {
                self._num_ways_recursive_helper(record_index + 1, contiguous_index, 0)
            } else if contiguous_index == self.num_contiguous.len()
                || self.num_contiguous[contiguous_index] != prev_damaged
            {
                0
            } else {
                // move 1 forward in the contiguous index
                self._num_ways_recursive_helper(record_index + 1, contiguous_index + 1, 0)
            }
        };

        match self.damaged_record[record_index] {
            b'#' => handle_damaged(),
            b'.' => handle_operational(),
            b'?' => handle_damaged() + handle_operational(),
            _ => panic!("Input contains an unexpected character."),
        }
    }

    fn _num_ways_recursive(&self) -> usize {
        self._num_ways_recursive_helper(0, 0, 0)
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

    fn num_ways(&self) -> usize {
        let mut num_contiguous_end_damaged: HashMap<Vec<u32>, usize> = HashMap::new();
        let mut num_contiguous_end_operational: HashMap<Vec<u32>, usize> =
            HashMap::from([(vec![], 1)]);

        let handle_damaged =
            |prev_damaged: HashMap<Vec<u32>, usize>, prev_operational: HashMap<Vec<u32>, usize>| {
                let mut result = prev_damaged
                    .into_iter()
                    .filter_map(|(mut comb_end_damaged, num_ways)| {
                        // should always be true that this vec is not empty!
                        let i = comb_end_damaged.len() - 1;
                        let last_elem = comb_end_damaged.last_mut().unwrap();

                        *last_elem += 1;

                        // new combinations is too big!
                        if *last_elem > self.num_contiguous[i] {
                            None
                        } else {
                            Some((comb_end_damaged, num_ways))
                        }
                    })
                    .collect::<HashMap<_, _>>();

                for (mut comb_end_operational, num_ways) in prev_operational {
                    // skip this element; can't add another contiguous to it bc it would be too many separate groups.
                    if comb_end_operational.len() == self.num_contiguous.len() {
                        continue;
                    }

                    comb_end_operational.push(1);

                    result
                        .entry(comb_end_operational)
                        .and_modify(|val| *val += num_ways)
                        .or_insert(num_ways);
                }

                result
            };

        for c in self.damaged_record.iter() {
            match c {
                b'#' => {
                    num_contiguous_end_damaged =
                        handle_damaged(num_contiguous_end_damaged, num_contiguous_end_operational);

                    num_contiguous_end_operational = HashMap::new();
                }
                b'.' => {
                    for (comb_end_damaged, num_ways) in num_contiguous_end_damaged {
                        num_contiguous_end_operational
                            .entry(comb_end_damaged)
                            .and_modify(|val| *val += num_ways)
                            .or_insert(num_ways);
                    }

                    num_contiguous_end_damaged = HashMap::new();
                }
                b'?' => {
                    let mut new_operational = num_contiguous_end_operational.clone();

                    for (comb_end_damaged, &num_ways) in num_contiguous_end_damaged.iter() {
                        if let Some(val) = new_operational.get_mut(comb_end_damaged) {
                            *val += num_ways;
                        } else {
                            new_operational.insert(comb_end_damaged.clone(), num_ways);
                        }
                    }

                    num_contiguous_end_damaged =
                        handle_damaged(num_contiguous_end_damaged, num_contiguous_end_operational);

                    num_contiguous_end_operational = new_operational;
                }
                _ => panic!(),
            }
        }

        num_contiguous_end_damaged
            .get(&self.num_contiguous)
            .unwrap_or(&0)
            + num_contiguous_end_operational
                .get(&self.num_contiguous)
                .unwrap_or(&0)
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
