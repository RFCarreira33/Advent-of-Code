use std::collections::HashSet;
use std::f64::INFINITY;

const STEP: f64 = 1.0;

fn parse_input() -> Vec<i32> {
    let input = include_str!("../input.txt");
    input
        .split_terminator(",")
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

#[allow(dead_code)]
fn part1(input: Vec<i32>) -> i32 {
    // set a worst case efficiency
    let mut efficiency: i32 = INFINITY as i32;
    // set to loop over
    let max: i32 = *input.iter().max().unwrap();
    let min: i32 = *input.iter().min().unwrap();
    let mut no_dups: HashSet<i32> = HashSet::new();

    for i in min..max {
        no_dups.insert(i);
    }

    for i in no_dups {
        let mut fuel_total: i32 = 0;
        for n in &input {
            let value = (*n - i).abs();
            if value == 0 {
                continue;
            }
            fuel_total += value;
            // if the fuel total is to high already we can stop
            if fuel_total >= efficiency {
                break;
            }
        }
        if fuel_total < efficiency {
            efficiency = fuel_total;
        }
    }
    efficiency
}

fn part2(input: Vec<i32>) -> f64 {
    // major dif is that we gotta use floats because of the division at line 68
    // set a worst case efficiency
    let mut efficiency: f64 = INFINITY as f64;
    // set to loop over
    let max: i32 = *input.iter().max().unwrap();
    let min: i32 = *input.iter().min().unwrap();
    let mut no_dups: HashSet<i32> = HashSet::new();

    for i in min..max {
        no_dups.insert(i);
    }

    for i in no_dups {
        let mut fuel_total: f64 = 0.0;
        for n in &input {
            let value: f64 = (*n - i).abs() as f64;
            if value == 0.0 {
                continue;
            }
            fuel_total += (value / 2.0) * (value + STEP);
            // if the fuel total is to high already we can stop
            if fuel_total >= efficiency {
                break;
            }
        }
        if fuel_total < efficiency {
            efficiency = fuel_total;
        }
    }
    efficiency
}

fn main() {
    let input = parse_input();
    // println!("Part 1 = {}\n", part1(input));
    println!("Part 2 = {}\n", part2(input));
}
