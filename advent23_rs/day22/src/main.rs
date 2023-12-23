use std::{array, collections::HashSet, fs, mem::take};

mod linked_list;
use linked_list::Node;

#[derive(Debug, Clone, Default)]
struct Position {
    x: u32,
    y: u32,
    z: u32,
}

// impl Position {
//     fn below(&self) -> Self {
//         Position {
//             z: self.z - 1,
//             ..*self
//         }
//     }

//     fn above(&self) -> Self {
//         Position {
//             z: self.z + 1,
//             ..*self
//         }
//     }
// }

#[derive(Debug)]
struct Brick {
    min: Position,
    max: Position,
}

impl Brick {
    // fn horizontal_slice_iter(&self, z: u32) -> impl '_ + Iterator<Item = Position> {
    //     (self.min.x..self.max.x)
    //         .flat_map(move |x| (self.min.y..self.max.y).map(move |y| Position { x, y, z }))
    // }

    fn horizontally_collides(&self, other: &Self) -> bool {
        let x_collision = (self.min.x <= other.min.x && other.min.x <= self.max.x)
            || (other.min.x <= self.min.x && self.min.x <= other.max.x);

        let y_collision = (self.min.y <= other.min.y && other.min.y <= self.max.y)
            || (other.min.y <= self.min.y && self.min.y <= other.max.y);

        x_collision && y_collision
    }

    fn set_bottom_z(&mut self, z: u32) {
        let height = self.max.z - self.min.z;

        self.min.z = z;
        self.max.z = z + height;
    }
}

impl From<&str> for Position {
    fn from(value: &str) -> Self {
        // value = "x,y,z" where x y and z are u32
        let mut iter = value.split(',').map(|s| s.parse().unwrap());

        let [x, y, z] = array::from_fn(|_| iter.next().unwrap());

        Position { x, y, z }
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

fn part1(mut bricks: Vec<Brick>) -> usize {
    bricks.sort_by(|brick1, brick2| brick1.min.z.cmp(&brick2.min.z));

    let ground_brick = Brick {
        // (defaults to the origin)
        min: Position::default(),
        max: Position {
            x: u32::MAX,
            y: u32::MAX,
            z: 0,
        },
    };

    let mut landed_bricks = Node::new(ground_brick);

    for mut brick in bricks {
        // note landed bricks is in topological order
        // and always ends in the ground
        let mut cur_node = &mut landed_bricks;

        // if first brick collides, add to start of list and continue
        if brick.horizontally_collides(&cur_node.val) {
            brick.set_bottom_z(cur_node.val.max.z + 1);
            landed_bricks = landed_bricks.push_left(brick);
            continue;
        }

        loop {
            let next_brick = &cur_node.next.as_ref().unwrap().val;
            if brick.horizontally_collides(next_brick) {
                brick.set_bottom_z(next_brick.max.z + 1);
                let new_node = Node {
                    val: brick,
                    next: take(&mut cur_node.next),
                };
                cur_node.next = Some(Box::new(new_node));
                break;
            } else {
                cur_node = cur_node.next.as_mut().unwrap();
            }
        }
    }

    // for brick in landed_bricks.iter() {
    //     println!("{:?}", brick);
    // }

    let mut landed_bricks = landed_bricks.into_iter().collect::<Vec<_>>();

    landed_bricks.reverse();

    // landed bricks is in order from lowest to highest in
    // terms of minimum z value
    let landed_bricks = landed_bricks;

    // bricks_below[i] = a vector containing the indices of bricks below landed_bricks[i]
    let mut bricks_below = vec![vec![]; landed_bricks.len()];

    for (i, lower_brick) in landed_bricks.iter().enumerate() {
        for (j, higher_brick) in landed_bricks[i + 1..].iter().enumerate() {
            if lower_brick.max.z + 1 < higher_brick.min.z {
                break;
            }

            if lower_brick.max.z + 1 == higher_brick.min.z
                && lower_brick.horizontally_collides(higher_brick)
            {
                bricks_below[i + 1 + j].push(i);
            }
        }
    }

    for (i, below) in bricks_below.iter().enumerate() {
        println!("Below {} is {:?}", i, below.to_vec())
    }

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

    println!(
        "non_disintegrateable = {:?}",
        non_disintegrateable.iter().copied().collect::<Vec<_>>()
    );

    landed_bricks.len() - non_disintegrateable.len()
}

fn main() {
    let file_contents = fs::read_to_string("input.txt").unwrap();

    let bricks = parse_input(&file_contents);

    println!("{}", part1(bricks));
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

        assert_eq!(part1(bricks), 5);
    }
}
