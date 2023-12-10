use std::{collections::{HashMap, VecDeque}, usize};

fn get_pipe_map() -> HashMap<char, Vec<(i32, i32)>> {
    let mut pipe_map: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    pipe_map.insert('S', vec![(0, 1), (1, 0), (0, -1), (-1, 0)]);
    pipe_map.insert('|', vec![(0, 1), (0, -1)]);
    pipe_map.insert('-', vec![(1, 0), (-1, 0)]);
    pipe_map.insert('L', vec![(0, -1), (1, 0)]);
    pipe_map.insert('J', vec![(0, -1), (-1, 0)]);
    pipe_map.insert('7', vec![(0, 1), (-1, 0)]);
    pipe_map.insert('F', vec![(0, 1), (1, 0)]);
    pipe_map.insert('.', vec![]);
    pipe_map
}

fn parse_input() -> (Vec<Vec<char>>, (i32, i32)) {
    let mut matrice: Vec<Vec<char>> = Vec::new();
    let mut starting_point: Option<(i32, i32)> = None;
    std::fs::read_to_string("input.txt")
        .expect("Error reading file")
        .split_terminator("\n")
        .enumerate()
        .for_each(|(li, line)| {
            if starting_point.is_some() {
                matrice.push(line.chars().take(line.len()-1).collect());
                return;
            }
            matrice.push(line.chars().take(line.len()-1).enumerate().map(|(i,c)| {
                if c == 'S' {
                    starting_point = Some((li as i32, i as i32));
                }
                c
            }).collect());
        });
    (matrice, starting_point.unwrap())
}

#[allow(dead_code)]
fn part1(input: Vec<Vec<char>>, start: (i32, i32)) -> u32 {
    let pipe_map = get_pipe_map();
    let (max_x, max_y) = (input[0].len() as i32, input.len() as i32);
    let mut steps = 0;
    let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
    let mut visited: Vec<Vec<bool>> = vec![vec![false; max_x as usize]; max_y as usize];
    queue.push_back(start);
    while !queue.is_empty() {
        let (x, y) = queue.pop_front().unwrap();
        if visited[y as usize][x as usize] {
            continue;
        }
        let c = input[y as usize][x as usize];
        visited[y as usize][x as usize] = true;
        if c == '.' {
            continue;
        }
        let moves = pipe_map.get(&c).unwrap();
        moves.iter().for_each(|(ax, ay)| {
            let (nx, ny) = (x + ax, y + ay);
            if nx < 0 || nx >= max_x || ny < 0 || ny >= max_y {
                return;
            }
            // check if the next pipe accepts input from the current pipe
            if !pipe_map.get(&input[ny as usize][nx as usize]).unwrap().contains(&(*ax * -1, *ay * -1)) {
                return;
            }
            queue.push_back((nx, ny));
        });
        steps += 1;
    }
    steps/2
}

fn part2(input: Vec<Vec<char>>, start: (i32, i32)) -> i32 {
    0
}

fn main() {
    let (input, start) = parse_input();
    println!("Part 1 = {}", part1(input, start));
    // println!("Part 2 = {}", part2(input, start));
}



