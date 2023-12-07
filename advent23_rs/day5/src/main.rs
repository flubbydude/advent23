use std::{
    cmp::{max, min},
    ops::Range,
    usize,
};

// TODO: make this an enum, one for having after
// and one for not having after
#[derive(Debug)]
struct OverlapInfo<T> {
    before: Option<T>,
    overlap: Option<T>,
    after: Option<T>,
}

trait RangeExt
where
    Self: Sized,
{
    fn split_on_overlap(self, other: &Self) -> OverlapInfo<Self>;
}

impl<T: Ord + Copy> RangeExt for Range<T> {
    // finds where self falls with regards to other
    // splits self into multiple smaller ranges which add to self
    fn split_on_overlap(self, other: &Self) -> OverlapInfo<Self> {
        if self.end <= other.start {
            return OverlapInfo {
                before: Some(self),
                overlap: None,
                after: None,
            };
        }

        if self.start >= other.end {
            return OverlapInfo {
                before: None,
                overlap: None,
                after: Some(self),
            };
        }

        // there is some overlap => find it
        let overlap = Some(max(self.start, other.start)..min(self.end, other.end));

        let before = if self.start < other.start {
            Some(self.start..other.start)
        } else {
            None
        };

        let after = if self.end > other.end {
            Some(other.end..self.end)
        } else {
            None
        };

        OverlapInfo {
            before,
            overlap,
            after,
        }
    }
}

#[derive(Debug)]
struct MapRange {
    range: Range<usize>,
    dest_start: usize,
}

impl MapRange {
    fn new(start: usize, end: usize, dest_start: usize) -> Self {
        MapRange {
            range: start..end,
            dest_start,
        }
    }

    fn src_to_dest(&self, val: usize) -> usize {
        val - self.range.start + self.dest_start
    }

    fn src_to_dest_range(&self, range: Range<usize>) -> Range<usize> {
        self.src_to_dest(range.start)..self.src_to_dest(range.end)
    }
}

fn parse_input(puzzle_input: &str) -> (Vec<usize>, Vec<Vec<MapRange>>) {
    let mut puzzle_lines = puzzle_input.lines();

    // split first line on : then parse usizes as seed values
    let seeds: Vec<usize> = puzzle_lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|num_str| num_str.parse().unwrap())
        .collect();

    let mut maps: Vec<Vec<MapRange>> = Vec::new();

    let mut new_map_next = false;
    for line in puzzle_lines {
        if line.is_empty() {
            new_map_next = true;
        } else if new_map_next {
            // line is: x-to-y map:
            maps.push(Vec::new());
            new_map_next = false;
        } else {
            // line is: [dest start] [source start] [range len]
            let nums: Vec<usize> = line
                .split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();

            maps.last_mut()
                .unwrap()
                .push(MapRange::new(nums[1], nums[1] + nums[2], nums[0]))
        }
    }

    // sort by start of source range
    for map in maps.iter_mut() {
        map.sort_by(|mr1, mr2| mr1.range.start.cmp(&mr2.range.start));
    }

    (seeds, maps)
}

trait MapSlice {
    fn follow(&self, seed: usize) -> usize;
    fn follow_range(&self, seed_range: Range<usize>) -> Vec<Range<usize>>;
}

impl MapSlice for &[Vec<MapRange>] {
    fn follow(&self, seed: usize) -> usize {
        let mut src_val = seed;
        // for each map
        for map in self.iter() {
            // for map range in the map (ex: for each range in seed-to-soil map)
            // TODO: make this its own function
            // i.e. src_val = map.follow(src_val);
            for map_range in map {
                if map_range.range.contains(&src_val) {
                    src_val = map_range.src_to_dest(src_val);
                    break;
                }
            }
        }

        src_val
    }

    // assume each map is sorted by start of map range
    fn follow_range(&self, seed_range: Range<usize>) -> Vec<Range<usize>> {
        let mut src_ranges = vec![seed_range];

        // for each map
        for map in self.iter() {
            // TODO: make this its own function
            // i.e. src_ranges = map.follow_range(src_ranges)
            let mut next_src_ranges = Vec::new();

            for src_range in src_ranges {
                // for each map range in the map (ex: for each range in seed-to-soil map)
                // sorted by start

                // TODO: redo this section assuming changed OverlapInfo
                // to be an enum containing either option before + option overlap
                // or option before + option overlap + after (after is not an option)
                //
                // this will make remaining range able to not be an option and the
                // code will be much more readable
                let mut remaining_range = Some(src_range);
                for map_range in map {
                    // Note: panics if remaining range is none
                    // => break whenever remaining range is None!
                    let overlap_info = remaining_range.unwrap().split_on_overlap(&map_range.range);

                    if let Some(overlap) = overlap_info.overlap {
                        next_src_ranges.push(map_range.src_to_dest_range(overlap));

                        if let Some(before) = overlap_info.before {
                            next_src_ranges.push(before);
                        }

                        // some overlap after => keep going
                        // no overlap after => stop!
                        remaining_range = overlap_info.after;
                        if remaining_range.is_none() {
                            break;
                        }
                    } else if overlap_info.before.is_some() {
                        // if this is true, since maps are sorted
                        // in terms of map_range.range.start,
                        // then no more ranges will have overlap.

                        // equiv to next_src_ranges.push(overlap_info.before)
                        // if we used a let Some(before)
                        remaining_range = overlap_info.before;
                        break;
                    } else {
                        // otherwise, the entire range is after
                        // and remaining range is some!
                        // this is because a range can't be before and after
                        // but not overlap!
                        remaining_range = overlap_info.after;
                    }
                }
                if let Some(the_range) = remaining_range {
                    next_src_ranges.push(the_range);
                }
            }

            src_ranges = next_src_ranges;
        }

        src_ranges
    }
}

fn part1(seeds: &[usize], maps: &[Vec<MapRange>]) -> usize {
    seeds.iter().map(|&seed| maps.follow(seed)).min().unwrap()
}

// remember assume map ranges are sorted (each maps[i] is sorted)
fn part2(seeds: &[usize], maps: &[Vec<MapRange>]) -> usize {
    let seed_ranges = seeds
        .chunks_exact(2)
        .map(|chunk| chunk[0]..chunk[0] + chunk[1]);

    seed_ranges
        .map(|seed_range| {
            maps.follow_range(seed_range)
                .into_iter()
                .map(|src_range| src_range.start)
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
}

fn main() {
    let file_contents = std::fs::read("input.txt").unwrap();

    let puzzle_input = std::str::from_utf8(&file_contents).unwrap();

    let (seeds, maps) = parse_input(puzzle_input);

    println!("{}", part1(&seeds, &maps));
    println!("{}", part2(&seeds, &maps));
}
