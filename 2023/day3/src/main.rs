use std::usize;

#[derive(Debug, Clone)]
struct Number {
    start: i32,
    end: i32,
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

fn parse_input() -> (Vec<Vec<Number>>, Vec<(i32, i32)>, Vec<(i32, i32)>) {
    let mut validators_p1: Vec<(i32, i32)> = vec![];
    let mut validators_p2: Vec<(i32, i32)> = vec![];
    let mut numbers: Vec<Vec<Number>> = vec![vec![]; 140];
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
                    numbers[y].push(Number {
                        start:x as i32-current_num_len,
                        end:x as i32-1,
                        val:current_num
                    });
                }
                current_num = 0;
                current_num_len = 0;
            });
            if current_num != 0 {
                numbers[y].push(Number {
                    start: str_size as i32 - current_num_len,
                    end: str_size as i32 - 1,
                    val: current_num
              });
            }
        });
    (numbers, validators_p1, validators_p2)
}

#[allow(dead_code)]
fn part1(mut input: Vec<Vec<Number>>, validators: Vec<(i32, i32)>) -> u32 {
    let mut total = 0;
    validators.iter().for_each(|validator| {
        HELPER.iter().for_each(|(hx, hy)| {
            let nx = validator.0 + hx;
            let ny = validator.1 + hy;
            if ny < 0 || ny >= input.len() as i32 {
                return;
            }
            let mut post_index:i32 = -1;
            input[ny as usize].iter().enumerate().for_each(|(i, number)| {
                if nx >= number.start && nx <= number.end  {
                    total += number.val;
                    post_index = i as i32;
                }
            });
            if post_index != -1 {
                input[ny as usize].remove(post_index as usize);
            }
        })
    });
    total
}

fn part2(mut input: Vec<Vec<Number>>, validators: Vec<(i32, i32)>) -> u32 {
    let mut total = 0;
    validators.iter().for_each(|validator| {
        let mut first_gear = 0;
        let mut second_gear = 0;
        HELPER.iter().for_each(|(hx, hy)| {
            if first_gear != 0 && second_gear != 0 {
                return;
            }
            let nx = validator.0 + hx;
            let ny = validator.1 + hy;
            if ny < 0 || ny >= input.len() as i32 {
                return;
            }
            let mut post_index:i32 = -1;
            input[ny as usize].iter().enumerate().for_each(|(i, number)| {
                if nx >= number.start && nx <= number.end  {
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
                input[ny as usize].remove(post_index as usize);
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


