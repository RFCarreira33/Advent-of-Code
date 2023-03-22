use std::collections::HashSet;

fn parse_input() -> (HashSet<(i32, i32)>, Vec<(char, i32)>) {
    let mut input = include_str!("../input.txt").lines();
    let mut paper: HashSet<(i32, i32)> = HashSet::new();
    let mut instructions: Vec<(char, i32)> = Vec::new();
    // parse points
    while let Some(line) = input.next() {
        let (x, y) = match line.split_once(",") {
            Some((x, y)) => (x, y),
            None => break,
        };
        let x = x.trim().parse().unwrap();
        let y = y.trim().parse().unwrap();
        paper.insert((x, y));
    }
    // parse instructions
    while let Some(line) = input.next() {
        line.split_whitespace().for_each(|word| {
            let (dir, value) = match word.split_once("=") {
                Some((dir, value)) => (dir, value),
                None => return,
            };
            instructions.push((dir.chars().next().unwrap(), value.parse().unwrap()));
        })
    }

    (paper, instructions)
}

#[allow(dead_code)]
fn part1(mut input: HashSet<(i32, i32)>, instructions: Vec<(char, i32)>) -> i32 {
    for (dir, value) in instructions {
        for (x, y) in input.clone().into_iter() {
            match dir {
                'x' => {
                    if x >= value {
                        input.remove(&(x, y));
                        input.insert(((x - value * 2).abs(), y.clone()));
                    }
                }
                'y' => {
                    if y >= value {
                        input.remove(&(x, y));
                        input.insert((x.clone(), (y - value * 2).abs()));
                    }
                }
                _ => panic!("Unknown direction"),
            }
        }
        // stop at first instruction
        break;
    }

    input.len() as i32
}

// function to turn my hashset into a 2d vector
fn to_2d_vec(coords: HashSet<(i32, i32)>) -> Vec<Vec<char>> {
    // find the max x and y coordinates
    let (max_x, max_y) = coords
        .iter()
        .fold((0, 0), |acc, &(x, y)| (acc.0.max(x), acc.1.max(y)));

    let max_x = max_x as usize;
    let max_y = max_y as usize;

    // initialize a 2D vector with '.' as default
    let mut vec2d = vec![vec!['.'; max_y + 1]; max_x + 1];

    // update the elements corresponding to the coordinates to '#'
    for (x, y) in coords {
        vec2d[x as usize][y as usize] = '#';
    }

    vec2d
}

fn part2(mut input: HashSet<(i32, i32)>, instructions: Vec<(char, i32)>) {
    for (dir, value) in instructions {
        for (x, y) in input.clone().into_iter() {
            match dir {
                'x' => {
                    if x >= value {
                        input.remove(&(x, y));
                        input.insert(((x - value * 2).abs(), y.clone()));
                    }
                }
                'y' => {
                    if y >= value {
                        input.remove(&(x, y));
                        input.insert((x.clone(), (y - value * 2).abs()));
                    }
                }
                _ => panic!("Unknown direction"),
            }
        }
    }
    // convert and print letters
    let image = to_2d_vec(input);
    for row in image {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}

fn main() {
    let (input, instructions) = parse_input();
    // println!("Part 1 = {}", part1(input, instructions));
    part2(input, instructions);
}
