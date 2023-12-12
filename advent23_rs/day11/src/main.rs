// (row, col) position
struct Galaxy(usize, usize);

fn find_sum_of_dists(puzzle_input: &[&[u8]], expansion_factor: usize) -> usize {
    let mut row_expands = Vec::with_capacity(puzzle_input.len());
    let mut num_empty = 0;

    for line in puzzle_input.iter() {
        if line.iter().all(|&c| c == b'.') {
            num_empty += expansion_factor - 1;
        }
        row_expands.push(num_empty);
    }

    let mut col_expands = Vec::with_capacity(puzzle_input[0].len());
    let mut num_empty = 0;

    for col in 0..puzzle_input[0].len() {
        if puzzle_input.iter().all(|row| row[col] == b'.') {
            num_empty += expansion_factor - 1;
        }
        col_expands.push(num_empty);
    }

    let mut galaxies = Vec::new();

    for (row, line) in puzzle_input.iter().enumerate() {
        for (col, &cell) in line.iter().enumerate() {
            if cell == b'#' {
                galaxies.push(Galaxy(row + row_expands[row], col + col_expands[col]));
            }
        }
    }

    let mut result = 0;
    for (i, Galaxy(row1, col1)) in galaxies.iter().enumerate() {
        for Galaxy(row2, col2) in galaxies[i + 1..].iter() {
            result += if row1 > row2 {
                row1 - row2
            } else {
                row2 - row1
            };

            result += if col1 > col2 {
                col1 - col2
            } else {
                col2 - col1
            };
        }
    }

    result
}

fn main() {
    let file_contents = std::fs::read("input.txt").unwrap();
    let file_contents_as_str = std::str::from_utf8(&file_contents).unwrap();

    let puzzle_input = file_contents_as_str
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();

    println!("{}", find_sum_of_dists(&puzzle_input, 2));
    println!("{}", find_sum_of_dists(&puzzle_input, 1000000));
}
