const DAYS_PART1: usize = 80;
const DAYS_PART2: usize = 256;

fn parse_input() -> Vec<i32> {
    let input = include_str!("../input.txt");
    input
        .split_terminator(",")
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

#[allow(dead_code)]
fn part1(input: Vec<i32>) -> usize {
    let mut fishes: Vec<i32> = vec![0; 9];
    // fish quantity in vector corresponds to fish number
    input.iter().for_each(|n| fishes[*n as usize] += 1);
    // loop through days
    for _day in 0..DAYS_PART1 {
        // get the count of fish 0
        let zero_count = fishes[0];
        // shift all fish to the left
        for i in 0..fishes.len() - 1 {
            fishes[i] = fishes[i + 1];
        }
        // update 8 and 6 with the count of fish 0
        fishes[8] = zero_count;
        fishes[6] += zero_count;
    }
    fishes.iter().sum::<i32>() as usize
}

// same solution as part 1 but with i64 instead of i32
// to avoid overflow
fn part2(input: Vec<i32>) -> usize {
    let mut fishes: Vec<i64> = vec![0; 9];
    // fish quantity in vector corresponds to fish number
    input.iter().for_each(|n| fishes[*n as usize] += 1);
    // loop through days
    for _day in 0..DAYS_PART2 {
        // get the count of fish 0
        let zero_count = fishes[0];
        // shift all fish to the left
        for i in 0..fishes.len() - 1 {
            fishes[i] = fishes[i + 1];
        }
        // update 8 and 6 with the count of fish 0
        fishes[8] = zero_count;
        fishes[6] += zero_count;
    }
    fishes.iter().sum::<i64>() as usize
}

fn main() {
    let input = parse_input();
    // println!("Part 1 = {}\n", part1(input));
    println!("Part 2 = {}\n", part2(input));
}
