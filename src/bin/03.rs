use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
987654321111111
811111111111119
234234234234278
818181911112111";

fn find_max_joltage(input: &str) -> Result<usize> {
    //println!("Finding max joltage in {input}");
    // convert the string into a Vec<usize>
    let digits: Vec<usize> = input
        .chars()
        .map(|c| usize::from_str_radix(&c.to_string(), 10).unwrap())
        .collect();
    let mut tens_max = digits.get(0).unwrap();
    let mut unit_max = digits.get(1).unwrap();
    for i in 1..digits.len() - 1 {
        if digits.get(i).unwrap() > tens_max {
            tens_max = digits.get(i).unwrap();
            unit_max = digits.get(i + 1).unwrap();
        } else if digits.get(i).unwrap() > unit_max {
            unit_max = digits.get(i).unwrap();
        }
    }
    if digits.get(digits.len() - 1).unwrap() > unit_max {
        unit_max = digits.get(digits.len() - 1).unwrap();
    }
    Ok(tens_max * 10 + unit_max)
}

fn find_max_joltage_n_batteries(digits: Vec<usize>, num_batteries: usize) -> Result<usize> {
    let mut top_max = digits.get(0).unwrap();
    let mut top_max_index: usize = 0;
    //println!("Looking for the best {num_batteries} in {:?}", digits);
    for i in 1..digits.len() - (num_batteries - 1) {
        if digits.get(i).unwrap() > top_max {
            top_max = digits.get(i).unwrap();
            top_max_index = i;
        }
    }
    if num_batteries > 1 {
        Ok((top_max * usize::pow(10, num_batteries as u32 - 1))
            + find_max_joltage_n_batteries(digits[top_max_index + 1..].to_vec(), num_batteries - 1)
                .unwrap())
    } else {
        Ok(*top_max)
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let lines = reader.lines();
        let mut answer = 0;
        for l in lines {
            answer += find_max_joltage(l.unwrap().as_str()).unwrap();
        }
        Ok(answer)
    }

    assert_eq!(357, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let lines = reader.lines();
        let mut answer = 0;
        for l in lines {
            let digits: Vec<usize> = l
                .unwrap()
                .chars()
                .map(|c| usize::from_str_radix(&c.to_string(), 10).unwrap())
                .collect();
            let new_bank = find_max_joltage_n_batteries(digits, 12).unwrap();
            answer += new_bank;
        }
        Ok(answer)
    }

    assert_eq!(3121910778619, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
