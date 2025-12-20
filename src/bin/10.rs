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
fn part1_all_button_combinations(num_lights: u32, target: usize, buttons: &Vec<usize>) -> Vec<usize> {
    let num_buttons = buttons.len();
    let mut answer: Vec<usize> = Vec::new();

    // we don't want to try pressing no buttons
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

    part2_fewest_button_presses(&joltage, &buttons)
}

fn distance_between_joltages(source: &Vec<usize>, dest: &Vec<usize>) -> usize {
    assert_eq!(source.len(), dest.len());
    let mut answer = 0;
    for i in 0..source.len() {
        answer = cmp::max(answer, dest[i] - source[i]);
    }
    answer
}

fn part2_fewest_button_presses(joltage: &Vec<usize>, buttons: &Vec<usize>) -> Result<usize> {
    let mut djikstra: HashMap<Vec<usize>, usize> = HashMap::new();
    let mut visited: HashMap<Vec<usize>, bool> = HashMap::new();
    let num_lights = joltage.len();
    let start = vec![0; joltage.len()];
    djikstra.insert(start, 0);

    loop {
        // find the smallest distance spent so far + minimum possible distance to target
        let mut min_path = usize::MAX;
        let mut min_index = Vec::new();
        let mut path_so_far = usize::MAX;
        for (index, d) in &djikstra {
            if *d < min_path && !visited.contains_key(index) {
                min_index = (*index).clone();
                min_path = *d;
                path_so_far = *d;
            }
        }
        print!(
            "So far {} and made {}, size of visited {}, going from {:?}\r",
            min_path,
            path_so_far,
            visited.len(),
            &min_index
        );

        //println!("Starting from {:?}", min_index);

        assert_ne!(path_so_far, usize::MAX);
        assert!(!visited.contains_key(&min_index));
        visited.insert(min_index.clone(), true);

        // try all the buttons and see if there is any path to update
        let new_distance = path_so_far + 1;

        'buttons: for b in buttons {
            let mut new_dest = min_index.clone();
            let mut on_target = true;

            for i in 0..num_lights {
                if (b & (1 << i)) != 0 {
                    new_dest[i] += 1;
                    if new_dest[i] > joltage[i] {
                        continue 'buttons;
                    }
                }
                if new_dest[i] != joltage[i] {
                    on_target = false;
                }
            }

            if on_target {
                println!("Got one machine with {new_distance}");
                return Ok(new_distance);
            }

            if !djikstra.contains_key(&new_dest) {
                //println!("Can get to {:?} in {new_distance}", new_dest);
                djikstra.insert(new_dest, new_distance);
            }
        }

        //println!("Visited is now {:?}", visited.keys());
    }
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
