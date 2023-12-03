from aocd import get_data
import re

DAY = 1

DIGIT_NUM_STRS = [str(i) for i in range(1, 10)]
DIGIT_WORD_STRS = [
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
]
DIGIT_STRS = DIGIT_NUM_STRS + DIGIT_WORD_STRS

DIGIT_STRS_RE = "|".join(DIGIT_STRS)


def part1(inp: list[str]):
    result = 0

    for line in inp:
        first_digit = next(c for c in line if c.isdigit())
        last_digit = next(c for c in reversed(line) if c.isdigit())

        num = int(first_digit + last_digit)
        result += num

    return result


def process_line(line: str):
    first_appearances = (line.find(num_str) for num_str in DIGIT_STRS)
    last_appearances = (line.rfind(num_str) for num_str in DIGIT_STRS)

    _, first_index = min((j, i)
                         for i, j in enumerate(first_appearances) if j != -1)
    _, last_index = max((j, i)
                        for i, j in enumerate(last_appearances) if j != -1)

    if first_index >= 9:
        first_index -= 9

    if last_index >= 9:
        last_index -= 9

    return int(DIGIT_STRS[first_index] + DIGIT_STRS[last_index])


def part2(inp: list[str]):
    return sum(process_line(line) for line in inp)


FIRST_DIGIT_STR_RE = rf".*?({DIGIT_STRS_RE})"
REVERSED_FIRST_DIGIT_STR_RE = rf".*?({DIGIT_STRS_RE[::-1]})"


def process_line_re(line: str):
    start_match = re.match(FIRST_DIGIT_STR_RE, line)
    end_match = re.match(REVERSED_FIRST_DIGIT_STR_RE, line[::-1])

    assert start_match is not None
    assert end_match is not None

    first_digit_str = start_match.group(1)
    last_digit_str = end_match.group(1)

    first_index = DIGIT_STRS.index(first_digit_str)
    last_index = DIGIT_STRS.index(last_digit_str[::-1])

    if first_index >= 9:
        first_index -= 9

    if last_index >= 9:
        last_index -= 9

    return int(DIGIT_STRS[first_index] + DIGIT_STRS[last_index])


def part2_re(inp: list[str]):
    return sum(process_line_re(line) for line in inp)


def parse_data(data: str):
    return data.splitlines()


def main(data: str):
    inp = parse_data(data)

    print(part1(inp))
    print(part2(inp))
    print(part2_re(inp))


if __name__ == "__main__":
    data = get_data(day=DAY, year=2023)
    assert isinstance(data, str)
    main(data)
