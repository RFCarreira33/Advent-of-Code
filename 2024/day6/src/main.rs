use std::{collections:: HashSet, usize};

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

fn run_loop(map: &Vec<Vec<char>>, pos: (i32, i32), dir: usize, max: i32, part2: bool) -> i32 {
    let mut visited: HashSet<((i32, i32), Option<usize>)> = HashSet::new();
    let (mut x, mut y) = pos;
    let mut dir = dir;

    loop {
        let hx = x + HELPER[dir].0;
        let hy = y + HELPER[dir].1;
        if !is_in_bounds(hx, hy, max) {
            if part2 {
                return 0
            }

            return visited.len() as i32 + 1;
        }

        if map[hx as usize][hy as usize] == '#' {
            dir = (dir + 1) % 4;
            continue;
        }

        if part2 && visited.contains(&((hx, hy), Some(dir))) {
            return 1;
        }

        if part2 {
            visited.insert(((x,y), Some(dir)));
        } else {
            visited.insert(((x,y), None));
        }

        x = hx;
        y = hy;
    }
}

fn part1(map: &Vec<Vec<char>>, start: &(i32, i32)) -> i32 {
    run_loop(&map, *start, 0, (map.len() as i32) - 1, false)
}

fn part2(mut map: Vec<Vec<char>>, start: &(i32, i32)) -> i32 {
    let mut blocked: HashSet<(i32, i32)> = HashSet::new();
    let max = map.len() as i32 - 1;

    (0..max).for_each(|i| {
        (0..max).for_each(|j| {
            if map[i as usize][j as usize] == '#' {
                return;
            }

            map[i as usize][j as usize] = '#';
            if run_loop(&map, *start, 0, max, true) == 1 {
                blocked.insert((i, j));
            }
            map[i as usize][j as usize] = '.';
        });
    });

    blocked.len() as i32
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Please provide an argument for which part to run");
        return;
    }

    let ( map, start ) = parse_input();
    match args[1].as_str() {
        "1" => {
            println!("Part 1: {}", part1(&map, &start));
        }
        "2" => {
            println!("Part 2: {}", part2(map, &start));
        }
        _ => println!("Invalid argument. Please use 1 or 2"),
    }
}

