use std::usize;

#[derive(Debug)]
struct Number {
    start: i32,
    end: i32,
    line: i32,
    val: u32
}

const HELPER: [(i32, i32); 8] = [
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
    (1, 1),
    (-1, -1),
    (1, -1),
    (-1, 1),
];

fn parse_input() -> (Vec<Number>, Vec<(i32, i32)>, Vec<(i32, i32)>) {
    let mut validators_p1: Vec<(i32, i32)> = vec![];
    let mut validators_p2: Vec<(i32, i32)> = vec![];
    let mut numbers: Vec<Number> = vec![];
    std::fs::read_to_string("input.txt")
        .expect("Error reading file")
        .split_terminator("\n")
        .enumerate()
        .for_each(|(y, line)| {
            let mut current_num = 0;
            let mut current_num_len = 0;
            let str_size = line.len()-1;
            line.chars().take(str_size).enumerate().for_each(|(x,c)| {
                if c.is_numeric() {
                    current_num *= 10;
                    current_num += c.to_digit(10).unwrap();
                    current_num_len += 1;
                    return;
                }
                if c != '.' {
                    validators_p1.push((x as i32, y as i32));
                    if c == '*' {
                        validators_p2.push((x as i32, y as i32));
                    }
                }
                if current_num != 0 {
                    numbers.push(Number {
                        start:x as i32-current_num_len,
                        end:x as i32-1,
                        line: y as i32,
                        val:current_num
                    });
                }
                current_num = 0;
                current_num_len = 0;
            });
            if current_num != 0 {
                numbers.push(Number {
                    start: str_size as i32 - current_num_len,
                    end: str_size as i32 - 1,
                    line: y as i32,
                    val: current_num
              });
            }
        });
    (numbers, validators_p1, validators_p2)
}

#[allow(dead_code)]
fn part1(mut input: Vec<Number>, validators: Vec<(i32, i32)>) -> u32 {
    let mut total = 0;
    validators.iter().for_each(|validator| {
        HELPER.iter().for_each(|(hx, hy)| {
            let mut post_index:i32 = -1;
            for (index, number) in input.iter().enumerate() {
                let nx = validator.0 + hx;
                if nx >= number.start && nx <= number.end && validator.1 + hy == number.line {
                    total += number.val;
                    post_index = index as i32;
                }
            }
            if post_index != -1 {
                input.remove(post_index as usize);
            }
        })
    });
    total
}

fn part2(mut input: Vec<Number>, validators: Vec<(i32, i32)>) -> u32 {
    let mut total = 0;
    validators.iter().for_each(|validator| {
        let mut first_gear = 0;
        let mut second_gear = 0;
        HELPER.iter().for_each(|(hx, hy)| {
            let mut post_index:i32 = -1;
            if first_gear != 0 && second_gear != 0 {
                return;
            }
            input.iter().enumerate().for_each(|(i, number)| {
                let nx = validator.0 + hx;
                if nx >= number.start && nx <= number.end && validator.1 + hy == number.line {
                    post_index = i as i32;
                    if first_gear == 0 {
                        first_gear = number.val;
                        return;
                    }
                    if second_gear == 0 {
                        second_gear = number.val
                    }
                }
            });
            if post_index != -1 {
                input.remove(post_index as usize);
            }
        });
        total += first_gear * second_gear
    });
    total

}

fn main() {
    let (input, validators_p1, validators_p2) = parse_input();
    // println!("Part 1 = {}", part1(input, validators_p1));
    println!("Part 2 = {}", part2(input, validators_p2));
}


