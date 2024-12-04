use std::{collections::BinaryHeap, time::Instant, usize};

const HELPER: [(i32, i32); 8] = [
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
    (1, 1),
    (-1, -1),
    (-1, 1),
    (1, -1),
];

#[derive(Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord, Debug)]
struct Node {
    x: i32,
    y: i32,
    direction: Option<(i32, i32)>,
}

fn parse_input() -> Vec<Vec<char>> {
    let mut vec1: Vec<Vec<char>> = Vec::new();

    std::fs::read_to_string("input.txt")
        .expect("Error reading file")
        .split_terminator("\n")
        .for_each(|line| vec1.push(line.chars().collect()));

    vec1
}

fn part1(input: Vec<Vec<char>>) -> i32 {
    let mut total: i32 = 0;
    let max = (input[0].len() - 1) as i32;
    let starting_points = get_starting_points(&input, 'X');

    starting_points.iter().for_each(|s| {
        let mut to_visit: BinaryHeap<Node> = BinaryHeap::new();
        to_visit.push(*s);

        while !to_visit.is_empty() {
            let node: Node = to_visit.pop().unwrap();
            let expected_char = match input[node.x as usize][node.y as usize] {
                'X' => 'M',
                'M' => 'A',
                'A' => 'S',
                _ => continue,
            };

            HELPER.iter().for_each(|(hx, hy)| {
                let direction = node.direction;
                if direction.is_some() && direction != Some((*hx, *hy)) {
                    return;
                }

                let x = node.x + hx;
                let y = node.y + hy;
                if !is_in_bounds(x, y, max) {
                    return;
                }

                if input[x as usize][y as usize] == expected_char {
                    if expected_char == 'S' {
                        total += 1;
                        return;
                    }

                    to_visit.push(Node {
                        x,
                        y,
                        direction: Some((*hx, *hy)),
                    })
                }
            });
        }
    });

    total
}

fn part2(input: Vec<Vec<char>>) -> i32 {
    let mut total: i32 = 0;
    let max = (input[0].len() - 1) as i32;
    let starting_points = get_starting_points(&input, 'A');

    starting_points.iter().for_each(|s| {
        let mut valid_sides = 0;

        HELPER.windows(2).skip(4).step_by(2).for_each(|diagonals| {
            let x1 = diagonals[0].0 + s.x;
            let x2 = diagonals[1].0 + s.x;
            let y1 = diagonals[0].1 + s.y;
            let y2 = diagonals[1].1 + s.y;

            if !is_in_bounds(x1, y1, max) {
                return;
            }
            if !is_in_bounds(x2, y2, max) {
                return;
            }

            let char1 = input[x1 as usize][y1 as usize];
            let char2 = input[x2 as usize][y2 as usize];

            match (char1, char2) {
                ('M', 'S') => valid_sides += 1,
                ('S', 'M') => valid_sides += 1,
                _ => return,
            }
        });

        if valid_sides == 2 {
            total += 1;
        }
    });

    total
}

fn get_starting_points(input: &Vec<Vec<char>>, to_find: char) -> BinaryHeap<Node> {
    let mut starting_nodes = BinaryHeap::new();

    input.iter().enumerate().for_each(|(x, line)| {
        line.iter().enumerate().for_each(|(y, char)| {
            if *char == to_find {
                starting_nodes.push(Node {
                    x: x as i32,
                    y: y as i32,
                    direction: None,
                });
            }
        });
    });

    starting_nodes
}

fn is_in_bounds(x: i32, y: i32, max: i32) -> bool {
    if x < 0 || y < 0 || x > max || y > max {
        return false;
    }

    true
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
