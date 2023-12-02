fn parse_input() -> Vec<String> {
    std::fs::read_to_string("input.txt")
        .expect("Error reading file")
        .split_terminator("\n")
        .map(|line| line.to_string())
        .collect()
}

#[allow(dead_code)]
fn part1(input: Vec<String>) -> u32 {
    let mut total: u32 = 0;
    input.iter().for_each(|line| {
        let mut f_char: char = '\0';
        let mut s_char: char = '\0';
        line.chars().zip(line.chars().rev()).for_each(|(f, s)| {
            if f_char != '\0' && s_char != '\0' {
                return;
            }
            if f.is_numeric() && f_char == '\0' {
                f_char = f;
            }
            if s.is_numeric() && s_char == '\0' {
                s_char = s;
            }
        });
        total += f_char.to_digit(10).unwrap()*10 + s_char.to_digit(10).unwrap();
    });
    total
}

fn part2(input: Vec<String>) -> u32 {
    let mut total: u32 = 0;
    input.iter().for_each(|line| {
        let mut f_char: char = '\0';
        let mut s_char: char = '\0';
        let line = convert_word_to_char(line);
        line.chars().zip(line.chars().rev()).for_each(|(f, s)| {
            if f_char != '\0' && s_char != '\0' {
                return;
            }
            if f.is_numeric() && f_char == '\0' {
                f_char = f;
            }
            if s.is_numeric() && s_char == '\0' {
                s_char = s;
            }
        });
        total += f_char.to_digit(10).unwrap()*10 + s_char.to_digit(10).unwrap();
    });
    total
}

// shenanigans
fn convert_word_to_char(line: &String) -> String {
    line.replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e")
}

fn main() {
    let input: Vec<String> = parse_input();
    // println!("Part 1 = {}", part1(input));
    println!("Part 2 = {}", part2(input));
}
