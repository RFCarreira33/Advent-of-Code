use std::collections::{HashSet, VecDeque};

const HELPERX: [i8; 4] = [0, 1, 0, -1];
const HELPERY: [i8; 4] = [-1, 0, 1, 0];

fn parse_input() -> Vec<Vec<i8>> {
    let mut input = include_str!("../input.txt").lines();
    let mut result = Vec::new();
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
fn part1(input: Vec<Vec<i8>>) -> i32 {
    let mut count: i32 = 0;
    let maxx = input.len();
    let maxy = input[0].len();
    for i in 0..maxx {
        for j in 0..maxy {
            let location = input[i][j];
            let mut is_valid = true;
            // check all neighbors
            for index in 0..4 {
                let ii: usize = (i as i8 + HELPERX[index]) as usize;
                let jj: usize = (j as i8 + HELPERY[index]) as usize;
                // neighbor is out of bounds
                if ii >= maxx || jj >= maxy {
                    continue;
                }
                let neighbor = input[ii][jj];
                if location >= neighbor {
                    is_valid = false;
                    break;
                }
            }
            if is_valid {
                count += location as i32 + 1;
            }
        }
    }
    count
}

fn part2(input: Vec<Vec<i8>>) -> i32 {
    let mut low_points: Vec<(usize, usize)> = Vec::new();
    let maxx = input.len();
    let maxy = input[0].len();
    for i in 0..maxx {
        for j in 0..maxy {
            let location = input[i][j];
            let mut is_valid = true;
            // check all neighbors
            for index in 0..4 {
                let ii: usize = (i as i8 + HELPERX[index]) as usize;
                let jj: usize = (j as i8 + HELPERY[index]) as usize;
                // neighbor is out of bounds
                if ii >= maxx || jj >= maxy {
                    continue;
                }
                let neighbor = input[ii][jj];
                if location >= neighbor {
                    is_valid = false;
                    break;
                }
            }
            if is_valid {
                // each low point has a basin
                low_points.push((i, j));
            }
        }
    }
    // check for basin
    let mut basin_sizes: Vec<i32> = Vec::new();
    for lp in low_points {
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        let mut size: i32 = 1;
        queue.push_back(lp);
        visited.insert(lp);
        while !queue.is_empty() {
            let (i, j) = queue.pop_front().unwrap();
            // check all neighbors
            for index in 0..4 {
                let ii: usize = (i as i8 + HELPERX[index]) as usize;
                let jj: usize = (j as i8 + HELPERY[index]) as usize;
                // neighbor is out of bounds
                if ii >= maxx || jj >= maxy {
                    continue;
                }
                // check if neighbor is a wall aka 9 or if it has been visited
                if !visited.contains(&(ii, jj)) && input[ii][jj] < 9 {
                    visited.insert((ii, jj));
                    queue.push_back((ii, jj));
                    size += 1;
                }
            }
        }
        basin_sizes.push(size);
    }
    basin_sizes.sort();
    basin_sizes.reverse();
    basin_sizes[0] * basin_sizes[1] * basin_sizes[2]
}

fn main() {
    let input: Vec<Vec<i8>> = parse_input();
    // println!("Part 1 = {}\n", part1(input));
    println!("Part 2 = {}\n", part2(input));
}
