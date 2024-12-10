use std::{
    collections::{HashMap, VecDeque},
    usize,
};

fn parse_input() -> (Vec<Option<usize>>, VecDeque<usize>) {
    let mut files: Vec<Option<usize>> = Vec::new();
    let mut empty_index: VecDeque<usize> = VecDeque::new();
    let mut real_index = 0;
    let mut real_number = 0;

    std::fs::read_to_string("input.txt")
        .expect("Error reading file")
        .split_terminator("\n")
        .for_each(|line| {
            line.chars().enumerate().for_each(|(i, c)| {
                let number = c.to_digit(10).unwrap() as usize;
                if i % 2 == 0 {
                    (0..number).for_each(|_| {
                        files.push(Some(real_index / 2));
                    });
                } else {
                    (0..number).for_each(|n| {
                        files.push(None);
                        empty_index.push_back(real_number + n);
                    });
                }

                real_number += number;
                real_index += 1;
            });
        });

    (files, empty_index)
}

fn part1(blocks: Vec<Option<usize>>, mut empty_index: VecDeque<usize>) -> usize {
    let mut new_order = blocks.clone();
    let mut end = false;

    blocks.iter().enumerate().rev().for_each(|(i, block)| {
        if end {
            return;
        }

        if block.is_none() {
            return;
        }

        let pos = empty_index.pop_front();
        if pos.is_none() {
            return;
        }

        let pos = pos.unwrap();
        if i < pos {
            end = true;
            return;
        }

        new_order[pos] = *block;
        new_order[i] = None;
    });

    new_order
        .iter()
        .enumerate()
        .map(|(i, block)| {
            if let Some(number) = block {
                return i * number;
            }
            0
        })
        .sum()
}

fn part2(blocks: Vec<Option<usize>>, mut empty_index: VecDeque<usize>) -> usize {
    0
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Please provide an argument for which part to run");
        return;
    }

    let (blocks, empty_index) = parse_input();
    match args[1].as_str() {
        "1" => {
            println!("Part 1: {}", part1(blocks, empty_index));
        }
        "2" => {
            println!("Part 2: {}", part2(blocks, empty_index));
        }
        _ => println!("Invalid argument. Please use 1 or 2"),
    }
}
