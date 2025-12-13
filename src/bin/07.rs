use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "07";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

fn read_first_line(lines: &mut VecDeque<String>, rays: &mut Vec<usize>) -> Result<usize> {
    let line = lines.pop_front().unwrap();
    for (i, c) in line.chars().enumerate() {
        if c == 'S' {
            rays.push(i);
            return Ok(i);
        }
    }

    Err(anyhow!("Could not find the starting ray"))
}

fn go_down(line: &String, rays: &mut Vec<usize>) -> Result<usize> {
    let mut count: usize = 0;

    for (i, c) in line.chars().enumerate() {
        if (c == '^') && (rays.contains(&i)) {
            count = count + 1;
            if !rays.contains(&(i - 1)) {
                rays.push(i - 1);
            }
            rays.push(i + 1);
            rays.retain(|value| *value != i);
        }
    }

    Ok(count)
}
fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut rays: Vec<usize> = Vec::new();
        let mut answer: usize = 0;
        let mut lines: VecDeque<String> = reader.lines().collect::<Result<_, _>>().unwrap();
        read_first_line(&mut lines, &mut rays).expect("Oops");
        for l in lines {
            answer += go_down(&l, &mut rays).unwrap();
        }
        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(21, part1(BufReader::new(TEST.as_bytes()))?);

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
