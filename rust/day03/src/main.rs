use std::usize;

mod commons;

fn main() {
    println!("Hello, world!");
    let input = commons::lines_from_file("input/inputDay3");
    let input_as_str = input.iter().map(|s| s.as_str()).collect();
    let result: String = (0..12).into_iter().map(|i| {
        get_most_existing_char(&input_as_str, i)
    }).collect();
    println!("{}", result);
    let int_val: u128 = parse_binary(result.as_str());
    let inverted: String = invert_binary_string(result.as_str());
    println!("{}", inverted);
    let int_val_2 = parse_binary(inverted.as_str());
    println!("{}", int_val);
    println!("{}", int_val_2);
    println!("Part 1: {}", int_val * int_val_2);

    let part2_1 = parse_binary(part2(&input_as_str, false));
    let part2_2 = parse_binary(part2(&input_as_str, true));
    println!("{}", part2_1);
    println!("{}", part2_2);
    println!("{}", part2_1 * part2_2);
}

fn part2<'a>(input: &Vec<&'a str>, should_invert_char: bool) -> &'a str {
    let mut valid: Vec<&str> = input.clone();
    let mut counter = 0;
    while valid.len() > 1 {
        let mut char_to_compare_to = get_most_existing_char(&valid, counter);
        if should_invert_char {
            char_to_compare_to = invert_char(char_to_compare_to)
        }
        valid = valid.into_iter().filter(|line| line.chars().nth(counter).expect("out of bounds") == char_to_compare_to).collect();
        counter = counter + 1;
    };
    valid.iter().next().expect("nothing found")
}

fn invert_binary_string(result: &str) -> String {
    result.chars().map(invert_char).collect()
}

fn invert_char(c: char) -> char {
    if c == '1' { '0' } else { '1' }
}

fn parse_binary(result: &str) -> u128 {
    u128::from_str_radix(&result, 2).expect("cannot parse binary")
}

fn get_most_existing_char(input: &Vec<&str>, i: usize) -> char {
    let chars: Vec<char> = get_chars_at_index(input, i);
    let ones = chars.iter().filter(|char| **char == '1').count();
    let zeroes = chars.len() - ones;
    if ones >= zeroes { '1' } else { '0' }
}

fn get_chars_at_index(input: &Vec<&str>, i: usize) -> Vec<char> {
    input.iter().map(|line| line.chars().nth(i).expect("line to short").clone()).collect()
}
