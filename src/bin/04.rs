use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "04"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

fn read_grid<R: BufRead>(reader: R, grid: &mut Vec<Vec<u8>>) -> Result<bool> {
    // First an empty row
    grid.push(Vec::new());
    for (row, line) in reader.lines().enumerate() {
        grid.push(Vec::new());
        // one empty spot on the left edge
        grid[row + 1].push(0);
        for c in line.unwrap().chars() {
            grid[row + 1].push(if c == '.' { 0 } else { 1 });
        }
        // one empty spot on the right edge
        grid[row + 1].push(0);
    }
    // End with an empty row
    grid.push(Vec::new());

    let line_length = grid[1].len();
    let grid_len = grid.len();
    for _ in 0..line_length {
        grid[0].push(0);
        grid[grid_len - 1].push(0);
    }

    Ok(true)
}

fn count_movable_boxes(grid: Vec<Vec<u8>>) -> Result<usize> {
    let mut answer = 0;

    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.into_iter().enumerate() {
            if *cell == 1 {
                let count = grid[y - 1][x - 1]
                    + grid[y - 1][x]
                    + grid[y - 1][x + 1]
                    + grid[y][x - 1]
                    + grid[y][x + 1]
                    + grid[y + 1][x - 1]
                    + grid[y + 1][x]
                    + grid[y + 1][x + 1];
                if count < 4 {
                    answer += 1;
                }
            }
        }
    }

    Ok(answer)
}
fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut grid: Vec<Vec<u8>> = Vec::new();
        read_grid(reader, &mut grid).expect("Could not read the file");

        Ok(count_movable_boxes(grid).unwrap())
    }

    assert_eq!(13, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        Ok(0)
    }

    assert_eq!(43, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
