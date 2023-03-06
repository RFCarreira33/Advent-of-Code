use std::fs;

const NUM_OF_BITS: usize = 12;

fn parse_input() -> Vec<String> {
    return fs::read_to_string("input.txt")
        .expect("Unable to read file")
        .split_terminator("\n")
        .map(|s| s.trim().to_string())
        .collect();
}

#[allow(dead_code)]
fn part1(input: Vec<String>) -> i32 {
    //create a vector of 12 bits initialized to 0 if the bit is > 0 then it is a 1 else it is a 0
    let mut bits: Vec<i32> = vec![0; NUM_OF_BITS];
    input.iter().for_each(|line| {
        for (i, c) in line.chars().enumerate() {
            if c == '1' {
                bits[i] += 1;
                continue;
            }
            bits[i] -= 1;
        }
    });

    let mut gamma_value: i32 = 0;
    let mut epsilon_value: i32 = 0;
    let mut bit_value: i32 = 1;
    //iterate over the bits in reverse order because binary
    bits.iter().rev().for_each(|bit| {
        if bit > &0 {
            gamma_value += bit_value;
        } else {
            epsilon_value += bit_value;
        }
        bit_value *= 2;
    });
    return gamma_value * epsilon_value;
}

// Part 2 -------------------------------------------------------
fn part2(input: Vec<String>) -> i32 {
    let mut new_input: Vec<String> = input.clone();
    let mut index: usize = 0;
    loop {
        new_input = recursive_oxygen(new_input, index);
        if new_input.len() == 1 {
            break;
        }
        index += 1;
    }
    let mut oxygen_value: i32 = 0;
    let mut bit_value: i32 = 1;
    //iterate over the bits in reverse order because binary
    // there will only be 1 string in the vector
    new_input[0].chars().rev().for_each(|c| {
        if c == '1' {
            oxygen_value += bit_value;
        }
        bit_value *= 2;
    });

    // Carbon
    let mut new_input: Vec<String> = input;
    let mut index: usize = 0;
    loop {
        new_input = recursive_carbon(new_input, index);
        if new_input.len() == 1 {
            break;
        }
        index += 1;
    }
    let mut carbon_value: i32 = 0;
    let mut bit_value: i32 = 1;
    // iterate over the bits in reverse order because binary
    // there will only be 1 string in the vector
    new_input[0].chars().rev().for_each(|c| {
        if c == '1' {
            carbon_value += bit_value;
        }
        bit_value *= 2;
    });
    return oxygen_value * carbon_value;
}

fn main() {
    let input: Vec<String> = parse_input();
    // println!("Part 1 = {}\n", part1(input));
    println!("Part 2 = {}\n", part2(input));
}

// Functions used in part2

fn recursive_oxygen(input: Vec<String>, index: usize) -> Vec<String> {
    let mut bit: i32 = 0;
    input.iter().for_each(|line| {
        for (i, c) in line.chars().enumerate() {
            if i > index {
                break;
            }
            if i != index {
                continue;
            }
            if c == '1' {
                bit += 1;
                continue;
            }
            bit -= 1;
        }
    });

    let bit_char = if bit >= 0 { '1' } else { '0' };
    let mut new_input: Vec<String> = Vec::new();

    input.iter().for_each(|line| {
        for (i, c) in line.chars().enumerate() {
            if i > index {
                break;
            }
            if i != index {
                continue;
            }
            if c != bit_char {
                continue;
            }
            new_input.push(line.to_string());
        }
    });
    return new_input;
}

fn recursive_carbon(input: Vec<String>, index: usize) -> Vec<String> {
    let mut bit: i32 = 0;
    input.iter().for_each(|line| {
        for (i, c) in line.chars().enumerate() {
            if i > index {
                break;
            }
            if i != index {
                continue;
            }
            if c == '1' {
                bit += 1;
                continue;
            }
            bit -= 1;
        }
    });

    let bit_char = if bit < 0 { '1' } else { '0' };
    let mut new_input: Vec<String> = Vec::new();

    input.iter().for_each(|line| {
        for (i, c) in line.chars().enumerate() {
            if i > index {
                break;
            }
            if i != index {
                continue;
            }
            if c != bit_char {
                continue;
            }
            new_input.push(line.to_string());
        }
    });
    return new_input;
}
