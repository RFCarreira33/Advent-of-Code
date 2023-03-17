use std::collections::{HashSet, VecDeque};

// helper to check adjacent cells
const HELPER: [(i8, i8); 8] = [
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
    (1, 1),
    (-1, -1),
    (1, -1),
    (-1, 1),
];
const MAX_STEPS: usize = 100;

fn parse_input() -> Vec<Vec<i8>> {
    let mut input = include_str!("../input.txt").lines();
    let mut result: Vec<Vec<i8>> = Vec::new();
    while let Some(line) = input.next() {
        let row: Vec<i8> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i8)
            .collect();
        result.push(row);
    }
    result
}

#[allow(dead_code)]
fn part1(mut input: Vec<Vec<i8>>) -> i32 {
    let mut flash_counter: i32 = 0;
    let max = (input.len() - 1) as i8;
    for _step in 0..MAX_STEPS {
        let mut flashed: HashSet<(usize, usize)> = HashSet::new();
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        // first encrease all values and store that will flash
        for x in 0..input.len() {
            for y in 0..input[x].len() {
                let num = &mut input[x][y];
                *num += 1;
                if *num == 10 {
                    queue.push_back((x, y));
                    flashed.insert((x, y));
                    *num = 0;
                    flash_counter += 1;
                }
            }
        }
        while !queue.is_empty() {
            let (x, y) = queue.pop_front().unwrap();
            for (hx, hy) in HELPER.iter() {
                let x = x as i8 + hx;
                let y = y as i8 + hy;
                // verify if its out of bounds before converting to usize
                if x < 0 || y < 0 || x > max || y > max {
                    continue;
                }
                // if its not out of bounds we can convert it to usize safely
                let x = x as usize;
                let y = y as usize;
                if flashed.contains(&(x, y)) {
                    continue;
                }
                let num = &mut input[x][y];
                *num += 1;
                if *num == 10 {
                    queue.push_back((x, y));
                    flashed.insert((x, y));
                    *num = 0;
                    flash_counter += 1;
                }
            }
        }
    }
    flash_counter
}

fn part2(mut input: Vec<Vec<i8>>) -> i32 {
    let mut step_counter: i32 = 1;
    let max = (input.len() - 1) as i8;
    loop {
        let mut flashed: HashSet<(usize, usize)> = HashSet::new();
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        // first encrease all values and store that will flash
        for x in 0..input.len() {
            for y in 0..input[x].len() {
                let num = &mut input[x][y];
                *num += 1;
                if *num == 10 {
                    queue.push_back((x, y));
                    flashed.insert((x, y));
                    *num = 0;
                }
            }
        }
        while !queue.is_empty() {
            let (x, y) = queue.pop_front().unwrap();
            for (hx, hy) in HELPER.iter() {
                let x = x as i8 + hx;
                let y = y as i8 + hy;
                // verify if its out of bounds before converting to usize
                if x < 0 || y < 0 || x > max || y > max {
                    continue;
                }
                // if its not out of bounds we can convert it to usize safely
                let x = x as usize;
                let y = y as usize;
                if flashed.contains(&(x, y)) {
                    continue;
                }
                let num = &mut input[x][y];
                *num += 1;
                if *num == 10 {
                    queue.push_back((x, y));
                    flashed.insert((x, y));
                    *num = 0;
                }
            }
        }
        // if the total of flashed is 10x10 = 100 end
        if flashed.len() == 100 {
            return step_counter;
        }
        step_counter += 1;
    }
}

fn main() {
    let input: Vec<Vec<i8>> = parse_input();
    // println!("Part 1 = {}", part1(input));
    println!("Part 2 = {}", part2(input));
}
