from aocd import get_data
from typing import NamedTuple
from math import prod

DAY = 2


class Reveal(NamedTuple):
    red: int = 0
    green: int = 0
    blue: int = 0

    def power(self):
        return prod(self)


Game = list[Reveal]

MAX_REVEAL_PART1 = Reveal(12, 13, 14)

######################## PARSING ########################


def parse_data(data: str):
    return [parse_game(line) for line in data.splitlines()]


def parse_game(line: str) -> Game:
    return [parse_reveal(reveal_str) for reveal_str in line.split(':')[1].split(';')]


def parse_reveal(reveal_str: str):
    # num_and_color == "red 3" => updates result to have red as 3
    kwargs: dict[str, int] = {}
    for num_and_color in reveal_str.split(','):
        num_str, color = num_and_color.split()
        kwargs[color] = int(num_str)

    return Reveal(**kwargs)


######################### PART 1 #########################

def possible_part1(game: Game):
    return all(x <= y for reveal in game for x, y in zip(reveal, MAX_REVEAL_PART1))


def part1(games: list[Game]):
    return sum(i for i, game in enumerate(games, 1) if possible_part1(game))


######################### PART 2 #########################

def get_min_possible(game: Game):
    return Reveal(*(max(r[i] for r in game) for i in range(len(Reveal._fields))))


def part2(games: list[Game]):
    return sum(get_min_possible(game).power() for game in games)


#########################  MAIN  #########################

def main(data: str):
    games = parse_data(data)

    print(part1(games))
    print(part2(games))


if __name__ == '__main__':
    main(get_data(day=DAY, year=2023))
