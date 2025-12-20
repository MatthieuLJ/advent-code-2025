use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;
use std::cmp;
use std::collections::HashMap;
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

// This is part of solving part 2
// Not my algorithm, I could not figure it out, got too deep into algorithms
// to solve systems of diophantine equations and matrix manipulation
// it was interesting, but not practical here (and too generic)
// The algorithm was described here:
// https://www.reddit.com/r/adventofcode/comments/1pk87hl/2025_day_10_part_2_bifurcate_your_way_to_victory/
fn part1_all_button_combinations(target: usize, buttons: &Vec<usize>) -> Vec<usize> {
    let num_buttons = buttons.len();
    let mut answer: Vec<usize> = Vec::new();

    // the bits of try_button will tell us which buttons we are trying to press
    for try_button in 1..usize::pow(2, num_buttons as u32) {
        let mut try_result: usize = 0;
        for button_bit in 0..num_buttons {
            if try_button & (1 << button_bit) != 0 {
                // trying with that button pressed
                try_result ^= buttons[button_bit];
            }
        }
        if try_result == target {
            answer.push(try_button);
        }
    }
    answer
}

fn part2_process_line(machine: String) -> Result<usize> {
    // Read the buttons
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

    // then read the joltages
    let re_joltage = Regex::new(r"\{([^}]+)\}").unwrap();
    let mut joltage: Vec<usize> = Vec::new();
    let joltage_cap = re_joltage.captures(&machine).unwrap();
    for jolt_str in joltage_cap.get(1).unwrap().as_str().split(",") {
        let jolt_num = usize::from_str_radix(jolt_str, 10).unwrap();
        joltage.push(jolt_num);
    }
    println!("Got joltage {:?}", joltage);

    let mut cache: HashMap<Vec<usize>, usize> = HashMap::new();

    Ok(part2_fewest_button_presses(&joltage, &buttons, &mut cache))
}

fn part2_fewest_button_presses(
    joltage: &Vec<usize>,
    buttons: &Vec<usize>,
    cache: &mut HashMap<Vec<usize>, usize>,
) -> usize {
    println!("Trying to find the solution to reach {:?}", joltage);
    if cache.contains_key(joltage) {
        return *cache.get(joltage).unwrap();
    }

    // first create the target based on the evenness of the joltages
    let mut target = 0;
    for (index, j) in joltage.into_iter().enumerate() {
        if *j % 2 == 1 {
            target |= 1 << index;
        }
    }

    let mut answer = usize::MAX;

    let mut all_even = true;
    for light_index in 0..joltage.len() {
        if joltage[light_index] % 2 != 0 {
            all_even = false;
            break;
        }
    }
    if all_even {
        let mut new_joltage = joltage.clone();
        for light_index in 0..joltage.len() {
            new_joltage[light_index] /= 2;
        }
        answer = part2_fewest_button_presses(&new_joltage, buttons, cache);
        if answer != usize::MAX {
            answer *= 2;
        }
    }

    let possible_button_presses = part1_all_button_combinations(target, buttons);

    'buttons: for test_buttons in possible_button_presses {
        let mut new_joltage = joltage.clone();
        let mut temp_answer = 0;
        for button_index in 0..buttons.len() {
            if test_buttons & (1 << button_index) != 0 {
                // this button was pressed
                temp_answer += 1;
                for light_index in 0..joltage.len() {
                    if buttons[button_index] & (1 << light_index) != 0 {
                        if new_joltage[light_index] == 0 {
                            continue 'buttons;
                        }
                        new_joltage[light_index] -= 1;
                    }
                }
            }
        }
        let mut all_clear = true;
        for light_index in 0..joltage.len() {
            if new_joltage[light_index] > 0 {
                all_clear = false;
            }
            if new_joltage[light_index] % 2 != 0 {
                panic!("All new_joltage should be even here");
            }
            new_joltage[light_index] /= 2;
        }
        if !all_clear {
            let recurse_answer = part2_fewest_button_presses(&new_joltage, buttons, cache);
            if recurse_answer == usize::MAX {
                continue 'buttons;
            }
            temp_answer += 2 * recurse_answer;
        }
        answer = cmp::min(temp_answer, answer);
    }

    println!("Could get to {:?} with {answer} button presses", joltage);
    cache.insert(joltage.clone(), answer);
    answer
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
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
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let machines: Vec<String> = reader.lines().flatten().collect();
        let mut answer = 0;
        for m in machines {
            answer += part2_process_line(m).unwrap();
        }
        Ok(answer)
    }

    assert_eq!(33, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
