use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
824824821-824824827,2121212118-2121212124\
";

// for part 1
fn add_ids(range: &str) -> Result<usize> {
    let bounds: Vec<&str> = range.split("-").collect();
    assert_eq!(bounds.len(), 2);

    println!(
        "Counting IDs between {0} and {1}",
        bounds.get(0).unwrap(),
        bounds.get(1).unwrap()
    );

    let mut min_bound: usize;
    let mut max_bound: usize;

    // looking at the lower bound first
    // this is the length of the string we got
    let min_len = bounds.get(0).unwrap().chars().count();
    if min_len % 2 != 0 {
        // the first time we can ever get a number that's repeated
        // is when there will be an even number of digits overall
        min_bound = usize::pow(10, (min_len as u32 - 1) / 2);
    } else {
        let lower_number = usize::from_str_radix(bounds.get(0).unwrap(), 10).unwrap();
        min_bound = usize::from_str_radix(&bounds.get(0).unwrap()[..min_len / 2], 10).unwrap();
        // check to see where the number min_bound|min_bound is compared to lower_number
        if min_bound * usize::pow(10, min_len as u32 / 2) + min_bound < lower_number {
            min_bound += 1;
        }
    }

    //println!("Lowest possible ID is {min_bound}");

    let max_len = bounds.get(1).unwrap().chars().count();
    if max_len % 2 != 0 {
        max_bound = usize::pow(10, (max_len as u32 - 1) / 2) - 1;
    } else {
        let higher_number = usize::from_str_radix(bounds.get(1).unwrap(), 10).unwrap();
        max_bound = usize::from_str_radix(&bounds.get(1).unwrap()[..max_len / 2], 10).unwrap();
        // check to see where the number min_bound|min_bound is compared to lower_number
        if max_bound * usize::pow(10, max_len as u32 / 2) + max_bound > higher_number {
            max_bound -= 1;
        }
    }

    //println!("Highest possible ID is {max_bound}");

    if min_bound > max_bound {
        Ok(0)
    } else {
        let mut answer: usize = 0;
        for i in min_bound..max_bound + 1 {
            let num_digits: u32 = i.to_string().len() as u32;
            answer += i * usize::pow(10, num_digits) + i;
        }
        Ok(answer)
    }
}

fn check_if_invalid_id(candidate: usize, num_slices: usize) -> Result<bool> {
    let num_len = candidate.to_string().len();
    let chunk = &candidate.to_string()[..num_len / num_slices];
    let repeated = chunk.repeat(num_slices);

    return Ok(usize::from_str_radix(&repeated, 10).unwrap() == candidate);
}

// for part 2
fn add_ids_abrit(range: &str) -> Result<usize> {
    let bounds: Vec<&str> = range.split("-").collect();
    assert_eq!(bounds.len(), 2);
    let mut answer: usize = 0;

    let lower_number = usize::from_str_radix(bounds.get(0).unwrap(), 10).unwrap();
    let higher_number = usize::from_str_radix(bounds.get(1).unwrap(), 10).unwrap();

    for candidate in lower_number..higher_number + 1 {
        let num_len = candidate.to_string().len();
        for num_slices in 2..num_len + 1 {
            if num_len % num_slices == 0 {
                if check_if_invalid_id(candidate, num_slices).unwrap() {
                    //println!("Adding id {}", candidate);
                    answer += candidate;
                    break;
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

    fn part1<R: BufRead>(mut reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let mut answer = 0;
        let mut input: String = String::new();
        reader.read_to_string(&mut input).unwrap();

        let ranges: Vec<String> = input.split(',').map(|s| s.to_string()).collect();
        for r in ranges {
            answer += add_ids(&r).unwrap();
        }

        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(1227775554, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut answer = 0;
        let mut input: String = String::new();
        reader.read_to_string(&mut input).unwrap();

        let ranges: Vec<String> = input.split(',').map(|s| s.to_string()).collect();
        for r in ranges {
            answer += add_ids_abrit(&r).unwrap();
        }

        Ok(answer)
    }

    assert_eq!(4174379265, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
