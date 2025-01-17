use std::{array, collections::HashSet, fs};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Position {
    x: u32,
    y: u32,
    z: u32,
}

impl From<&str> for Position {
    fn from(value: &str) -> Self {
        // value = "x,y,z" where x y and z are u32
        let mut iter = value.split(',').map(|s| s.parse().unwrap());

        let [x, y, z] = array::from_fn(|_| iter.next().unwrap());

        Position { x, y, z }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Brick {
    min: Position,
    max: Position,
}

impl Brick {
    fn horizontally_collides(&self, other: &Self) -> bool {
        !(self.max.x < other.min.x
            || other.max.x < self.min.x
            || self.max.y < other.min.y
            || other.max.y < self.min.y)
    }

    fn set_bottom_z(&mut self, z: u32) {
        let height = self.max.z - self.min.z;

        self.min.z = z;
        self.max.z = z + height;
    }
}

impl From<&str> for Brick {
    fn from(value: &str) -> Self {
        // looks like: <min position>~<max position>
        let mut iter = value.split('~').map(Position::from);

        let min = iter.next().unwrap();
        let max = iter.next().unwrap();

        Brick { min, max }
    }
}

fn parse_input(input: &str) -> Vec<Brick> {
    input.lines().map(Brick::from).collect()
}

fn drop_bricks(mut bricks: Vec<Brick>) -> Vec<Brick> {
    // sort by minimum Z
    bricks.sort_unstable_by(|brick1, brick2| brick1.max.z.cmp(&brick2.max.z));

    // create a list which is sorted by maximum Z value
    let mut landed_bricks = Vec::new();

    for mut falling_brick in bricks {
        // landed_bricks is in sorted order lowest max Z to highest max Z
        // but we iterate in reverse to find the first thing hit, to_land_on
        let to_land_on = landed_bricks
            .iter()
            .rev()
            .find(|other_brick| falling_brick.horizontally_collides(other_brick));

        let new_z_value = match to_land_on {
            // land on the first brick that collided with
            Some(brick_to_land_on) => brick_to_land_on.max.z + 1,
            // doesn't collide with an existing brick, land on the ground
            None => 1,
        };

        // land on to_land_on
        falling_brick.set_bottom_z(new_z_value);

        // add the brick to the list
        let insert_index = match landed_bricks
            .binary_search_by(|other_brick| other_brick.max.z.cmp(&falling_brick.max.z))
        {
            Ok(i) => i,
            Err(i) => i,
        };

        landed_bricks.insert(insert_index, falling_brick);
    }

    // get landed_bricks in descending order of max z
    landed_bricks.reverse();

    landed_bricks
}

// ASSUME: landed_bricks is sorted in terms of maximum Z in descending order!
fn get_bricks_above_and_below(landed_bricks: &[Brick]) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    // bricks_below[i] = a vector containing the indices of bricks below landed_bricks[i]
    let mut bricks_above = vec![vec![]; landed_bricks.len()];
    let mut bricks_below = vec![vec![]; landed_bricks.len()];

    for (i, higher_brick) in landed_bricks.iter().enumerate() {
        for (j, lower_brick) in landed_bricks[i + 1..].iter().enumerate() {
            if lower_brick.max.z + 1 < higher_brick.min.z {
                break;
            }

            if lower_brick.max.z + 1 == higher_brick.min.z
                && lower_brick.horizontally_collides(higher_brick)
            {
                bricks_above[i + 1 + j].push(i);
                bricks_below[i].push(i + 1 + j);
            }
        }
    }

    (bricks_above, bricks_below)
}

// ASSUME: landed_bricks is sorted in terms of maximum Z in descending order!
fn num_disintegrateable(landed_bricks: &[Brick]) -> usize {
    let (_, bricks_below) = get_bricks_above_and_below(landed_bricks);
    let non_disintegrateable = bricks_below
        .into_iter()
        .filter_map(|below| {
            if below.len() == 1 {
                Some(below[0])
            } else {
                None
            }
        })
        .collect::<HashSet<_>>();

    landed_bricks.len() - non_disintegrateable.len()
}

// ASSUME: landed_bricks is sorted in terms of maximum Z in descending order!
fn part2(landed_bricks: &[Brick]) -> usize {
    let (bricks_above, bricks_below) = get_bricks_above_and_below(landed_bricks);

    (0..landed_bricks.len())
        .map(|i| {
            let mut stack = vec![i];
            let mut fallen = HashSet::new();

            while let Some(cur_index) = stack.pop() {
                fallen.insert(cur_index);

                // bricks only fall when everything under them has fallen :O
                for above_index in bricks_above[cur_index].iter().copied() {
                    if bricks_below[above_index]
                        .iter()
                        .all(|sibling_index| fallen.contains(sibling_index))
                    {
                        stack.push(above_index);
                    }
                }
            }

            // don't count the starting node
            // since only want to count which have fallen
            // and start node was disintegrated
            fallen.len() - 1
        })
        .sum()
}

fn main() {
    let file_contents = fs::read_to_string("input.txt").unwrap();

    let bricks = parse_input(&file_contents);

    let landed_bricks = drop_bricks(bricks);

    println!("Part 1: {}", num_disintegrateable(&landed_bricks));
    println!("Part 2: {}", part2(&landed_bricks));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "1,0,1~1,2,1\n\
                              0,0,2~2,0,2\n\
                              0,2,3~2,2,3\n\
                              0,0,4~0,2,4\n\
                              2,0,5~2,2,5\n\
                              0,1,6~2,1,6\n\
                              1,1,8~1,1,9";

    #[test]
    fn test_part1() {
        let bricks = parse_input(TEST_INPUT);
        let landed_bricks = drop_bricks(bricks);

        assert_eq!(num_disintegrateable(&landed_bricks), 5);
    }

    #[test]
    fn test_part2() {
        let bricks = parse_input(TEST_INPUT);
        let landed_bricks = drop_bricks(bricks);

        assert_eq!(part2(&landed_bricks), 7);
    }
}
