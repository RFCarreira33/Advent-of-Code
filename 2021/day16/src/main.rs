fn parse_input() -> String {
    let input = include_str!("../input.txt").chars();
    let mut result = String::new();
    input.into_iter().for_each(|c| {
        let _ = match u32::from_str_radix(&c.to_string(), 16) {
            Ok(n) => result.push_str(&format!("{:0>4b}", n)),
            Err(_) => return,
        };
    });
    result
}

fn recursive_parse(input: &String, index: &mut usize) -> u32 {
    let mut v_count = 0;
    let v = u32::from_str_radix(&input[*index..*index + 3], 2).unwrap();
    let id = u32::from_str_radix(&input[*index + 3..*index + 6], 2).unwrap();
    v_count += v;
    *index += 6;
    if id == 4 {
        let mut val = Vec::new();
        let mut end = false;
        while !end {
            if &input[*index..*index + 1] == "0" {
                end = true;
            }
            val.push(u32::from_str_radix(&input[*index + 1..*index + 5], 2).unwrap());
            *index += 5;
        }
    } else {
        let id_type = &input[*index..*index + 1];
        *index += 1;
        if id_type == "0" {
            let len = u32::from_str_radix(&input[*index..*index + 15], 2).unwrap();
            *index += 15;
            let end = *index + (len as usize);
            while *index < end {
                v_count += recursive_parse(input, index);
            }
        } else {
            let len = u32::from_str_radix(&input[*index..*index + 11], 2).unwrap();
            *index += 11;
            for _ in 0..len {
                v_count += recursive_parse(input, index);
            }
        }
    }
    v_count
}

#[allow(dead_code)]
fn part1(input: String) -> u32 {
    let mut index: usize = 0;
    let mut v_count = 0;
    let input_len = input.len() - 1;
    while index < input_len {
        if input_len - index < 20 {
            break;
        }
        v_count += recursive_parse(&input, &mut index);
    }
    v_count
}

fn part2(map: Vec<Vec<i32>>) -> i32 {
    0
}

fn main() {
    let bin_input = parse_input();
    println!("Part 1 = {}", part1(bin_input));
    // println!("Part 2 = {}", part2(map));
}
