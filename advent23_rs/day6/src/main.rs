use std::ops::RangeInclusive;

struct Race {
    time: usize,
    record: usize,
}

impl Race {
    // need to solve (hold time) * ((time limit) - (hold time)) > record
    // in other words, -x^2 + (time limit) * x - record > 0
    // assume f64 does not cause an error, since numbers are a good size
    fn compute_elite_hold_times(&self) -> Option<RangeInclusive<usize>> {
        // solves -x^2 + time_limit * x - record > 0
        let time_f = self.time as f64;

        let determinant = time_f * time_f - 4.0 * self.record as f64;
        if determinant < 0.0 {
            return None;
        }
        let sqrt_determinant = determinant.sqrt();
        let xmin_f = (time_f - sqrt_determinant) / 2.0;
        let xmax_f = (time_f + sqrt_determinant) / 2.0;

        let mut xmin = xmin_f.ceil() as usize;
        let mut xmax = xmax_f.floor() as usize;

        if xmin * (self.time - xmin) <= self.record {
            xmin += 1;
        }

        if xmax * (self.time - xmax) <= self.record {
            xmax -= 1;
        }

        if xmin <= xmax {
            Some(xmin..=xmax)
        } else {
            None
        }
    }

    fn num_elite_hold_times(&self) -> usize {
        match self.compute_elite_hold_times() {
            Some(range) => range.count(),
            None => 0,
        }
    }
}

fn parse_input_part1(puzzle_input: &str) -> Vec<Race> {
    let mut lines = puzzle_input.lines();

    let times = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap());

    let records = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap());

    times
        .zip(records)
        .map(|(time, record)| Race { time, record })
        .collect()
}

fn parse_input_part2(puzzle_input: &str) -> Race {
    let mut lines = puzzle_input.lines();
    let mut time = 0;
    for c in lines.next().unwrap().chars() {
        if let Some(digit) = c.to_digit(10) {
            time *= 10;
            time += digit as usize;
        }
    }

    let mut record = 0;
    for c in lines.next().unwrap().chars() {
        if let Some(digit) = c.to_digit(10) {
            record *= 10;
            record += digit as usize;
        }
    }

    Race { time, record }
}

fn part1(races: &[Race]) -> usize {
    races.iter().map(Race::num_elite_hold_times).product()
}

fn part2(race: &Race) -> usize {
    race.num_elite_hold_times()
}

fn main() {
    let puzzle_input = "Time:        56     71     79     99\n\
                              Distance:   334   1135   1350   2430";

    println!("{}", part1(&parse_input_part1(puzzle_input)));
    println!("{}", part2(&parse_input_part2(puzzle_input)));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Time:      7  15   30\n\
                              Distance:  9  40  200";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_input_part1(TEST_INPUT)), 288);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse_input_part2(TEST_INPUT)), 71503);
    }
}
