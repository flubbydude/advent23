use std::{
    array,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3 {
    const fn filled_with(val: f64) -> Self {
        Vector3 {
            x: val,
            y: val,
            z: val,
        }
    }
}

impl From<&str> for Vector3 {
    fn from(value: &str) -> Self {
        // ex: "19, 13, 30"
        let mut vals = value.split(", ").map(|s| s.parse().unwrap());
        let [x, y, z] = array::from_fn(|_| vals.next().unwrap());
        Vector3 { x, y, z }
    }
}

impl From<(f64, f64)> for Vector3 {
    fn from(value: (f64, f64)) -> Self {
        Vector3 {
            x: value.0,
            y: value.1,
            z: 0.,
        }
    }
}

#[derive(Debug)]
struct Hailstone {
    position: Vector3,
    velocity: Vector3,
}

struct Bounds {
    min: Vector3,
    max: Vector3,
}

impl Bounds {
    fn contains_point_2d(&self, point: &Vector3) -> bool {
        self.min.x <= point.x
            && point.x <= self.max.x
            && self.min.y <= point.y
            && point.y <= self.max.y
    }
}

const PART1_BOUNDS: Bounds = Bounds {
    min: Vector3::filled_with(200000000000000.),
    max: Vector3::filled_with(400000000000000.),
};

enum LineIntersection {
    SinglePoint(Vector3),
    #[allow(dead_code)]
    InfinitePoints,
}

impl Hailstone {
    fn intersection_point_2d(&self, other: &Self) -> Option<LineIntersection> {
        // TODO: assumed no undefined slopes, based on my input

        let slope1 = self.velocity.y / self.velocity.x;
        let slope2 = other.velocity.y / other.velocity.x;

        // TODO: also assume parallel lines don't intersect, :)
        if slope1 == slope2 {
            return None;
        }

        let intersection_x = (-self.position.y + other.position.y + slope1 * self.position.x
            - slope2 * other.position.x)
            / (slope1 - slope2);

        let intersection_y = slope1 * (intersection_x - self.position.x) + self.position.y;

        Some(LineIntersection::SinglePoint(
            (intersection_x, intersection_y).into(),
        ))
    }

    fn crosses_in_future_2d(&self, point: &Vector3) -> bool {
        (point.x - self.position.x).signum() == self.velocity.x.signum()
            && (point.y - self.position.y).signum() == self.velocity.y.signum()
    }

    fn intersects_in_bounds_2d(&self, other: &Self, bounds: &Bounds) -> bool {
        match self.intersection_point_2d(other) {
            Some(intersection) => match intersection {
                LineIntersection::SinglePoint(point) => {
                    bounds.contains_point_2d(&point)
                        && self.crosses_in_future_2d(&point)
                        && other.crosses_in_future_2d(&point)
                }
                LineIntersection::InfinitePoints => todo!(),
            },
            None => false,
        }
    }
}

impl From<&str> for Hailstone {
    fn from(value: &str) -> Self {
        // ex: "19, 13, 30 @ -2,  1, -2"
        let (position_str, velocity_str) = value.split_once(" @ ").unwrap();

        Hailstone {
            position: position_str.into(),
            velocity: velocity_str.into(),
        }
    }
}

fn part1(hailstones: &[Hailstone], bounds: &Bounds) -> usize {
    let mut result = 0;
    for (i, stone1) in hailstones.iter().enumerate() {
        for stone2 in &hailstones[i + 1..] {
            if stone1.intersects_in_bounds_2d(stone2, bounds) {
                result += 1;
            }
        }
    }

    result
}

// prints output to put into console for use in part2.py
fn part2(hailstones: &[Hailstone]) {
    // find 3 lines, assume a path through these lines
    // should pass through all lines because i think there's only 1 unique solution

    for (i, hailstone) in hailstones[0..3].iter().enumerate() {
        println!(
            "x+{time}*u{:+}{:+}*{time},\n\
             y+{time}*v{:+}{:+}*{time},\n\
             z+{time}*w{:+}{:+}*{time},\n",
            -hailstone.position.x,
            -hailstone.velocity.x,
            -hailstone.position.y,
            -hailstone.velocity.y,
            -hailstone.position.z,
            -hailstone.velocity.z,
            time = (b'r' + i as u8) as char
        );
    }
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let hailstones = reader
        .lines()
        .map(|line| Hailstone::from(line.unwrap().as_str()))
        .collect::<Vec<_>>();

    println!("Part 1: {}", part1(&hailstones, &PART1_BOUNDS));
    part2(&hailstones);
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2, Bounds, Hailstone, Vector3};

    const TEST_INPUT: &str = "19, 13, 30 @ -2, 1, -2\n\
                              18, 19, 22 @ -1, -1, -2\n\
                              20, 25, 34 @ -2, -2, -4\n\
                              12, 31, 28 @ -1, -2, -1\n\
                              20, 19, 15 @ 1, -5, -3";

    const TEST_BOUNDS: Bounds = Bounds {
        min: Vector3::filled_with(7.),
        max: Vector3::filled_with(27.),
    };

    #[test]
    fn test_part1() {
        let hailstones = TEST_INPUT.lines().map(Hailstone::from).collect::<Vec<_>>();

        assert_eq!(part1(&hailstones, &TEST_BOUNDS), 2);
    }

    #[test]
    fn test_part2() {
        let hailstones = TEST_INPUT.lines().map(Hailstone::from).collect::<Vec<_>>();
        part2(&hailstones);
    }
}
