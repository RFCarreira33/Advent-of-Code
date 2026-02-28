use std::collections::HashMap;

const HELPER: [(i32, i32); 8] = [
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
    (1, 1),
    (-1, -1),
    (1, -1),
    (-1, 1),
];

fn parse_input() -> HashMap<(i32, i32), bool> {
    let mut paper_map: HashMap<(i32, i32), bool> = HashMap::new();

    std::fs::read_to_string("input.txt")
        .expect("Error reading file")
        .split_whitespace()
        .enumerate()
        .for_each(|(line_index, line)| {
            line.chars().enumerate().for_each(|(c_index, c)| {
                if c == '.' {
                    return;
                }

                paper_map.insert((line_index as i32, c_index as i32), true);
            });
        });

    paper_map
}

fn part1(input: HashMap<(i32, i32), bool>) -> u32 {
    let mut total: u32 = 0;

    input.iter().for_each(|((x, y), _)| {
        let mut count = 0;
        for (x_helper, y_helper) in HELPER {
            let new_x = x + x_helper;
            let new_y = y + y_helper;

            if count >= 4 {
                break;
            }

            if input.contains_key(&(new_x, new_y)) {
                count += 1;
            }
        }

        if count < 4 {
            total += 1;
        }
    });

    total
}

fn part2(input: HashMap<(i32, i32), bool>) -> u32 {
    let mut paper_map: HashMap<(i32, i32), bool> = input.clone();
    let mut total: u32 = 0;

    loop {
        let mut new_paper_map: HashMap<(i32, i32), bool> = HashMap::new();

        paper_map.iter().for_each(|((x, y), _)| {
            let mut count = 0;

            for (x_helper, y_helper) in HELPER {
                let new_x = x + x_helper;
                let new_y = y + y_helper;

                if count >= 4 {
                    break;
                }

                if paper_map.contains_key(&(new_x, new_y)) {
                    count += 1;
                }
            }

            if count >= 4 {
                new_paper_map.insert((*x, *y), true);
                return;
            }

            total += 1;
        });

        if new_paper_map.len() == paper_map.len() {
            break;
        }

        paper_map = new_paper_map.clone();
    }

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
