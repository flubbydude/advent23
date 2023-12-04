import unittest
from solver import main, part1, part2, find_engine_numbers

PART1_TEST_DATA = """467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."""
PART1_SOLUTION = 4361

PART2_TEST_DATA = """467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."""
PART2_SOLUTION = 467835


class Test(unittest.TestCase):

    def test_part1(self):
        puzzle_input: list[str] = PART1_TEST_DATA.splitlines()
        engine_numbers = find_engine_numbers(puzzle_input)

        self.assertEqual(part1(puzzle_input, engine_numbers), PART1_SOLUTION)

    def test_part2(self):
        puzzle_input: list[str] = PART2_TEST_DATA.splitlines()
        engine_numbers = find_engine_numbers(puzzle_input)

        self.assertEqual(part2(puzzle_input, engine_numbers), PART2_SOLUTION)

if __name__ == '__main__':
    unittest.main()