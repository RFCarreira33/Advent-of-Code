use std::collections::HashMap;

fn parse_input() -> Vec<String> {
    std::fs::read_to_string("input.txt")
        .expect("Error reading file")
        .split_terminator("\n")
        .map(|line| line.to_string())
        .collect()
}

#[allow(dead_code)]
fn part1(input: Vec<String>) -> usize {
    let mut total: usize = 0;
    input.iter().enumerate().for_each(|(i, line)| {
        let mut is_valid = true;
        line.split(":").skip(1).for_each(|formatted| formatted.split(";").for_each(|set| set.split(",").for_each(|play| {
            let (number, color) = play.trim().split_at(2);
            let number = number.trim().parse::<u32>().unwrap();
            if number > 14 {
                is_valid = false;
                return;
            }
            match color.trim() {
                "green" => {
                    if number > 13 {
                        is_valid = false
                    }
                },
                "red" => {
                    if number > 12 {
                        is_valid = false
                    }
                },
                _ => {}
            }
        })));
        if is_valid {
            total += i+1
        }
    });
    total
}

fn part2(input: Vec<String>) -> u32 {
    let mut total: u32 = 0;
    let mut cubes: HashMap<String, u32> = HashMap::from([("red".to_owned(), 0), ("green".to_owned(), 0), ("blue".to_owned(), 0)]);
    input.iter().for_each(|line| {
        cubes.iter_mut().for_each(|(_,val)| {
            *val = 0
        });
        line.split(":").skip(1).for_each(|game| game.split(";").for_each(|set| set.split(",").for_each(|play| {
            let (number, color) = play.trim().split_at(2);
            let color = color.trim();
            let number = number.trim().parse::<u32>().unwrap();
            if cubes[color] < number {
                cubes.insert(color.to_owned(), number);
            }
        })));
        total += cubes["green"]*cubes["blue"]*cubes["red"];
    });
    total
}

fn main() {
    let input: Vec<String> = parse_input();
    // println!("Part 1 = {}", part1(input));
    println!("Part 2 = {}", part2(input));
}

