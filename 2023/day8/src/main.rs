use std::collections::HashMap;

fn parse_input() -> (HashMap<String, (String, String)>, Vec<char>) {
    let mut map: HashMap<String, (String, String)> = HashMap::new();
    let mut input = include_str!("../input.txt").lines();
    let instructions: Vec<char> = input.next().unwrap().chars().collect();
    input.skip(1).for_each(|line| {
        let mut key: String = String::new();
        let mut cl: String = String::new();
        let mut cr: String = String::new();
        line.split_whitespace().enumerate().for_each(|(i, str)| {
            match i {
                0 => key = str.to_owned(),
                1 => return,
                2 => cl = str.chars().skip(1).take(3).collect::<String>(),
                3 => cr = str.chars().take(3).collect::<String>(),
                _ => unreachable!(),
            }
          });
        map.insert(key, (cl, cr));
    });
    (map, instructions)
}

#[allow(dead_code)]
fn part1(map :HashMap<String, (String, String)>, instructions:Vec<char>) -> u32 {
    let mut steps = 0;
    let mut node: &str = "AAA";
    for instruction in instructions.iter().cycle() {
        if node == "ZZZ" {
            break;
        }
        let (cl, cr) = map.get(node).unwrap();

        if *instruction == 'L' {
            node = cl;
        }else {
            node = cr;
        }
        steps += 1;
    }
    steps
}

fn part2(map :HashMap<String, (String, String)>, instructions:Vec<char>) -> u64 {
    let mut starting_nodes = map.keys().filter(|&key| key.ends_with('A')).collect::<Vec<&String>>();
    let mut node_steps:Vec<u64> = vec![0;starting_nodes.len()];
    starting_nodes.iter_mut().enumerate().for_each(|(i,node)| {
        let mut steps = 0;
        for instruction in instructions.iter().cycle() {
            if node.ends_with('Z') {
                node_steps[i] = steps;
                return;
            }

            let (cl, cr) = map.get(&node.clone()).unwrap();

            if *instruction == 'L' {
                *node = cl;
            }else {
                *node = cr;
            }
            steps += 1;
        }
    });

    let mut lcm = node_steps[0];
    node_steps.iter().skip(1).for_each(|&step| {
        lcm = lcm * step / gcd(lcm, step);
    });
    lcm
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn main() {
    let (map, input) = parse_input();
    // println!("Part 1 = {}", part1(map,input));
    println!("Part 2 = {}", part2(map,input));
}


