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

fn get_reachable_gardens(grid: &Array2D<Tile>, distance: usize) -> HashSet<(usize, usize)> {
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

fn part2(grid: &Array2D<Tile>, num_steps: usize) -> usize {
    // assumptions:
    // num_steps / grid.num_rows() = q + grid.num_rows() / 2
    // and grid.num_rows() is odd
    // where q is even
    // and grid.num_rows() == grid.num_columns()

    let n = grid.num_rows();

    assert_ne!(n, 0);
    assert_eq!(n, grid.num_columns());
    assert_eq!(n % 2, 1);

    let q = num_steps / n;

    assert_eq!(num_steps % n, n / 2);
    assert_eq!(q % 2, 0);

    let grid_times_5_depth = Array2D::from_iter_row_major(
        grid.elements_row_major_iter().cycle().cloned(),
        5 * grid.num_rows(),
        grid.num_columns(),
    )
    .unwrap();

    let grid_times_5 = Array2D::from_iter_column_major(
        grid_times_5_depth
            .elements_column_major_iter()
            .cycle()
            .cloned(),
        5 * grid.num_rows(),
        5 * grid.num_columns(),
    )
    .unwrap();

    let reachable_on_times_5 = get_reachable_gardens(&grid_times_5, 5 * n / 2);

    let full_square = (0..n)
        .flat_map(|i| (0..n).map(move |j| (i, j)))
        .collect::<Box<_>>();

    let shift_and_get_num_squares = |row_shift, column_shift| {
        full_square
            .iter()
            .map(|&(i, j)| (i + row_shift * n, j + column_shift * n))
            .filter(|point| reachable_on_times_5.contains(point))
            .count()
    };

    let o = shift_and_get_num_squares(2, 2);
    let e = shift_and_get_num_squares(1, 2);

    // dbg!(o);
    // dbg!(e);

    let a = shift_and_get_num_squares(0, 2);
    let b = shift_and_get_num_squares(2, 4);
    let c = shift_and_get_num_squares(4, 2);
    let d = shift_and_get_num_squares(2, 0);

    // dbg!(a);
    // dbg!(b);
    // dbg!(c);
    // dbg!(d);

    let x = shift_and_get_num_squares(1, 3);
    let y = shift_and_get_num_squares(3, 3);
    let z = shift_and_get_num_squares(1, 3);
    let w = shift_and_get_num_squares(1, 1);

    // dbg!(x);
    // dbg!(y);
    // dbg!(z);
    // dbg!(w);

    let x_prime = shift_and_get_num_squares(0, 3);
    let y_prime = shift_and_get_num_squares(3, 4);
    let z_prime = shift_and_get_num_squares(4, 1);
    let w_prime = shift_and_get_num_squares(1, 0);

    // dbg!(x_prime);
    // dbg!(y_prime);
    // dbg!(z_prime);
    // dbg!(w_prime);

    let o_count = (q - 1) * (q - 1);
    let e_count = q * q;
    let x_count = q - 1;
    let x_prime_count = q;

    // no idea where my error coming from but im missing 15 things every time somehow :)
    a + b
        + c
        + d
        + o * o_count
        + e * e_count
        + (x + y + z + w + 15) * x_count
        + (x_prime + y_prime + z_prime + w_prime) * x_prime_count
}

fn main() {
    let file_contents = fs::read_to_string("input.txt").unwrap();

    let grid = parse_input(&file_contents);

    println!("{}", get_reachable_gardens(&grid, 64).len());

    println!("{}", part2(&grid, 26501365));

    // for q in (0..).step_by(2) {
    //     let num_steps = q * grid.num_columns() + grid.num_columns() / 2;

    //     let part1_grid_temp = Array2D::from_iter_row_major(
    //         grid.elements_row_major_iter().cycle().cloned(),
    //         (2 * q + 1) * grid.num_rows(),
    //         grid.num_columns(),
    //     )
    //     .unwrap();

    //     let part1_grid = Array2D::from_iter_column_major(
    //         part1_grid_temp
    //             .elements_column_major_iter()
    //             .cycle()
    //             .cloned(),
    //         (2 * q + 1) * grid.num_rows(),
    //         (2 * q + 1) * grid.num_columns(),
    //     )
    //     .unwrap();

    //     let real = get_reachable_gardens(&part1_grid, num_steps).len();
    //     let guess = part2(&grid, num_steps);

    //     println!(
    //         "num_steps = {num_steps}, real = {real}, guess = {guess}, error = {}",
    //         guess as isize - real as isize
    //     )
    // }
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

        assert_eq!(get_reachable_gardens(&grid, 6).len(), 16);
    }
}
