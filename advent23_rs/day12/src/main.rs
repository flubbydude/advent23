fn count_ways(line: &str) -> usize {
    todo!()
}

fn part1(puzzle_input: &str) -> usize {
    puzzle_input.lines().map(count_ways).sum::<usize>()
}

fn main() {
    let file_contents = std::fs::read("input.txt").unwrap();
    let puzzle_input = std::str::from_utf8(&file_contents).unwrap();

    println!("{}", part1(puzzle_input));
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
        assert_eq!(part1(TEST_INPUT), 21);
    }
}
