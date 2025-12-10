use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp;

const DAY: &str = "05";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32";

struct Segment {
    start: usize,
    end: usize,
}

impl std::fmt::Debug for Segment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Segment")
            .field("start", &self.start)
            .field("end", &self.end)
            .finish()
    }
}

fn insert_new_range(start: usize, end: usize, ingredient_ranges: &mut Vec<Segment>) -> Result<usize> {
    let mut index: usize = 0;

    println!("Inserting {start} - {end}");

    loop {
        if index >= ingredient_ranges.len() {
            // insert the new segment at the end
            ingredient_ranges.push(Segment {
                start: start,
                end: end,
            });
            return Ok(ingredient_ranges.len());
        }
        if ingredient_ranges[index].end < start - 1 {
            index += 1;
        } else {
            break;
        }
    }

    // the start of the the new range is less or equal to the end of the segment
    // at index
    // ingredient_ranges[index].end >= start - 1
    if ingredient_ranges[index].start > end + 1 {
        // we just insert the new segment in between, no merging
        ingredient_ranges.insert(
            index,
            Segment {
                start: start,
                end: end,
            },
        );
        return Ok(ingredient_ranges.len());
    }

    // Now we have 
    // ingredient_ranges[index].end >= start - 1
    // and
    // ingredient_ranges[index].start <= end + 1
    // so there is an overlap
    ingredient_ranges[index].start = cmp::min(ingredient_ranges[index].start, start);
    let mut new_end = cmp::max(ingredient_ranges[index].end, end);
    

    loop {
        if index >= ingredient_ranges.len() - 1 {
            break;
        }
        if ingredient_ranges[index + 1].start > new_end + 1 {
            break;
        }
        new_end = cmp::max(ingredient_ranges[index + 1].end, new_end);
        ingredient_ranges.remove(index+1);
    }
    ingredient_ranges[index].end = new_end;
    
    Ok(ingredient_ranges.len())
}

fn read_ranges(lines: &mut VecDeque<String>, ingredient_ranges: &mut Vec<Segment>) -> Result<usize> {
    let re = Regex::new(r"^(\d*)-(\d*)$").unwrap();
    loop {
        let l = lines.pop_front().unwrap();
        if l.as_str() == "" {
            break;
        }
        let Some(caps) = re.captures(&l) else {
            panic!("Failed to read range {l}");
        };
        assert_eq!(caps.len(), 3);
        insert_new_range(
            usize::from_str_radix(&caps[1], 10).unwrap(),
            usize::from_str_radix(&caps[2], 10).unwrap(),
            ingredient_ranges,
        )
        .expect("Failed to insert range");
    }

    println!("Now ranges are {:?}", ingredient_ranges);
    Ok(0)
}

fn check_fresh_ingredients(lines: &mut VecDeque<String>, ingredient_ranges: &Vec<Segment>) -> Result<usize> {
    let mut answer = 0;
    loop {
        if lines.len() == 0 {
            break;
        }
        let l = lines.pop_front().unwrap();
        let id = usize::from_str_radix(&l, 10).unwrap();

        for r in ingredient_ranges {
            if (id <= r.end) && (id >= r.start) {
                println!("Found ingredient {id} in range");
                answer += 1;
                break;
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
        let mut ingredient_ranges: Vec<Segment> = Vec::new();
        let mut lines: VecDeque<String> = reader.lines().flatten().collect();
        read_ranges(&mut lines, &mut ingredient_ranges).expect("I failed!");
        let answer = check_fresh_ingredients(&mut lines, &ingredient_ranges).expect("Could not check ingredients");
        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(3, part1(BufReader::new(TEST.as_bytes()))?);

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
