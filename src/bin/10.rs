use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "10";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

fn part1_process_line(machine: String) -> Result<usize> {
    // Read the light first
    let re_lights = Regex::new(r"\[([.#]+)\]").unwrap();
    let Some(light_caps) = re_lights.captures(&machine) else {
        panic!("Failed to read lights {machine}");
    };
    let num_lights = light_caps.get(1).unwrap().len() as u32;
    let mut target: usize = 0;
    for l in light_caps.get(1).unwrap().as_str().chars().rev() {
        match l {
            '.' => target = target * 2,
            '#' => target = target * 2 + 1,
            x => panic!("Found a weird character in the lights {x}"),
        }
    }
    println!("Captured {} for lights, target is {target}", &light_caps[1]);

    // Read the buttons next
    let re_buttons = Regex::new(r"\((\s*\d+(?:,\s*\d+)*\s*)\)\s*").unwrap();
    let mut buttons: Vec<usize> = Vec::new();
    for button_caps in re_buttons.captures_iter(&machine) {
        let mut this_button = 0;
        for id_str in button_caps.get(1).unwrap().as_str().split(",") {
            let id_num = usize::from_str_radix(id_str, 10).unwrap();
            this_button |= 1 << id_num;
        }
        buttons.push(this_button);
    }
    println!("Captured buttons {:?} for buttons", { &buttons });

    part1_fewest_button_presses(num_lights, target, &buttons)
}

fn part1_fewest_button_presses(
    num_lights: u32,
    target: usize,
    buttons: &Vec<usize>,
) -> Result<usize> {
    let mut djikstra = vec![usize::MAX; usize::pow(2, num_lights)];
    djikstra[0] = 0;
    println!(
        "num_lights {num_lights}, len of djikstra {}",
        djikstra.len()
    );
    let mut visited = vec![false; usize::pow(2, num_lights)];

    loop {
        // find the shortest path so far
        let mut min_path = usize::MAX;
        let mut min_index = 0;
        for (index, d) in djikstra.iter().enumerate() {
            if *d < min_path && !visited[index] {
                min_index = index;
                min_path = *d;
            }
        }
        assert_ne!(min_path, usize::MAX);
        visited[min_index] = true;
        // try all the buttons and see if there is any path to update
        let new_distance = min_path + 1;
        for b in buttons {
            let new_dest = min_index ^ b;

            if new_dest == target {
                return Ok(new_distance);
            }

            if djikstra[new_dest] > new_distance {
                djikstra[new_dest] = new_distance;
            }
        }
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let machines: Vec<String> = reader.lines().flatten().collect();
        let mut answer = 0;
        for m in machines {
            answer += part1_process_line(m).unwrap();
        }
        Ok(answer)
    }

    assert_eq!(7, part1(BufReader::new(TEST.as_bytes()))?);

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
    // assert_eq!(33, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}
