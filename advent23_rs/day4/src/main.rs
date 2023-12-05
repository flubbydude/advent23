use std::error;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn get_num_of_winning_numbers(line: &str) -> usize {
    let mut set = [false; 100];

    let all_nums = line.split_once(':').unwrap().1;

    let (winning_nums_str, your_nums_str) = all_nums.split_once('|').unwrap();

    for num in winning_nums_str
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
    {
        set[num] = true;
    }

    your_nums_str
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .filter(|&num| set[num])
        .count()
}

fn score_line_part1(line: &str) -> i32 {
    let result = get_num_of_winning_numbers(line);

    if result == 0 {
        0
    } else {
        1 << (result - 1)
    }
}

fn part1(puzzle_input: &[&str]) -> i32 {
    puzzle_input.iter().map(|line| score_line_part1(line)).sum()
}

fn part2(puzzle_input: &[&str]) -> i32 {
    let mut card_amts = vec![1; puzzle_input.len()];

    for (i, line) in puzzle_input.iter().enumerate() {
        let result = get_num_of_winning_numbers(line);
        for j in 1..=result {
            card_amts[i + j] += card_amts[i];
        }
    }

    card_amts.iter().sum()
}

fn main() -> Result<()> {
    let file_contents = std::fs::read("input.txt")?;

    let puzzle_input = std::str::from_utf8(&file_contents)?
        .lines()
        .collect::<Vec<_>>();

    println!("{}", part1(&puzzle_input));
    println!("{}", part2(&puzzle_input));

    Ok(())
}
