from aocd import get_data

DAY = 7

def part1(inp: list[str]):
    return 142

def part2(inp: list[str]):
    return 281

def parse_data(data: str):
    return data.splitlines()

def main(data: str):
    inp = parse_data(data)

    print(part1(inp))
    print(part2(inp))

if __name__ == '__main__':
    main(get_data(day=DAY, year=2023))

