use std::fs;

fn parse_input() -> Vec<String> {
    return fs::read_to_string("input.txt")
        .expect("Unable to read file")
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
}

#[allow(dead_code)]
fn part1(input: Vec<String>) -> i32 {
    let mut depth: i32 = 0;
    let mut horizontal: i32 = 0;
    for i in (0..input.len()).step_by(2) {
        let direction: &str = &input[i];
        let value: i32 = input[i + 1].parse::<i32>().unwrap();
        // depending on the command the submarine moves
        match direction {
            "forward" => horizontal += value,
            "up" => depth -= value,
            "down" => depth += value,
            _ => eprintln!("Error"),
        }
    }

    return depth * horizontal;
}

fn part2(input: Vec<String>) -> i32 {
    let mut aim: i32 = 0;
    let mut depth: i32 = 0;
    let mut horizontal: i32 = 0;
    for i in (0..input.len()).step_by(2) {
        let direction: &str = &input[i];
        let value: i32 = input[i + 1].parse::<i32>().unwrap();
        // depending on the command the submarine moves
        match direction {
            "forward" => {
                horizontal += value;
                depth += aim * value;
            }
            "up" => aim -= value,
            "down" => aim += value,
            _ => eprintln!("Error"),
        }
    }

    return depth * horizontal;
}

fn main() {
    let input: Vec<String> = parse_input();
    // println!("Part 1 = {}\n", part1(input));
    println!("Part 2 = {}\n", part2(input));
}
