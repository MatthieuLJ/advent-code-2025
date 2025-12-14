use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use core::num;
use regex::Regex;
use std::cmp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "08"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

fn read_file<R: BufRead>(reader: R) -> Vec<(usize, usize, usize)> {
    let mut result = Vec::new();
    let re = Regex::new(r"^(\d*),(\d*),(\d*)$").unwrap();
    for l in reader.lines().flatten() {
        let Some(caps) = re.captures(&l) else {
            panic!("Failed to read range {l}");
        };
        result.push((
            usize::from_str_radix(&caps[1], 10).unwrap(),
            usize::from_str_radix(&caps[2], 10).unwrap(),
            usize::from_str_radix(&caps[3], 10).unwrap(),
        ));
    }

    result
}

fn calculate_distances(coordinates: &Vec<(usize, usize, usize)>) -> Vec<Vec<usize>> {
    let num_points = coordinates.len();
    let mut result: Vec<Vec<usize>> = Vec::with_capacity(num_points);
    for _ in 0..num_points {
        let row: Vec<usize> = vec![0; num_points];
        result.push(row)
    }

    for i in 0..num_points {
        for j in i + 1..num_points {
            result[i][j] = (isize::pow(coordinates[i].0 as isize - coordinates[j].0 as isize, 2)
                + isize::pow(coordinates[i].1 as isize - coordinates[j].1 as isize, 2)
                + isize::pow(coordinates[i].2 as isize - coordinates[j].2 as isize, 2))
                as usize;
        }
    }

    result
}

fn find_min_distance(distances: &Vec<Vec<usize>>) -> (usize, usize) {
    let num_points = distances.len();
    let mut min_distance = usize::MAX;
    let (mut res1, mut res2) = (0, 0);

    for i in 0..num_points {
        for j in i + 1..num_points {
            if distances[i][j] < min_distance {
                (res1, res2) = (i, j);
                min_distance = distances[i][j];
            }
        }
    }

    (res1, res2)
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R, num_connections: usize) -> Result<usize> {
        let coordinates: Vec<(usize, usize, usize)> = read_file(reader);
        let mut distances = calculate_distances(&coordinates);
        let num_points = coordinates.len();
        let mut networks: Vec<usize> = (0..num_points).collect();
        let mut made_connections = 0;

        loop {
            let (point1, point2) = find_min_distance(&distances);
            distances[point1][point2] = usize::MAX;
            made_connections += 1;
            if networks[point1] == networks[point2] {
                continue;
            }

            println!("Connecting {:?} and {:?}", coordinates[point1], coordinates[point2]);

            let old_network = cmp::max(networks[point1], networks[point2]);
            let new_network = cmp::min(networks[point1], networks[point2]);
            for n in networks.iter_mut() {
                if *n == old_network {
                    *n = new_network;
                }
            }
            
            //println!("Networks are {:?}", networks);

            if made_connections >= num_connections {
                break;
            }
        }

        // find the largest networks
        let mut networks_sizes = vec![0; num_points];
        for n in networks {
            networks_sizes[n] += 1;
        }
        networks_sizes.sort();

        Ok(networks_sizes[num_points - 1]
            * networks_sizes[num_points - 2]
            * networks_sizes[num_points - 3])
    }

    assert_eq!(40, part1(BufReader::new(TEST.as_bytes()), 10)?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file, 1000)?);
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
