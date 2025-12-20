use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "11";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

fn traverse_graph(
    node: String,
    graph: &HashMap<String, Vec<String>>,
    cache: &mut HashMap<String, usize>,
) -> usize {
    if cache.contains_key(&node) {
        return *cache.get(&node).unwrap();
    }
    let mut answer = 0;
    for d in graph.get(&node).unwrap() {
        answer += traverse_graph(d.to_string(), graph, cache)
    }
    cache.insert(node.clone(), answer);
    println!("Gettint to {node} in {answer} paths");
    answer
}

fn part1_process_line<R: BufRead>(reader: R) -> Result<usize> {
    // Read the light first
    let re_line = Regex::new(r"([a-z]{3}):(.*)").unwrap();
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for line in reader.lines().flatten() {
        let Some(caps) = re_line.captures(&line) else {
            panic!("Failed to read line {line}");
        };
        let node = caps.get(1).unwrap().as_str().to_string();
        let dests = caps.get(2).unwrap().as_str().to_string();
        let mut vdests: Vec<String> = Vec::new();
        for d in dests.trim().split(" ") {
            vdests.push(d.to_string());
        }
        println!("Captured {} going to {:?}", node, vdests);
        graph.insert(node, vdests);
    }
    let mut cache: HashMap<String, usize> = HashMap::new();
    cache.insert("out".to_string(), 1);
    Ok(traverse_graph("you".to_string(), &graph, &mut cache))
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        part1_process_line(reader)
    }

    assert_eq!(5, part1(BufReader::new(TEST.as_bytes()))?);

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
