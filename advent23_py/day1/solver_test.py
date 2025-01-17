import unittest
from solver import (
    parse_data,
    part1,
    part2,
    part2_re,
    DIGIT_NUM_STRS,
    DAY,
    process_line,
    process_line_re,
)
from aocd import get_data

PART1_TEST_DATA = """1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"""

PART2_TEST_DATA = """two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"""


class Test(unittest.TestCase):
    def test_part1(self):
        part1_input = parse_data(PART1_TEST_DATA)

        self.assertEqual(part1(part1_input), 142)

    def test_digit_num_strs(self):
        self.assertListEqual(
            DIGIT_NUM_STRS, ["1", "2", "3", "4", "5", "6", "7", "8", "9"]
        )

    def test_part2(self):
        part2_input = parse_data(PART2_TEST_DATA)

        self.assertEqual(part2(part2_input), 281)

    def test_part2_re(self):
        part2_input = parse_data(PART2_TEST_DATA)

        self.assertEqual(part2_re(part2_input), 281)

    def test_part2_re_comparitively(self):
        part2_input = parse_data(get_data(day=DAY, year=2023))

        for line in part2_input:
            self.assertEqual(process_line(
                line), process_line_re(line), f"{line = }")


if __name__ == "__main__":
    unittest.main()
