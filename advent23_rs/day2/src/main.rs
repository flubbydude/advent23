use std::convert::From;

#[derive(Debug)]
struct Reveal {
    red: i32,
    green: i32,
    blue: i32,
}

impl Reveal {
    fn power(&self) -> i32 {
        self.red * self.green * self.blue
    }

    fn part1_possible(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }
}

impl From<&[u8]> for Reveal {
    fn from(reveal_str: &[u8]) -> Self {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for num_and_color in reveal_str.split(|&c| c == b',') {
            let mut iter = num_and_color
                .split(|&c| c == b' ')
                .filter(|s| !s.is_empty());

            let num: i32 = std::str::from_utf8(iter.next().unwrap())
                .unwrap()
                .parse()
                .unwrap();

            let color = iter.next().unwrap();

            match color {
                b"red" => red = num,
                b"green" => green = num,
                b"blue" => blue = num,
                _ => panic!(),
            }
        }

        Reveal { red, green, blue }
    }
}

#[derive(Debug)]
struct Game {
    reveals: Vec<Reveal>,
}

impl Game {
    fn part1_possible(&self) -> bool {
        self.reveals.iter().all(|reveal| reveal.part1_possible())
    }

    fn get_min_possible_counts(&self) -> Reveal {
        Reveal {
            red: self.reveals.iter().map(|r| r.red).max().unwrap(),
            green: self.reveals.iter().map(|r| r.green).max().unwrap(),
            blue: self.reveals.iter().map(|r| r.blue).max().unwrap(),
        }
    }
}

impl From<&[u8]> for Game {
    fn from(line: &[u8]) -> Self {
        let reveals_str = line.split(|&c| c == b':').nth(1).unwrap();

        Game {
            reveals: reveals_str
                .split(|&c| c == b';')
                .map(Reveal::from)
                .collect(),
        }
    }
}

fn parse_puzzle_input(puzzle_input: Vec<u8>) -> Vec<Game> {
    let mut result = Vec::new();
    for line in puzzle_input.split(|&c| c == b'\n') {
        if line.is_empty() {
            continue;
        }
        result.push(Game::from(line))
    }

    result
}

fn part1(games: &[Game]) -> i32 {
    games
        .iter()
        .enumerate()
        .filter_map(|(i, game)| {
            if game.part1_possible() {
                Some((i + 1) as i32)
            } else {
                None
            }
        })
        .sum()
}

fn part2(games: &[Game]) -> i32 {
    games
        .iter()
        .map(|game| game.get_min_possible_counts().power())
        .sum()
}

fn main() -> Result<(), std::io::Error> {
    let file_contents = std::fs::read("input.txt")?;

    let games = parse_puzzle_input(file_contents);

    println!("{}", part1(&games));
    println!("{}", part2(&games));

    Ok(())
}
