fn parse_input() -> (Vec<String>, Vec<String>) {
    let mut input = include_str!("../input.txt").lines();
    let mut values_first: Vec<String> = Vec::new();
    let mut values_second: Vec<String> = Vec::new();
    while let Some(line) = input.next() {
        line.split_terminator("|").for_each(|s| {
            if s.len() < 50 {
                s.split_whitespace().for_each(|s| {
                    values_second.push(s.trim().to_string());
                });
            } else {
                s.split_whitespace().for_each(|s| {
                    values_first.push(s.trim().to_string());
                });
            }
        });
    }
    (values_first, values_second)
}

#[allow(dead_code)]
fn part1(input: Vec<String>) -> i32 {
    let mut count: i32 = 0;
    input.iter().for_each(|s| {
        let length = s.len();
        if length == 2 || length == 3 || length == 4 || length == 7 {
            count += 1;
        }
    });
    count
}

// functions for 2 part

fn multiply(i: usize, n: i32) -> i32 {
    match i {
        0 => n * 1000,
        1 => n * 100,
        2 => n * 10,
        3 => n,
        _ => 0,
    }
}

// this function checks if b is contained in a even if b doesnt have the same order
fn better_contains(a: String, b: String) -> bool {
    let result: bool = true;
    for c in b.chars() {
        if !a.contains(c.to_string().as_str()) {
            return false;
        }
    }
    result
}

fn check_5digit(input: String, one: String, four: String) -> i32 {
    if better_contains(input.clone(), one) {
        return 3;
    }
    let mut counter: i32 = 0;
    for c in four.chars() {
        if input.contains(c.to_string().as_str()) {
            counter += 1;
        }
    }
    if counter == 3 {
        return 5;
    }
    2
}

fn check_6digit(input: String, one: String, four: String) -> i32 {
    if !better_contains(input.clone(), one) {
        return 6;
    }
    if better_contains(input.clone(), four) {
        return 9;
    }
    0
}

fn part2(first: Vec<String>, second: Vec<String>) -> i32 {
    let mut sorted: Vec<Vec<String>> = Vec::new();
    // sorted each 10 words by length
    for seg in first.windows(10).step_by(10) {
        let mut temp: Vec<String> = seg.to_vec();
        temp.sort_by_key(|s| s.len());
        sorted.push(temp);
    }
    // loop over each 4 words in second and take the sorted vec at index to find the signals
    let mut index = 0;
    let mut count: i32 = 0;
    for seg in second.windows(4).step_by(4) {
        let one = sorted[index][0].clone();
        let four = sorted[index][2].clone();
        for (i, n) in seg.iter().enumerate() {
            match n.len() {
                2 => count += multiply(i, 1),
                3 => count += multiply(i, 7),
                4 => count += multiply(i, 4),
                5 => count += multiply(i, check_5digit(n.clone(), one.clone(), four.clone())),
                6 => count += multiply(i, check_6digit(n.clone(), one.clone(), four.clone())),
                7 => count += multiply(i, 8),
                _ => eprintln!("Error"),
            }
        }
        index += 1;
    }
    return count;
}

fn main() {
    let (first, second) = parse_input();
    // println!("Part 1 = {}\n", part1(second));
    println!("Part 2 = {}\n", part2(first, second));
}
