use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2025::*;

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut dial: i32 = 50;
        let mut answer: usize = 0;

        // TODO: Solve Part 1 of the puzzle
        for line in reader.lines() {
            let l = line.unwrap();
            let (direction, count) = l.split_at(1);
            let mut factor : i32 = 1;
            match direction.chars().next() {
                Some('L') => factor = -1,
                Some('R') => factor = 1,
                _ => println!("Don't understand the direction"),
            }
            let speed = i32::from_str_radix(count, 10).unwrap();
            dial += factor * speed;
            dial %= 100;
            if dial == 0 {
                answer += 1;
            }
        }
        Ok(answer)
    }

    assert_eq!(3, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut dial: i32 = 50;
        let mut answer: usize = 0;

        // TODO: Solve Part 1 of the puzzle
        for line in reader.lines() {
            let l = line.unwrap();
            let (direction, count) = l.split_at(1);
            let mut factor : i32 = 1;
            match direction.chars().next() {
                Some('L') => factor = -1,
                Some('R') => factor = 1,
                _ => println!("Don't understand the direction"),
            }
            let speed = i32::from_str_radix(count, 10).unwrap();
            if factor == 1 {
                answer += ((dial + speed) / 100) as usize;
            } else {
                answer += ((100 + speed - dial) / 100) as usize;
                if dial == 0 {
                    answer -= 1;
                }
            }
            dial += factor * speed;
            dial = ((dial % 100) + 100)%100;
        }
        Ok(answer)
    }
    
    assert_eq!(6, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
