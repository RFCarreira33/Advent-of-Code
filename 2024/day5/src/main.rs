use std::{cmp::Ordering, collections::HashMap, usize, vec};

fn parse_input() -> (Vec<Vec<i32>>, HashMap<i32, Vec<i32>>) {
    let mut vec1: Vec<Vec<i32>> = Vec::new();
    let mut priority_order: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut has_parsed = false;
    let mut index = 0;

    std::fs::read_to_string("input.txt")
        .expect("Error reading file")
        .split_terminator("\n")
        .for_each(|line| {
            if line == "" {
                has_parsed = true;
                return;
            }

            if !has_parsed {
                let parts = line.split("|").collect::<Vec<&str>>();
                let entry: i32 = parts[0].parse().unwrap();
                let child: i32 = parts[1].parse().unwrap();

                if let Some(existing_entry) = priority_order.get_mut(&entry) {
                    existing_entry.push(child);
                    return;
                }

                priority_order.insert(entry, vec![child]);
                return;
            }

            vec1.push(Vec::new());
            line.split(",").for_each(|n| {
                vec1[index].push(n.parse().unwrap());
            });
            index += 1;
        });

    (vec1, priority_order)
}

fn part1(pages: Vec<Vec<i32>>, prio: HashMap<i32, Vec<i32>>) -> i32 {
    let mut total: i32 = 0;

    pages.iter().for_each(|page| {
        let mut is_valid = true;
        page.iter().enumerate().for_each(|(i, num)| {
            if !is_valid {
                return;
            }

            page.iter().skip(i + 1).for_each(|n| {
                if !is_valid {
                    return;
                }

                if let Some(entry) = prio.get(&n) {
                    is_valid = !find_in_vec(entry, *num);
                }
            });
        });

        if is_valid {
            total += page[page.len() / 2 as usize]
        }
    });

    total
}

fn find_in_vec(vec: &Vec<i32>, to_find: i32) -> bool {
    if let Some(_) = vec.iter().find(|&&n| n == to_find) {
        return true;
    }

    false
}

fn part2(pages: Vec<Vec<i32>>, prio: HashMap<i32, Vec<i32>>) -> i32 {
    let mut total: i32 = 0;

    pages.iter().for_each(|page| {
        let mut is_valid = true;
        page.iter().enumerate().for_each(|(i, num)| {
            if !is_valid {
                return;
            }

            page.iter().skip(i + 1).for_each(|n| {
                if !is_valid {
                    return;
                }

                if let Some(entry) = prio.get(&n) {
                    is_valid = !find_in_vec(entry, *num);
                }
            });
        });

        if !is_valid {
            let mut sorted = page.clone();
            sorted.sort_by(|a, b| {
                if let Some(a_prio) = prio.get(a) {
                    if find_in_vec(a_prio, *b) {
                        return Ordering::Less;
                    }
                }

                if let Some(b_prio) = prio.get(b) {
                    if find_in_vec(b_prio, *a) {
                        return Ordering::Greater;
                    }
                }

                Ordering::Equal
            });
            total += sorted[sorted.len() / 2 as usize]
        }
    });

    total
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Please provide an argument for which part to run");
        return;
    }

    let (pages, prio) = parse_input();
    match args[1].as_str() {
        "1" => {
            println!("Part 1: {}", part1(pages, prio));
        }
        "2" => {
            println!("Part 2: {}", part2(pages, prio));
        }
        _ => println!("Invalid argument. Please use 1 or 2"),
    }
}
