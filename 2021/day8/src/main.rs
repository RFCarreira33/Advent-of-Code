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

fn part2(first: Vec<String>, second: Vec<String>) -> i32 {
    return 0;
}

fn main() {
    let (mut first, second) = parse_input();
    println!("Part 1 = {}\n", part1(second));
    // println!("Part 2 = {}\n", part2(first, second));
}
