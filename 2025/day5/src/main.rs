use std::cmp::{max, min};

fn parse_input() -> (Vec<(u64, u64)>, Vec<u64>) {
    let mut ranges: Vec<(u64, u64)> = Vec::new();
    let mut parsing_inputs = false;
    let mut inputs = Vec::new();

    std::fs::read_to_string("input.txt")
        .expect("Error reading file")
        .split_whitespace()
        .for_each(|line| {
            let parts: Vec<&str> = line.split('-').collect();
            if parts.len() == 2 {
                let start = parts[0].parse::<u64>().expect("Error parsing range start");
                let end = parts[1].parse::<u64>().expect("Error parsing range end");
                ranges.push((start, end));
                return;
            }

            inputs.push(line.parse::<u64>().expect("Error parsing input"));
        });

    (ranges, inputs)
}

fn part1(ranges: Vec<(u64, u64)>, inputs: Vec<u64>) -> u64 {
    let mut total: u64 = 0;

    inputs.iter().for_each(|input| {
        if ranges
            .iter()
            .any(|(start, end)| input >= start && input <= end)
        {
            total += 1;
        }
    });

    total
}

fn part2(mut ranges: Vec<(u64, u64)>) -> u64 {
    let mut merged_ranges: Vec<(u64, u64)> = Vec::new();

    ranges.sort_by(|a, b| a.0.cmp(&b.0));
    let (mut cur_start, mut cur_end) = ranges[0];

    ranges.iter().skip(1).for_each(|(start, end)| {
        if *start <= cur_end + 1 {
            cur_end = max(cur_end, *end);
            return;
        }

        merged_ranges.push((cur_start, cur_end));
        cur_start = *start;
        cur_end = *end;
    });

    merged_ranges.push((cur_start, cur_end));

    merged_ranges
        .iter()
        .map(|(start, end)| end - start + 1)
        .sum()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Please provide an argument for which part to run");
        return;
    }

    let input = parse_input();
    match args[1].as_str() {
        "1" => {
            println!("Part 1: {}", part1(input.0, input.1));
        }
        "2" => {
            println!("Part 2: {}", part2(input.0));
        }
        _ => println!("Invalid argument. Please use 1 or 2"),
    }
}

