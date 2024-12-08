use std::collections::{HashMap, HashSet};

fn parse_input() -> (HashMap<char, Vec<(i32, i32)>>, i32) {
    let mut positions: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut size = 0;

    std::fs::read_to_string("input.txt")
        .expect("Error reading file")
        .split_terminator("\n")
        .enumerate()
        .for_each(|(i, line )| {
            if i == 0 {
                // don't count the \r
                size = line.len() as i32 - 2;
            }

            line.chars().enumerate().for_each(|(j, c)| {
                if c == '.' || c == '\r'{
                    return;
                }

                positions.entry(c).or_insert(Vec::new()).push((i as i32, j as i32));
            });
        });

    ( positions, size )
}

fn is_in_bounds(pos: (i32, i32), max: i32) -> bool {
    if pos.0 < 0 || pos.1 < 0 || pos.0 > max || pos.1 > max {
        return false;
    }

    true
}

fn calculate_dif(c1: (i32, i32), c2: (i32, i32)) -> (i32, i32){
    let mut cc1 = c1;
    while cc1.0 != c2.0 || cc1.1 != c2.1 {
        if cc1.0 != c2.0 {
            cc1.0 = cc1.0 + (c2.0 - cc1.0).signum();
        }

        if cc1.1 != c2.1 {
            cc1.1 = cc1.1 + (c2.1 - cc1.1).signum();
        }
    }

    (cc1.0 - c1.0 , cc1.1 - c1.1)
}

fn get_freqs(c1: (i32, i32), c2: (i32, i32), dif: (i32, i32)) -> Vec<(i32, i32)> {
    vec![(c1.0 - dif.0, c1.1 - dif.1), (c2.0 + dif.0, c2.1 + dif.1)]
}

fn part1(positions: HashMap<char, Vec<(i32, i32 )>>, size: i32) -> usize {
    let mut frequency_points: HashSet<(i32, i32)> = HashSet::new();
    
    positions.iter().for_each(|(_, vec_pos)| {
        vec_pos.iter().for_each(|c1| {
            vec_pos.iter().for_each(|c2| {
                if c1 == c2 {
                    return;
                }

                let dif = calculate_dif(*c1, *c2);

                get_freqs(*c1, *c2, dif).iter().for_each(|f| {
                    if !is_in_bounds(*f, size) {
                        return;
                    }
                    frequency_points.insert(*f);
                });
            });
        });
    });

    frequency_points.len()
}

fn part2(positions: HashMap<char, Vec<(i32, i32 )>>, size: i32) -> usize {
    let mut frequency_points: HashSet<(i32, i32)> = HashSet::new();
    
    positions.iter().for_each(|(_, vec_pos)| {
        vec_pos.iter().for_each(|c1| {
            frequency_points.insert(*c1);
            vec_pos.iter().for_each(|c2| {
                if c1 == c2 {
                    return;
                }

                let mut c1 = *c1;
                let mut c2 = *c2;
                let dif = calculate_dif(c1, c2);

                loop {
                    if !is_in_bounds(c1, size) && !is_in_bounds(c2, size) {
                        break;
                    }

                    get_freqs(c1, c2, dif).iter().enumerate().for_each(|(i, f )| {
                        if is_in_bounds(*f, size) {
                            frequency_points.insert(*f);
                        }

                        match i {
                            0 => c1 = *f,
                            1 => c2 = *f,
                            _ => (),
                        }
                    });
                }

            });
        });
    });

    frequency_points.len()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Please provide an argument for which part to run");
        return;
    }

    let ( positions, size ) = parse_input();
    match args[1].as_str() {
        "1" => {
            println!("Part 1: {}", part1(positions, size));
        }
        "2" => {
            println!("Part 2: {}", part2(positions, size));
        }
        _ => println!("Invalid argument. Please use 1 or 2"),
    }
}


