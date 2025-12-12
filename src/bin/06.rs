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
*   +   *   +  ";

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
                _ => panic!("Found something I don't understand {o}"),
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
                    Operation::PLUS => results[index] += num,
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
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut table: Vec<VecDeque<char>> = Vec::new();
        let lines: Vec<String> = reader.lines().flatten().collect();
        let mut num_lines: usize = 0;
        let mut answer: usize = 0;
        for l in lines {
            let new_line_of_chars: VecDeque<char> = l.chars().collect();
            table.push(new_line_of_chars);
            num_lines += 1;
        }

        loop {
            if table[num_lines - 1].len() == 0 {
                break;
            }
            let current_operation: Operation = match table[num_lines - 1].pop_front().unwrap() {
                '+' => Operation::PLUS,
                '*' => Operation::MULTIPLY,
                x => panic!("Got some unexpected character [{x}]"),
            };
            let mut num_chars = 0;
            loop {
                if table[num_lines - 1].len() == 0 {
                    num_chars += 1;
                    break;
                }
                if table[num_lines - 1][0] == ' ' {
                    table[num_lines - 1].pop_front();
                    num_chars += 1;
                } else {
                    break;
                }
            }
            println!("Reading {num_chars} characters");
            let mut result: usize = match current_operation {
                Operation::MULTIPLY => 1,
                Operation::PLUS => 0,
            };

            match current_operation {
                Operation::MULTIPLY => println!("Current operation is *"),
                Operation::PLUS => println!("Current operation is +"),
            };

            for i in 0..num_chars {
                let mut current_number: usize = 0;
                for l in 0..num_lines - 1 {
                    let current_char = table[l].pop_front().unwrap();
                    if current_char != ' ' {
                        current_number = current_number * 10
                            + current_char.to_string().parse::<usize>().unwrap();
                    }
                }
                println!("Read number {current_number}");
                result = match current_operation {
                    Operation::MULTIPLY => result * current_number,
                    Operation::PLUS => result + current_number,
                };
            }
            if table[num_lines - 1].len() > 0 {
                // remove the blanks
                for l in 0..num_lines - 1 {
                    table[l].pop_front();
                }
            }
            println!("Result is {result}");
            answer = answer + result;
        }
        Ok(answer)
    }

    assert_eq!(3263827, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
