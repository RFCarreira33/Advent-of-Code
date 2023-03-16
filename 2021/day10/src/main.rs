use std::collections::VecDeque;

const OPENERS: [char; 4] = ['(', '[', '{', '<'];

fn parse_input() -> Vec<String> {
    let mut input = include_str!("../input.txt").lines();
    let mut result: Vec<String> = Vec::new();
    while let Some(line) = input.next() {
        result.push(line.to_string());
    }
    result
}

// fn for part 1
fn get_oposite(c: char) -> char {
    match c {
        '{' => '}',
        '(' => ')',
        '<' => '>',
        '[' => ']',
        _ => '0',
    }
}

#[allow(dead_code)]
fn part1(input: Vec<String>) -> i32 {
    let mut ilegal: Vec<char> = Vec::new();
    let mut result: i32 = 0;
    input.iter().for_each(|s| {
        let mut queue: VecDeque<char> = VecDeque::new();
        s.chars().for_each(|c| {
            // if its an openar char add to queue
            if OPENERS.contains(&c) {
                queue.push_back(get_oposite(c));
            } else {
                // if its not a opener check if it closes the last opener
                if queue.pop_back().unwrap() != c {
                    ilegal.push(c);
                    match c {
                        ')' => result += 3,
                        '>' => result += 25137,
                        ']' => result += 57,
                        '}' => result += 1197,
                        _ => eprint!("Error"),
                    }
                    return;
                }
            }
        });
    });
    result
}

fn part2(input: Vec<String>) -> i64 {
    let mut scores: Vec<i64> = Vec::new();
    input.iter().for_each(|s| {
        let mut queue: VecDeque<char> = VecDeque::new();
        let mut incomplete: bool = true;
        s.chars().for_each(|c| {
            // if its an openar char add to queue
            if OPENERS.contains(&c) {
                queue.push_back(get_oposite(c));
            } else {
                // if its not a opener check if it closes the last opener
                if queue.pop_back().unwrap() != c {
                    incomplete = false;
                    return;
                }
            }
        });
        if incomplete {
            // using i64 because the result is too big for i32
            let mut result: i64 = 0;
            while !queue.is_empty() {
                result *= 5;
                match queue.pop_back().unwrap() {
                    ')' => result += 1,
                    '>' => result += 4,
                    ']' => result += 2,
                    '}' => result += 3,
                    _ => eprint!("Error"),
                }
            }
            scores.push(result);
        }
    });
    scores.sort();
    scores[scores.len() / 2]
}

fn main() {
    let input: Vec<String> = parse_input();
    // println!("Part 1 = {}", part1(input));
    println!("Part 2 = {}", part2(input));
}
