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

const TEST2: &str = "\
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

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
    println!("Getting from {node} in {answer} paths");
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

fn part2_process_line<R: BufRead>(reader: R) -> Result<usize> {
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

    // because there is no loop, dac has to be before fft or the other way around
    let mut cache_dac: HashMap<String, usize> = HashMap::new();
    cache_dac.insert("out".to_string(), 0);
    cache_dac.insert("dac".to_string(), 1);
    let path_fft_to_dac = traverse_graph("fft".to_string(), &graph, &mut cache_dac);

    let mut cache_fft: HashMap<String, usize> = HashMap::new();
    cache_fft.insert("out".to_string(), 0);
    cache_fft.insert("fft".to_string(), 1);
    let path_dac_to_fft = traverse_graph("dac".to_string(), &graph, &mut cache_fft);

    let mut cache_out: HashMap<String, usize> = HashMap::new();
    cache_out.insert("out".to_string(), 1);

    let mut answer;
    if path_fft_to_dac == 0 {
        // dac is before
        answer = traverse_graph("svr".to_string(), &graph, &mut cache_dac);
        answer *= traverse_graph("dac".to_string(), &graph, &mut cache_fft);
        answer *= traverse_graph("fft".to_string(), &graph, &mut cache_out);
    } else if path_dac_to_fft == 0 {
        // fft is before
        answer = traverse_graph("svr".to_string(), &graph, &mut cache_fft);
        answer *= traverse_graph("fft".to_string(), &graph, &mut cache_dac);
        answer *= traverse_graph("dac".to_string(), &graph, &mut cache_out);
    } else {
        panic!("Something wrong in the reasoning!");
    }

    Ok(answer)
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
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        part2_process_line(reader)
    }

    assert_eq!(2, part2(BufReader::new(TEST2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
