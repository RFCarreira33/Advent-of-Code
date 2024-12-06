use std::{cmp::Ordering, collections::{HashMap, HashSet}, time::Instant, usize, vec};

const HELPER: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn parse_input() -> (Vec<Vec<char>>, (i32, i32)) {
    let mut vec1: Vec<Vec<char>> = Vec::new();
    let mut start: (i32, i32) = (0,0);

    std::fs::read_to_string("input.txt")
        .expect("Error reading file")
        .split_terminator("\n")
        .enumerate()
        .for_each(|(i, line )| {
            let mut vec_line = Vec::new();
            line.chars().enumerate().for_each(|(j, c )| {
                if c == '^' {
                    start = (i as i32, j as i32);
                    vec_line.push('.');
                    return;
                }
                vec_line.push(c);
            });
            vec1.push(vec_line);
        });

    ( vec1, start)
}

fn is_in_bounds(x: i32, y: i32, max: i32) -> bool {
    if x < 0 || y < 0 || x > max || y > max {
        return false;
    }

    true
}

fn part1(map: Vec<Vec<char>>, start: (i32, i32)) -> i32 {
    let max = map.len() as i32 - 1;
    let mut dir = 0;
    let (mut x, mut y): (i32, i32) = (start.0, start.1);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    loop {
        let hx = x + HELPER[dir].0;
        let hy = y + HELPER[dir].1;
        if !is_in_bounds(hx, hy, max) {
            break;
        }

        if map[hx as usize][hy as usize] == '#' {
            dir = (dir + 1) % 4;
            continue;
        }

        visited.insert((x,y));
        x = hx;
        y = hy;
    }
    
    visited.len() as i32 + 1
}

fn part2(map: Vec<Vec<char>>, start: (i32, i32)) -> i32 {
    let mut total: i32 = 0;

    total
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Please provide an argument for which part to run");
        return;
    }

    let ( map, start ) = parse_input();
    let time = Instant::now();
    match args[1].as_str() {
        "1" => {
            println!("Part 1: {}", part1(map, start));
        }
        "2" => {
            println!("Part 2: {}", part2(map, start));
        }
        _ => println!("Invalid argument. Please use 1 or 2"),
    }

    println!("{:?}", time.elapsed());
}

