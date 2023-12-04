import re

from aocd import get_data
from typing import Iterator, NamedTuple
from math import prod
from collections import defaultdict

DAY = 3

class EngineNumber(NamedTuple):
    row_index: int
    start: int
    end: int
    number: int

    def get_neighbors(self, num_rows: int, num_cols: int) -> Iterator[tuple[int, int]]:
        on_left = self.start == 0
        on_right = self.end == num_cols

        start_col = self.start
        if not on_left:
            start_col -= 1

        end_col = self.end
        if not on_right:
            end_col += 1

        # add row above
        if self.row_index > 0:
            yield from ((self.row_index - 1, col) for col in range(start_col, end_col))

        # add row below
        if self.row_index < num_rows - 1:
            yield from ((self.row_index + 1, col) for col in range(start_col, end_col))

        if not on_left:
            yield (self.row_index, start_col)

        if not on_right and start_col + 1 != end_col:
            yield (self.row_index, end_col - 1)

    def is_part_number(self, puzzle_input: list[str]):
        num_rows = len(puzzle_input)
        num_cols = len(puzzle_input[0])
        return any(is_symbol(puzzle_input[i][j]) for i, j in self.get_neighbors(num_rows, num_cols))

def is_symbol(c: str):
    return not c.isdigit() and c != '.'

def part1(puzzle_input: list[str], engine_numbers: list[EngineNumber]):
    return sum(en.number for en in engine_numbers if en.is_part_number(puzzle_input))

def part2(puzzle_input: list[str], engine_numbers: list[EngineNumber]):
    num_rows = len(puzzle_input)
    num_cols = len(puzzle_input[0])

    d: defaultdict[tuple[int, int], list[int]] = defaultdict(list)

    for engine_number in engine_numbers:
        for i, j in engine_number.get_neighbors(num_rows, num_cols):
            if puzzle_input[i][j] == '*':
                d[(i, j)].append(engine_number.number)

    return sum(prod(l) for l in d.values() if len(l) >= 2)

def parse_data(data: str):
    return data.splitlines()

def find_engine_numbers(puzzle_input: list[str]):
    result: list[EngineNumber] = []
    for i, line in enumerate(puzzle_input):
        for num_match in re.finditer(r'[0-9]+', line):
            result.append(EngineNumber(i, num_match.start(), num_match.end(), int(num_match.group())))

    return result

def main(data: str):
    puzzle_input = parse_data(data)

    engine_numbers = find_engine_numbers(puzzle_input)

    print(part1(puzzle_input, engine_numbers))
    print(part2(puzzle_input, engine_numbers))

if __name__ == '__main__':
    main(get_data(day=DAY, year=2023))

