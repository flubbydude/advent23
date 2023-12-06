use std::error;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() -> Result<()> {
    let file_contents = std::fs::read("input.txt")?;

    let puzzle_input = std::str::from_utf8(&file_contents)?
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();

    Ok(())
}
