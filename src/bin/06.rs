use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "06";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +";

enum Operation {
    PLUS,
    MULTIPLY,
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut lines: VecDeque<String> = reader.lines().flatten().collect();
        let operations_string = lines.pop_back().unwrap();
        let mut operations: Vec<Operation> = Vec::new();

        for o in operations_string.split_ascii_whitespace() {
            match o {
                "+" => operations.push(Operation::PLUS),
                "*" => operations.push(Operation::MULTIPLY),
                _ => panic!("Found something I don't understand {o}")
            };
        }

        let mut results: Vec<usize> = Vec::new();
        for o in &operations {
            match o {
                Operation::MULTIPLY => results.push(1),
                Operation::PLUS => results.push(0),
            }
        }

        for l in lines {
            for (index, num_str) in l.split_ascii_whitespace().enumerate() {
                let num = usize::from_str_radix(num_str, 10).unwrap();
                match operations[index] {
                    Operation::MULTIPLY => results[index] *= num,
                    Operation::PLUS =>  results[index] += num,
                }
            }
        }

        let answer = results.iter().sum();

        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(4277556, part1(BufReader::new(TEST.as_bytes()))?);

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
