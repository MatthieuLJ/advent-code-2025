use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "09";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

fn read_file<R: BufRead>(reader: R) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    let re = Regex::new(r"^(\d*),(\d*)$").unwrap();
    for l in reader.lines().flatten() {
        let Some(caps) = re.captures(&l) else {
            panic!("Failed to read range {l}");
        };
        result.push((
            usize::from_str_radix(&caps[1], 10).unwrap(),
            usize::from_str_radix(&caps[2], 10).unwrap(),
        ));
    }

    result
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let coordinates = read_file(reader);
        let num_points = coordinates.len();
        let mut largest_area = 0;

        for i in 0..num_points {
            for j in i + 1..num_points {
                let new_area = ((coordinates[i].0 as isize - coordinates[j].0 as isize + 1)
                    * (coordinates[i].1 as isize - coordinates[j].1 as isize + 1))
                    .abs() as usize;
                if new_area > largest_area {
                    largest_area = new_area;
                }
            }
        }
        Ok(largest_area)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(50, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //     Ok(0)
    // }
    //
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}
