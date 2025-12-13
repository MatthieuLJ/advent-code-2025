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

fn part2_read_first_line(
    lines: &mut VecDeque<String>,
    rays: &mut Vec<(usize, usize)>,
) -> Result<usize> {
    let line = lines.pop_front().unwrap();
    for (i, c) in line.chars().enumerate() {
        if c == 'S' {
            rays.push((i, 1));
            return Ok(i);
        }
    }

    Err(anyhow!("Could not find the starting ray"))
}

fn part2_go_down(line: &String, rays: &mut Vec<(usize, usize)>) -> Result<usize> {
    let mut count: usize = 0;

    for (i, c) in line.chars().enumerate() {
        if c == '^' {
            let indices: Vec<usize> = rays
                .iter()
                .enumerate()
                .filter_map(|(ind, (line, _count))| if *line == i { Some(ind) } else { None })
                .collect();
            if indices.len() > 0 {
                assert_eq!(indices.len(), 1);
            } else {
                continue;
            }
            count = count + 1;
            let prev_indices: Vec<usize> = rays
                .iter()
                .enumerate()
                .filter_map(|(ind, (line, _count))| if *line == i - 1 { Some(ind) } else { None })
                .collect();
            if prev_indices.len() > 0 {
                rays[prev_indices[0]].1 += rays[indices[0]].1;
            } else {
                rays.push((i - 1, rays[indices[0]].1));
            }

            let next_indices: Vec<usize> = rays
                .iter()
                .enumerate()
                .filter_map(|(ind, (line, _count))| if *line == i + 1 { Some(ind) } else { None })
                .collect();
            if next_indices.len() > 0 {
                rays[next_indices[0]].1 += rays[indices[0]].1;
            } else {
                rays.push((i + 1, rays[indices[0]].1));
            }
            rays.remove(indices[0]);
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
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut rays: Vec<(usize, usize)> = Vec::new();
        let mut lines: VecDeque<String> = reader.lines().collect::<Result<_, _>>().unwrap();
        part2_read_first_line(&mut lines, &mut rays).expect("Oops");
        for l in lines {
            part2_go_down(&l, &mut rays).unwrap();
        }
        
        let mut answer: usize = 0;
        for (_ray, count) in rays {
            answer += count;
        }
        Ok(answer)
    }

    assert_eq!(40, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
