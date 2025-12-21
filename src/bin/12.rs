use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::{fold, izip};
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "12";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut piece_sizes: Vec<usize> = Vec::new();
        // read the pieces first and extract the sizes (number of #)
        let mut lines = reader.lines();
        let re_line = Regex::new(r"^([0-9]+):").unwrap();
        let mut next_index = 0;
        let mut line;

        loop {
            line = lines.next().unwrap().unwrap();
            let Some(caps) = re_line.captures(&line) else {
                break;
            };
            assert_eq!(
                next_index,
                usize::from_str_radix(caps.get(1).unwrap().as_str(), 10).unwrap()
            );
            let mut block_count = 0;
            for _ in 0..3 {
                for c in lines.next().unwrap().unwrap().chars() {
                    if c == '#' {
                        block_count += 1;
                    }
                }
            }

            piece_sizes.push(block_count);
            next_index += 1;
            let _ = lines.next();
        }

        println!("Got pieces {:?}", piece_sizes);

        let re_puzzle = Regex::new(r"^([0-9]+)x([0-9]+):(.*)").unwrap();
        let mut answer = 0;
        loop {
            let Some(caps) = re_puzzle.captures(&line) else {
                panic!("Could not read {line}");
            };
            let width = usize::from_str_radix(caps.get(1).unwrap().as_str(), 10).unwrap();
            let height = usize::from_str_radix(caps.get(2).unwrap().as_str(), 10).unwrap();
            let mut num_pieces: Vec<usize> = Vec::new();
            for p in caps.get(3).unwrap().as_str().to_string().trim().split(" ") {
                num_pieces.push(usize::from_str_radix(p, 10).unwrap());
            }

            let total_num_pieces = fold(&num_pieces, 0, |a, &b| a + b);
            if (width / 3) * (height / 3) >= total_num_pieces {
                answer += 1;
            } else {
                let mut num_blocks = 0;
                for (si, nu) in izip!(&piece_sizes, &num_pieces) {
                    num_blocks += *si * *nu;
                }
                if num_blocks < width * height {
                    println!("I had {total_num_pieces} to fit in {}", (width / 3) * (height / 3));
                    panic!("We would need much more processing {num_blocks} to fit in {width} x {height}");
                }
            }

            let l = lines.next();
            if l.is_none() {
                break;
            } else {
                line = l.unwrap().unwrap();
            }
        }

        Ok(answer)
    }

    //assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

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
