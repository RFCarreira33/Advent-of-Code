fn parse_input() -> Vec<Vec<i32>> {
    let mut vec1: Vec<Vec<i32>> = Vec::new();

    std::fs::read_to_string("input.txt")
        .expect("Error reading file")
        .split_terminator("\n")
        .for_each(|line| {
            vec1.push(
                line.split_whitespace()
                    .map(|num| num.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>(),
            );
        });

    vec1
}

fn meet_req(nums: &Vec<i32>) -> bool {
    let is_increment = nums[0] < nums[1];

    for n in nums.windows(2) {
        let mut dif = n[0] - n[1];

        if is_increment && dif > 0 || !is_increment && dif < 0 {
            return false;
        } 

        dif = dif.abs();

        if dif > 3 || dif < 1 {
            return false;
        }
    }

    return true;
}

fn part1(input: Vec<Vec<i32>>) -> i32 {
    let mut total: i32 = 0;

    input.iter().for_each(|row| {
        if meet_req(row) {
            total += 1;
        }
    });

    total
}

fn part2(input: Vec<Vec<i32>>) -> i32 {
    let mut total: i32 = 0;

    input.iter().for_each(|row| {
        for i in 0..row.len() {
            let mut row_clone = row.clone();
            row_clone.remove(i);

            if meet_req(&row_clone) {
                total += 1;
                break;
            }
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

    let input = parse_input();
    match args[1].as_str() {
        "1" => {
            println!("Part 1: {}", part1(input));
        }
        "2" => {
            println!("Part 2: {}", part2(input));
        }
        _ => println!("Invalid argument. Please use 1 or 2"),
    }
}
