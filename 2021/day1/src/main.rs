use std::fs;

fn parse_input() -> Vec<i32> {
    return fs::read_to_string("input.txt")
        .expect("Unable to read file")
        .split_terminator("\n")
        .map(|s| s.trim().parse::<i32>().unwrap())
        .collect();
}

fn part1(input: Vec<i32>) -> i32 {
    let mut count: i32 = -1;
    let mut prev_number: i32 = 0;
    input.iter().for_each(|n| {
        if n > &prev_number {
            count += 1;
        }
        prev_number = *n;
    });
    return count;
}

fn part2(input: Vec<i32>) -> i32 {
    let mut count: i32 = -1;
    let mut prev_number: i32 = 0;
    for element in input.windows(3) {
        let sum = element.iter().sum();
        if sum > prev_number {
            count += 1;
        }
        prev_number = sum;
    }
    return count;
}

fn main() {
    let input: Vec<i32> = parse_input();
    // println!("Part 1 = {}\n", part1(input);
    println!("Part 2 = {}", part2(input));
}
