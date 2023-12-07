use std::{
    cmp::{max, min},
    ops::Range,
    usize,
};

enum OverlapInfo<T> {
    // option(option (before), overlap), after
    // therefore cant have before and after but no overlap
    HasAfter(Option<(Option<T>, T)>, T),
    // option before, overlap
    NoAfterHasOverlap(Option<T>, T),
    // everything comes before
    AllBefore(T),
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
            // self comes before and then there is no overlap
            return OverlapInfo::AllBefore(self);
        }

        if self.start >= other.end {
            // self comes after and there is no overlap
            return OverlapInfo::HasAfter(None, self);
        }

        // there is some overlap => find it
        let overlap = max(self.start, other.start)..min(self.end, other.end);

        let maybe_before = if self.start < other.start {
            Some(self.start..other.start)
        } else {
            None
        };

        if self.end > other.end {
            OverlapInfo::HasAfter(Some((maybe_before, overlap)), other.end..self.end)
        } else {
            OverlapInfo::NoAfterHasOverlap(maybe_before, overlap)
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

trait Map {
    fn follow(&self, seed: usize) -> usize;

    fn follow_ranges(&self, seed_range: &[Range<usize>]) -> Vec<Range<usize>>;
}

// assumes the map is in sorted order!
impl Map for [MapRange] {
    fn follow(&self, src_val: usize) -> usize {
        // TODO: use binary search to find the range containing src_val
        // (there can be at most 1)
        // for now just use a linear scan
        for map_range in self.iter() {
            if map_range.range.contains(&src_val) {
                return map_range.src_to_dest(src_val);
            }
        }
        src_val
    }

    // could make a follow_range
    // function as well but idk how hard it would be
    // with lifetimes and such so I'm gonna stop for rn
    fn follow_ranges(&self, src_ranges: &[Range<usize>]) -> Vec<Range<usize>> {
        let mut result = Vec::new();

        for src_range in src_ranges {
            let mut remaining_range = Some((*src_range).clone());

            // TODO: can use binary search to find the first place where the overlap
            // is not all after and then can linear scan from there.
            // for now just use a linear scan
            for map_range in self.iter() {
                let Some(range) = remaining_range else {
                    break;
                };

                match range.split_on_overlap(&map_range.range) {
                    OverlapInfo::HasAfter(maybe_before_and_overlap, after) => {
                        if let Some((maybe_before, overlap)) = maybe_before_and_overlap {
                            result.push(map_range.src_to_dest_range(overlap));
                            if let Some(before) = maybe_before {
                                result.push(before);
                            }
                        }
                        remaining_range = Some(after);
                    }
                    OverlapInfo::NoAfterHasOverlap(maybe_before, overlap) => {
                        result.push(map_range.src_to_dest_range(overlap));
                        if let Some(before) = maybe_before {
                            result.push(before);
                        }
                        remaining_range = None;
                        break;
                    }
                    OverlapInfo::AllBefore(before) => {
                        result.push(before);
                        remaining_range = None;
                        break;
                    }
                }
            }

            if let Some(range) = remaining_range {
                result.push(range);
            }
        }

        result
    }
}

trait MapSequence {
    fn follow(&self, seed: usize) -> usize;
    fn follow_range(&self, seed_range: Range<usize>) -> Vec<Range<usize>>;
}

impl MapSequence for [Vec<MapRange>] {
    fn follow(&self, seed: usize) -> usize {
        let mut src_val = seed;
        // for each map
        for map in self.iter() {
            src_val = map.follow(src_val);
        }

        src_val
    }

    // assume each map is sorted by start of map range
    fn follow_range(&self, seed_range: Range<usize>) -> Vec<Range<usize>> {
        let mut src_ranges = vec![seed_range];
        for map in self.iter() {
            src_ranges = map.follow_ranges(&src_ranges);
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
        .flat_map(|seed_range| {
            maps.follow_range(seed_range)
                .into_iter()
                .map(|src_range| src_range.start)
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
