use array2d::Array2D;
use std::{collections::HashSet, fs};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Clone)]
enum Tile {
    GardenPlot,
    Rock,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::GardenPlot,
            '#' => Tile::Rock,
            'S' => Tile::GardenPlot,
            _ => panic!("Cannot convert '{}' into Tile", value),
        }
    }
}

#[derive(EnumIter)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn parse_input(input: &str) -> Array2D<Tile> {
    Array2D::from_rows(
        &input
            .lines()
            .map(|line| line.chars().map(Tile::from).collect())
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

fn find_start(grid: &Array2D<Tile>) -> (usize, usize) {
    (grid.num_rows() / 2, grid.num_columns() / 2)
}

fn get_successors(grid: &Array2D<Tile>, point: (usize, usize)) -> Vec<(usize, usize)> {
    let mut result = Vec::with_capacity(4);
    let (row, column) = point;

    for direction in Direction::iter() {
        let point = match direction {
            Direction::North => {
                if row > 0 {
                    (row - 1, column)
                } else {
                    continue;
                }
            }
            Direction::East => {
                if column < grid.num_columns() - 1 {
                    (row, column + 1)
                } else {
                    continue;
                }
            }
            Direction::South => {
                if row < grid.num_rows() - 1 {
                    (row + 1, column)
                } else {
                    continue;
                }
            }
            Direction::West => {
                if column > 0 {
                    (row, column - 1)
                } else {
                    continue;
                }
            }
        };

        if !matches!(grid[point], Tile::Rock) {
            result.push(point);
        }
    }

    result
}

fn part1(grid: &Array2D<Tile>, distance: usize) -> HashSet<(usize, usize)> {
    let start_pt = find_start(grid);

    let mut queue = HashSet::from([start_pt]);

    for _ in 0..distance {
        let mut next_queue = HashSet::new();

        for point in queue {
            next_queue.extend(get_successors(grid, point));
        }

        queue = next_queue;
    }

    queue
}

fn num_gardens_with(reachable: &HashSet<(usize, usize)>, predicate: fn(usize, usize) -> bool) -> usize {
    reachable.iter().filter(|&&(i, j)| predicate(i, j)).count()
}

#[derive(Debug)]
struct CornerCounts {
    top_left: usize,
    top_right: usize,
    bottom_left: usize,
    bottom_right: usize,
}

// corner is where i + j <= 65 basically :O
fn num_gardens_in_corners_with(
    reachable: &HashSet<(usize, usize)>,
    // used for even/odd checking
    grid_side_length: usize,
    predicate: fn(usize, usize) -> bool,
) -> CornerCounts {
    let half_grid = grid_side_length / 2;

    let mut top_left = 0;
    let mut top_right = 0;
    let mut bottom_left = 0;
    let mut bottom_right = 0;

    let passes_predicate_and_is_garden = |i, j| predicate(i, j) && reachable.contains(&(i, j));

    for i in 0..half_grid + 1 {
        for j in 0..half_grid - i + 1 {
            let rev_i = grid_side_length - i - 1;
            let rev_j = grid_side_length - j - 1;
            if passes_predicate_and_is_garden(i, j) {
                top_left += 1;
            }
            if passes_predicate_and_is_garden(i, rev_j) {
                top_right += 1;
            }
            if passes_predicate_and_is_garden(rev_i, j) {
                bottom_left += 1;
            }
            if passes_predicate_and_is_garden(rev_i, rev_j) {
                bottom_right += 1;
            }
        }
    }

    CornerCounts {
        top_left,
        top_right,
        bottom_left,
        bottom_right,
    }
}

fn _part2_old(grid: &Array2D<Tile>, num_steps: usize) -> usize {
    // assumptions:
    // num_steps / grid.num_rows() = q + grid.num_rows() / 2
    // and grid.num_rows() is odd
    // where q is even
    // and grid.num_rows() == grid.num_columns()

    assert_eq!(grid.num_rows(), grid.num_columns());
    assert_eq!(grid.num_rows() % 2, 1);

    let q = num_steps / grid.num_rows();

    assert_eq!(num_steps % grid.num_rows(), grid.num_rows() / 2);
    assert_eq!(q % 2, 0);

    let reachable_grid = part1(grid, 65);

    let num_even_gardens = num_gardens_with(&reachable_grid, |i, j| (i + j) % 2 == 0);
    let num_odd_gardens = num_gardens_with(&reachable_grid, |i, j| (i + j) % 2 == 1);

    let CornerCounts {
        top_left: top_left_even,
        top_right: top_right_even,
        bottom_left: bottom_left_even,
        bottom_right: bottom_right_even,
    } = num_gardens_in_corners_with(&reachable_grid, |i, j| (i + j) % 2 == 0);
    let CornerCounts {
        top_left: top_left_odd,
        top_right: top_right_odd,
        bottom_left: bottom_left_odd,
        bottom_right: bottom_right_odd,
    } = num_gardens_in_corners_with(&reachable_grid, |i, j| (i + j) % 2 == 1);

    let num_gardens_a = top_left_odd + bottom_right_odd + top_right_even + bottom_left_even;
    let num_gardens_b = top_left_even + bottom_right_even + top_right_odd + bottom_left_odd;
    let num_gardens_o =
        num_even_gardens - top_left_even - top_right_even - bottom_left_even - bottom_right_even;
    let num_gardens_e =
        num_odd_gardens - top_left_odd - top_right_odd - bottom_left_odd - bottom_right_odd;

    let o_count = (q + 1) * (q + 1);
    let e_count = q * q;
    let a_count = q * (q + 1);
    let b_count = a_count;

    // error = 0.5038167938931297 * num_steps + -131.7480916030534
    let error = 0.5038167938931297 * (num_steps as f64) + -131.7480916030534;

    o_count * num_gardens_o
        + e_count * num_gardens_e
        + a_count * num_gardens_a
        + b_count * num_gardens_b
        - error as usize
}

fn part2(grid: &Array2D<Tile>, num_steps: usize) -> usize {
    // assumptions:
    // num_steps / grid.num_rows() = q + grid.num_rows() / 2
    // and grid.num_rows() is odd
    // where q is even
    // and grid.num_rows() == grid.num_columns()

    assert_eq!(grid.num_rows(), grid.num_columns());
    assert_eq!(grid.num_rows() % 2, 1);

    let q = num_steps / grid.num_rows();

    assert_eq!(num_steps % grid.num_rows(), grid.num_rows() / 2);
    assert_eq!(q % 2, 0);

    let reachable_grid = reachable_gardens(grid);

    let num_even_gardens = num_gardens_with(&reachable_grid, |i, j| (i + j) % 2 == 0);
    let num_odd_gardens = num_gardens_with(&reachable_grid, |i, j| (i + j) % 2 == 1);

    let CornerCounts {
        top_left: top_left_even,
        top_right: top_right_even,
        bottom_left: bottom_left_even,
        bottom_right: bottom_right_even,
    } = num_gardens_in_corners_with(&reachable_grid, |i, j| (i + j) % 2 == 0);
    let CornerCounts {
        top_left: top_left_odd,
        top_right: top_right_odd,
        bottom_left: bottom_left_odd,
        bottom_right: bottom_right_odd,
    } = num_gardens_in_corners_with(&reachable_grid, |i, j| (i + j) % 2 == 1);

    let num_gardens_a = top_left_odd + bottom_right_odd + top_right_even + bottom_left_even;
    let num_gardens_b = top_left_even + bottom_right_even + top_right_odd + bottom_left_odd;
    let num_gardens_o =
        num_even_gardens - top_left_even - top_right_even - bottom_left_even - bottom_right_even;
    let num_gardens_e =
        num_odd_gardens - top_left_odd - top_right_odd - bottom_left_odd - bottom_right_odd;

    let o_count = (q + 1) * (q + 1);
    let e_count = q * q;
    let a_count = q * (q + 1);
    let b_count = a_count;

    let mut result = o_count * num_gardens_o
    + e_count * num_gardens_e
    + a_count * num_gardens_a
    + b_count * num_gardens_b;

    let part1_grid_temp = Array2D::from_iter_row_major(
        grid.elements_row_major_iter().cycle().cloned(),
        5 * grid.num_rows(),
        grid.num_columns(),
    )
    .unwrap();

    let part1_grid = Array2D::from_iter_column_major(
        part1_grid_temp
            .elements_column_major_iter()
            .cycle()
            .cloned(),
        5 * grid.num_rows(),
        5 * grid.num_columns(),
    )
    .unwrap();

    let a_top_right_count = q;
    let b_top

    result
}

fn main() {
    let file_contents = fs::read_to_string("input.txt").unwrap();

    let grid = parse_input(&file_contents);

    println!("{}", part1(&grid, 64).len());

    // println!("{}", part2(&grid, 26501365));

    for q in (0..).step_by(2) {
        let num_steps = q * grid.num_columns() + grid.num_columns() / 2;

        let part1_grid_temp = Array2D::from_iter_row_major(
            grid.elements_row_major_iter().cycle().cloned(),
            (2 * q + 1) * grid.num_rows(),
            grid.num_columns(),
        )
        .unwrap();

        let part1_grid = Array2D::from_iter_column_major(
            part1_grid_temp
                .elements_column_major_iter()
                .cycle()
                .cloned(),
            (2 * q + 1) * grid.num_rows(),
            (2 * q + 1) * grid.num_columns(),
        )
        .unwrap();

        let real = part1(&part1_grid, num_steps);
        let guess = part2(&grid, num_steps);

        println!(
            "num_steps = {num_steps}, real = {real}, guess = {guess}, error = {}",
            guess as isize - real as isize
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "...........\n\
                              .....###.#.\n\
                              .###.##..#.\n\
                              ..#.#...#..\n\
                              ....#.#....\n\
                              .##..S####.\n\
                              .##..#...#.\n\
                              .......##..\n\
                              .##.#.####.\n\
                              .##..##.##.\n\
                              ...........";

    #[test]
    fn test_part1() {
        let grid = parse_input(TEST_INPUT);

        println!("{}", part1(&grid, 64));
    }
}
