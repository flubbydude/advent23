use anyhow::{Context, Result};
use std::array;

enum Instruction<'a> {
    Equals { label: &'a [u8], focal_length: u8 },
    Dash { label: &'a [u8] },
}

impl<'a> TryFrom<&'a [u8]> for Instruction<'a> {
    type Error = anyhow::Error;

    fn try_from(s: &'a [u8]) -> Result<Self> {
        if let Some(label) = s.strip_suffix(b"-") {
            Ok(Instruction::Dash { label })
        } else {
            let instruction_as_str = std::str::from_utf8(s)?;
            let (label_as_str, focal_length_as_str) = instruction_as_str.split_once('=')
                .with_context(|| format!("Failed to parse instruction \"{}\" in Instruction::try_from as it does not contain an '=' nor does it end with an '-'", instruction_as_str))?;

            Ok(Instruction::Equals {
                label: label_as_str.as_bytes(),
                focal_length: focal_length_as_str
                    .parse()
                    .context("Failed to parse instruction in Instruction::try_from")?,
            })
        }
    }
}

struct HashMapEntry<'a> {
    label: &'a [u8],
    focal_length: u8,
}

fn holiday_hash(s: &[u8]) -> u8 {
    let mut result = 0;

    for c in s {
        result += *c as u32;
        result *= 17;
        result %= 256;
    }

    result as u8
}

fn part1(puzzle_input: &[u8]) -> u64 {
    puzzle_input
        .split(|&c| c == b',')
        .map(|s| holiday_hash(s) as u64)
        .sum()
}

fn part2(puzzle_input: &[u8]) -> Result<usize> {
    let mut hashmap: [Vec<HashMapEntry>; 256] = array::from_fn(|_| Vec::new());

    for instruction_as_bytes in puzzle_input.split(|&c| c == b',') {
        match Instruction::try_from(instruction_as_bytes)? {
            Instruction::Equals {
                label,
                focal_length,
            } => {
                // add label with focal length to the list,
                // or modify the label if it already there
                let list = &mut hashmap[holiday_hash(label) as usize];

                let mut found = false;

                for entry in list.iter_mut() {
                    if label == entry.label {
                        entry.focal_length = focal_length;
                        found = true;
                        break;
                    }
                }

                if !found {
                    list.push(HashMapEntry {
                        label,
                        focal_length,
                    })
                }
            }
            Instruction::Dash { label } => {
                // remove the element from the list if it exists
                let list = &mut hashmap[holiday_hash(label) as usize];
                if let Some(i) = list.iter().position(|entry| label == entry.label) {
                    list.remove(i);
                }
            }
        }
    }

    let result = hashmap
        .into_iter()
        .enumerate()
        .map(|(box_num, list)| {
            (box_num + 1)
                * list
                    .into_iter()
                    .enumerate()
                    .map(|(slot_num, HashMapEntry { focal_length, .. })| {
                        (slot_num + 1) * (focal_length as usize)
                    })
                    .sum::<usize>()
        })
        .sum();

    Ok(result)
}

fn main() -> Result<()> {
    let puzzle_input = std::fs::read("input.txt")?;

    println!("{}", part1(&puzzle_input));
    println!("{}", part2(&puzzle_input)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &[u8] = b"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_holiday_hash() {
        assert_eq!(holiday_hash(b"HASH"), 52);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 1320);
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(TEST_INPUT)?, 145);

        Ok(())
    }
}
