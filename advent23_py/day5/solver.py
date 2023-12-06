from aocd import get_data
from typing import NamedTuple, overload
from bisect import bisect_left
import sys

DAY = 5


class Range(NamedTuple):
    start: int
    length: int

    @property
    def end(self):
        return self.start + self.length


class MapRange(NamedTuple):
    src_start: int
    length: int
    dest_start: int

    @property
    def src_end(self):
        return self.src_start + self.length

    def src_to_dest(self, src_val: int) -> int:
        if src_val < self.src_start or src_val >= self.src_end:
            return src_val

        return src_val - self.src_start + self.dest_start

    def has_overlap(self, src_range: Range):
        return (src_range.start <= self.src_start < src_range.end) or (self.src_start <= src_range.start < self.src_end)

    # assumes ranges overlap
    # returns overlap range, before range, after range
    def src_range_to_dest_ranges(self, src_range: Range) -> tuple[Range, Range | None, Range | None]:
        # get overlap range
        start = max(src_range.start, self.src_start)
        end = min(src_range.end, self.src_end)
        result = Range(start + self.dest_start - self.src_start, end - start)

        # sv start is before mr start -> add the range from before it
        if src_range.start < self.src_start:
            before_range = Range(
                src_range.start, self.src_start - src_range.start)
        else:
            before_range = None

        # mr end is before sv end
        if self.src_end < src_range.end:
            after_range = Range(self.src_end, src_range.end - self.src_end)
        else:
            after_range = None

        return result, before_range, after_range


def parse_data(data: str):
    lines = data.splitlines()
    seeds = [int(s) for s in lines[0].split(':', 1)[1].split()]

    new_map_next = True
    maps: list[list[MapRange]] = []

    for line in lines[2:]:
        if not line:
            new_map_next = True
        elif new_map_next:
            # line is:
            # x-to-y map:
            maps.append([])
            new_map_next = False
        else:
            # line is:
            # [dest start] [source start] [range len]
            ds_str, ss_str, len_str = line.split()

            maps[-1].append(MapRange(int(ss_str), int(len_str), int(ds_str)))

    for map in maps:
        map.sort()

    return seeds, maps


def part1(seeds: list[int], maps: list[list[MapRange]]):
    best = sys.maxsize  # treated like infinity

    for seed in seeds:
        src_val = seed
        for map_ranges in maps:
            # find src_elem in the map range
            i = bisect_left(map_ranges, (src_val, ))
            if i == len(map_ranges) or map_ranges[i].src_start > src_val:
                if i == 0:
                    continue

                i -= 1

            src_val = map_ranges[i].src_to_dest(src_val)

        best = min(src_val, best)

    return best


def part2(seed_vals: list[int], maps: list[list[MapRange]]):
    seed_ranges = [Range(seed_vals[i], seed_vals[i + 1])
                   for i in range(0, len(seed_vals), 2)]

    best = sys.maxsize  # treated like infinity

    for seed_range in seed_ranges:
        src_ranges = [seed_range]
        for map_ranges in maps:
            new_src_ranges: list[Range] = []

            for src_range in src_ranges:
                for map_range in map_ranges:
                    if src_range.end <= map_range.src_start:
                        new_src_ranges.append(src_range)
                        break

                    if map_range.has_overlap(src_range):
                        overlap_range, before_range, after_range = map_range.src_range_to_dest_ranges(
                            src_range)

                        if before_range is not None:
                            new_src_ranges.append(before_range)

                        new_src_ranges.append(overlap_range)

                        if after_range is not None:
                            src_range = after_range
                        else:
                            break
                else:
                    # no range overlap found and did not break
                    new_src_ranges.append(src_range)

            src_ranges = new_src_ranges

        best = min(best, min(src_range.start for src_range in src_ranges))

    return best


def main(data: str):
    seeds, maps = parse_data(data)

    print(part1(seeds, maps))
    print(part2(seeds, maps))


if __name__ == '__main__':
    main(get_data(day=DAY, year=2023))
