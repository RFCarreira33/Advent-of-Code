enum Operator {
    Sum,
    Product,
    Min,
    Max,
    Equal,
    LessThen,
    GreaterThen,
}

struct Operation {
    operator: Operator,
    packets: Vec<u64>,
}

impl Operation {
    fn calculate(&self) -> u64 {
        match self.operator {
            Operator::Sum => self.packets.iter().sum(),
            Operator::Product => self.packets.iter().product(),
            Operator::Min => *self.packets.iter().min().unwrap(),
            Operator::Max => *self.packets.iter().max().unwrap(),
            Operator::Equal => {
                let mut iter = self.packets.iter();
                let first = iter.next().unwrap();
                let second = iter.next().unwrap();
                if first == second {
                    1
                } else {
                    0
                }
            }
            Operator::LessThen => {
                let mut iter = self.packets.iter();
                let first = iter.next().unwrap();
                let second = iter.next().unwrap();
                if first < second {
                    1
                } else {
                    0
                }
            }
            Operator::GreaterThen => {
                let mut iter = self.packets.iter();
                let first = iter.next().unwrap();
                let second = iter.next().unwrap();
                if first > second {
                    1
                } else {
                    0
                }
            }
        }
    }
}

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

fn recursive_parse_1(input: &String, index: &mut usize) -> u32 {
    // get version and id
    let mut v_count = 0;
    v_count += u32::from_str_radix(&input[*index..*index + 3], 2).unwrap();
    let id = &input[*index + 3..*index + 6];
    *index += 6;
    // literal if id == 4 or 100 in binary
    if id == "100" {
        let mut end = false;
        while !end {
            if &input[*index..*index + 1] == "0" {
                end = true;
            }
            *index += 5;
        }
        // otherwise we have an operator
    } else {
        let id_type = &input[*index..*index + 1];
        *index += 1;
        // id type 0 means the number of the next 15 bits is the length of the sub packets
        if id_type == "0" {
            let len = u32::from_str_radix(&input[*index..*index + 15], 2).unwrap();
            *index += 15;
            let end = *index + (len as usize);
            while *index < end {
                v_count += recursive_parse_1(input, index);
            }
        // id type 1 means the number of the next 11 bits is the number of packets
        } else {
            let len = u32::from_str_radix(&input[*index..*index + 11], 2).unwrap();
            *index += 11;
            for _ in 0..len {
                v_count += recursive_parse_1(input, index);
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
        // any packet will have atleast 12 bits
        if input_len - index < 12 {
            break;
        }
        v_count += recursive_parse_1(&input, &mut index);
    }
    v_count
}

fn recursive_parse_2(input: &String, index: &mut usize) -> u64 {
    let id = &input[*index + 3..*index + 6];
    *index += 6;
    // literal if id == 4 or 100 in binary
    if id == "100" {
        let mut val = Vec::new();
        let mut end = false;
        while !end {
            if &input[*index..*index + 1] == "0" {
                end = true;
            }
            val.push(&input[*index + 1..*index + 5]);
            *index += 5;
        }
        u64::from_str_radix(&val.join(""), 2).unwrap_or(0)
        // otherwise we have an operator
    } else {
        // match the id to the operation
        let op = match id {
            "000" => Operator::Sum,
            "001" => Operator::Product,
            "010" => Operator::Min,
            "011" => Operator::Max,
            "101" => Operator::GreaterThen,
            "110" => Operator::LessThen,
            "111" => Operator::Equal,
            _ => panic!("Unknown operator"),
        };
        let mut operation = Operation {
            operator: op,
            packets: Vec::new(),
        };
        let id_type = &input[*index..*index + 1];
        *index += 1;
        // id type 0 means the number of the next 15 bits is the length of the sub packets
        if id_type == "0" {
            let len = u32::from_str_radix(&input[*index..*index + 15], 2).unwrap();
            *index += 15;
            let end = *index + (len as usize);
            while *index < end {
                operation.packets.push(recursive_parse_2(input, index));
            }
        // id type 1 means the number of the next 11 bits is the number of packets
        } else {
            let len = u32::from_str_radix(&input[*index..*index + 11], 2).unwrap();
            *index += 11;
            for _ in 0..len {
                operation.packets.push(recursive_parse_2(input, index));
            }
        }
        operation.calculate()
    }
}

fn part2(input: String) -> u64 {
    let mut index: usize = 0;
    let mut total = 0;
    let input_len = input.len() - 1;
    while index < input_len {
        // any packet will have atleast 12 bits
        if input_len - index < 12 {
            break;
        }
        total += recursive_parse_2(&input, &mut index);
    }
    total
}

fn main() {
    let bin_input = parse_input();
    // println!("Part 1 = {}", part1(bin_input));
    println!("Part 2 = {}", part2(bin_input));
}
