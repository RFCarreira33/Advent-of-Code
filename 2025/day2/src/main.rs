use std::collections::HashMap;

struct IdRange {
    start: i64,
    end: i64,
}

fn parse_input() -> Vec<IdRange> {
    let mut ids: Vec<IdRange> = Vec::new();

    std::fs::read_to_string("input.txt")
        .expect("Error reading file")
        .split(",")
        .for_each(|slice| {
            let bounds: Vec<i64> = slice
                .trim()
                .split("-")
                .map(|num_str| num_str.parse::<i64>().unwrap())
                .collect();

            ids.push(IdRange {
                start: bounds[0],
                end: bounds[1],
            });
        });

    ids
}

fn part1(input: Vec<IdRange>) -> i64 {
    let mut total: i64 = 0;

    input.iter().for_each(|id_range| {
        (id_range.start..=id_range.end).for_each(|i| {
            let id_str = i.to_string();
            let len = id_str.len();

            if len % 2 != 0 {
                return;
            }

            let half_len = len / 2;
            if id_str[..half_len] == id_str[half_len..] {
                total += i as i64;
            }
        })
    });

    total
}

fn part2(input: Vec<IdRange>) -> i64 {
    let mut total: i64 = 0;

    input.iter().for_each(|id_range| {
        (id_range.start..=id_range.end).for_each(|i| {
            let id_str = i.to_string();
            let len = id_str.len();

            if len % 2 != 0 {
                return;
            }

            let half_len = len / 2;
            if id_str[..half_len] == id_str[half_len..] {
                total += i as i64;
            }
        })
    });

    total
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
            println!("Part 1: {}", part1(input));
        }
        "2" => {
            println!("Part 2: {}", part2(input));
        }
        _ => println!("Invalid argument. Please use 1 or 2"),
    }
}
