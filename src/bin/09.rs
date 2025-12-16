use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;
use std::cmp;
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

// (7,9,11) and (1,9,11)
fn lines_intersect(line1: (usize, usize, usize), line2: (usize, usize, usize)) -> bool {
    //println!("Intersection between {:?} and {:?}", line1, line2);
    (line1.0 >= line2.1) && (line1.0 <= line2.2) && (line2.0 >= line1.1) && (line2.0 <= line1.2)
}

fn point_in_rectangle(
    top_left_corner: (usize, usize),
    bottom_right_corner: (usize, usize),
    point: (usize, usize),
) -> bool {
    point.0 > top_left_corner.0
        && point.0 < bottom_right_corner.0
        && point.1 > top_left_corner.1
        && point.1 < bottom_right_corner.1
}

fn crossing_lines(coordinates: &Vec<(usize, usize)>) -> Result<usize> {
    let mut horizontal_lines: Vec<(usize, usize, usize)> = Vec::new(); // (x, start, end)
    let mut vertical_lines: Vec<(usize, usize, usize)> = Vec::new(); // (y, start, end)

    for c in 0..coordinates.len() {
        let d = if c == coordinates.len() - 1 { 0 } else { c + 1 };
        if coordinates[c].0 == coordinates[d].0 {
            //vertical line
            let start = cmp::min(coordinates[c].1, coordinates[d].1);
            let end = cmp::max(coordinates[c].1, coordinates[d].1);
            vertical_lines.push((coordinates[c].0, start, end));
        } else {
            let start = cmp::min(coordinates[c].0, coordinates[d].0);
            let end = cmp::max(coordinates[c].0, coordinates[d].0);
            horizontal_lines.push((coordinates[c].1, start, end));
        }
    }

    println!("horizontal lines: {:?}", horizontal_lines);
    println!("vertical lines: {:?}", vertical_lines);

    let num_points = coordinates.len();
    let mut largest_area = 0;

    for i in 0..num_points {
        'rectangles: for j in i + 1..num_points {
            let min_x = cmp::min(coordinates[i].0, coordinates[j].0);
            let max_x = cmp::max(coordinates[i].0, coordinates[j].0);
            let min_y = cmp::min(coordinates[i].1, coordinates[j].1);
            let max_y = cmp::max(coordinates[i].1, coordinates[j].1);
            let new_area = (max_x - min_x + 1) * (max_y - min_y + 1);
            if new_area > largest_area {
                println!(
                    "Testing between {:?} and {:?}",
                    coordinates[i], coordinates[j]
                );

                // check if the lines forming the rectangles cross any lines of the polygon
                for v in &vertical_lines {
                    if ((*v).0 <= min_x) || ((*v).0 >= max_x) {
                        continue;
                    }
                    if ((*v).1 <= min_y) && ((*v).2 <= min_y) {
                        continue;
                    }
                    if ((*v).2 >= max_y) && ((*v).2 >= max_y) {
                        continue;
                    }
                    //println!("Vertical line crossing {:?}", v);
                    continue 'rectangles;
                }
                for h in &horizontal_lines {
                    if ((*h).0 <= min_y) || ((*h).0 >= max_y) {
                        continue;
                    }
                    if ((*h).1 <= min_x) && ((*h).2 <= min_x) {
                        continue;
                    }
                    if ((*h).1 >= max_x) && ((*h).2 >= max_x) {
                        continue;
                    }
                    //println!("Horizontal line crossing {:?}", h);
                    continue 'rectangles;
                }
                // check there is no line ending inside the rectangle
                for c in coordinates {
                    if point_in_rectangle((min_x, min_y), (max_x, max_y), *c) {
                        //println!("Found a point inside the rectangle");
                        continue 'rectangles;
                    }
                }

                if (min_x < max_x) && (min_y < max_y) {
                    // check if the center of the rectangle is inside the polygon
                    let center_x = (min_x + max_x) / 2;
                    let center_y = (min_y + max_y) / 2;
                    let mut count = 0;
                    for h in &horizontal_lines {
                        if lines_intersect(*h, (center_x, 0, center_y)) {
                            count += 1;
                        }
                    }
                    if count % 2 == 0 {
                        //println!("center is outside");
                        continue 'rectangles;
                    }
                }
                println!("new best area is {new_area}");
                largest_area = new_area;
            }
        }
    }

    Ok(largest_area)
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

    assert_eq!(50, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let coordinates = read_file(reader);
        crossing_lines(&coordinates)
    }

    assert_eq!(24, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
