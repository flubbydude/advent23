import unittest
from solver import main, parse_data, part1, part2

PART1_TEST_DATA = ""
PART1_SOLUTION = 142

PART2_TEST_DATA = ""
PART2_SOLUTION = 281


class Test(unittest.TestCase):

    def test_part1(self):
        part1_input = parse_data(PART1_TEST_DATA)

        self.assertEquals(part1(part1_input), PART1_SOLUTION)

    def test_part2(self):
        part2_input = parse_data(PART2_TEST_DATA)

        self.assertEquals(part2(part2_input), PART2_SOLUTION)

if __name__ == '__main__':
    unittest.main()