fn parse_input() -> Vec<Vec<i32>> {
    let mut history: Vec<Vec<i32>> = Vec::new();
    std::fs::read_to_string("input.txt")
        .expect("Error reading file")
        .split_terminator("\n")
        .for_each(|line| {
            history.push(line.split_whitespace().map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>());
        });
    history
}

fn find_dif(input: &Vec<i32>, reverse:bool) -> i32 {
    if input.iter().all(|n| *n == 0) {
        return 0;
    }
    let dif = find_dif(&input.windows(2).map(|window| window[1] - window[0]).collect::<Vec<i32>>(), reverse);
    if reverse {
        return input[0] - dif;
    }
    input[input.len() - 1] + dif
}


#[allow(dead_code)]
fn part1(input: Vec<Vec<i32>>) -> i32 {
    let mut total = 0;
    input.iter().for_each(|sequence| {
        total += find_dif(sequence, false);
    });
    total
}

fn part2(input : Vec<Vec<i32>>) -> i32 {
    let mut total = 0;
    input.iter().for_each(|sequence| {
        total += find_dif(sequence, true);
    });
    total
}

fn main() {
    let input = parse_input();
    // println!("Part 1 = {}", part1(input));
    println!("Part 2 = {}", part2(input));
}



