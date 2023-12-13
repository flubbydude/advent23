use std::iter::repeat;

#[derive(Clone)]
struct RecordRow {
    damaged_record: Box<[u8]>,
    num_contiguous: Box<[usize]>,
}

impl From<&str> for RecordRow {
    fn from(line: &str) -> Self {
        let (damaged_record_str, num_contiguous_str) = line.split_once(' ').unwrap();

        let damaged_record_bytes = damaged_record_str.as_bytes();

        let mut damaged_record = Vec::with_capacity(damaged_record_bytes.len());
        damaged_record.extend(damaged_record_bytes);

        let damaged_record = damaged_record.into_boxed_slice();

        let num_contiguous = num_contiguous_str
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect::<Vec<usize>>()
            .into_boxed_slice();

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
        let mut damaged_record = Vec::with_capacity(self.damaged_record.len() * 5 + 4);
        let mut num_contiguous = Vec::with_capacity(self.num_contiguous.len() * 5);

        for i in 0..5 {
            if i != 0 {
                damaged_record.push(b'?');
            }
            damaged_record.extend_from_slice(&self.damaged_record);
            num_contiguous.extend_from_slice(&self.num_contiguous);
        }

        RecordRow {
            damaged_record: damaged_record.into_boxed_slice(),
            num_contiguous: num_contiguous.into_boxed_slice(),
        }
    }

    fn num_ways(&self) -> usize {
        let num_damaged: usize = self.num_contiguous.iter().sum();
        let mut num_contiguous_index_bins = Vec::with_capacity(num_damaged);
        for (i, &count) in self.num_contiguous.iter().enumerate() {
            num_contiguous_index_bins.extend(repeat(i).take(count));
        }

        let num_contiguous_index_bins = num_contiguous_index_bins;

        // println!(
        //     "{:?} => {:?}",
        //     self.num_contiguous, num_contiguous_index_bins
        // );

        let mut num_ways_end_damaged = vec![0; num_damaged + 1];
        let mut num_ways_end_operational = vec![0; num_damaged + 1];

        num_ways_end_operational[0] = 1;

        // num_ways_end_damaged[i] is the number of ways to have
        // i damaged elems at the current point while staying
        // within the criteria (num_contiguous)

        // ".?##.?????#?#?#??## 3,5,1"

        for c in self.damaged_record.iter() {
            match c {
                b'#' => {
                    // bruh i was missing this line for so long:
                    num_ways_end_operational[num_damaged] = 0;
                    for i in (0..num_damaged).rev() {
                        num_ways_end_damaged[i + 1] = num_ways_end_operational[i];
                        num_ways_end_operational[i] = 0;

                        // if in the same bin, meaning that they would be contiguous
                        // then add to the count
                        // otherwise don't!
                        if i > 0 && num_contiguous_index_bins[i] == num_contiguous_index_bins[i - 1]
                        {
                            num_ways_end_damaged[i + 1] += num_ways_end_damaged[i];
                        }
                    }
                }
                b'.' => {
                    for i in 1..=num_damaged {
                        // if at the end of a bin
                        // as in you are at the end of a contiguous group of damaged things
                        if i == num_damaged
                            || (num_contiguous_index_bins[i] != num_contiguous_index_bins[i - 1])
                        {
                            num_ways_end_operational[i] += num_ways_end_damaged[i];
                        }
                        num_ways_end_damaged[i] = 0;
                    }
                }
                b'?' => {
                    let mut new_damaged = Vec::with_capacity(num_damaged + 1);
                    new_damaged.push(0);
                    new_damaged.extend(&num_ways_end_operational[..num_damaged]);

                    for i in 1..=num_damaged {
                        // if at the end of a bin
                        // as in you are at the end of a contiguous group of damaged things
                        if i == num_damaged
                            || (num_contiguous_index_bins[i] != num_contiguous_index_bins[i - 1])
                        {
                            num_ways_end_operational[i] += num_ways_end_damaged[i];
                        }
                    }

                    for i in (0..num_damaged).rev() {
                        // if in the same bin, meaning that they would be contiguous
                        // then add to the count
                        // otherwise don't!
                        if i > 0 && num_contiguous_index_bins[i] == num_contiguous_index_bins[i - 1]
                        {
                            new_damaged[i + 1] += num_ways_end_damaged[i];
                        }
                    }
                    num_ways_end_damaged = new_damaged;
                }
                _ => panic!(),
            }
        }

        // let rv = num_ways_end_damaged[num_damaged] + num_ways_end_operational[num_damaged];
        // println!(
        //     "num ways for {} {:?} = {rv:?}",
        //     std::str::from_utf8(&self.damaged_record).unwrap(),
        //     self.num_contiguous,
        // );
        // rv

        num_ways_end_damaged[num_damaged] + num_ways_end_operational[num_damaged]
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

    #[test]
    fn compare_num_ways() {
        let rr = RecordRow::from(".?##.?????#?#?#??## 3,5,1");

        assert_eq!(rr.num_ways(), rr._num_ways_recursive())
    }
}
