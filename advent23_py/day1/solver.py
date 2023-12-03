from aocd import get_data

DAY = 1

DIGIT_NUM_STRS = [str(i) for i in range(1, 10)]
DIGIT_WORD_STRS = ['one', 'two', 'three', 'four', 'five', 'six', 'seven', 'eight', 'nine']
DIGIT_STRS = DIGIT_NUM_STRS + DIGIT_WORD_STRS

def part1(inp: list[str]):
    result = 0

    for line in inp:
        first_digit = next(c for c in line if c.isdigit())
        last_digit = next(c for c in reversed(line) if c.isdigit())

        num = int(first_digit + last_digit)
        result += num

    return result


def part2(inp: list[str]):
    result = 0

    for line in inp:
        first_appearances = (line.find(num_str) for num_str in DIGIT_STRS)
        last_appearances = (line.rfind(num_str) for num_str in DIGIT_STRS)

        _, first_index = min((j, i) for i, j in enumerate(first_appearances) if j != -1)
        _, last_index = max((j, i) for i, j in enumerate(last_appearances) if j != -1)

        if first_index >= 9:
            first_index -= 9
        
        if last_index >= 9:
            last_index -= 9

        result += int(DIGIT_STRS[first_index] + DIGIT_STRS[last_index])

    return result

def parse_data(data: str):
    return data.splitlines()

def main(data: str):
    inp = parse_data(data)

    print(part1(inp))
    print(part2(inp))

if __name__ == '__main__':
    main(get_data(day=DAY, year=2023))

