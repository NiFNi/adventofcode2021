use std::collections::HashMap;

mod commons;

fn main() {
    let input = commons::lines_from_file("input/input");
    let input_str = input.iter().map(|it| it.as_str()).collect();
    println!("Part1 {}", part1(&input_str));
    println!("Part2 {}", part2(&input_str));
}

fn part1(input: &Vec<&str>) -> usize {
    let allowed_length = vec![2, 4, 3, 7];
    input
        .into_iter()
        .map(|it| it.split(" | ").nth(1).unwrap())
        .map(|it| {
            it.split_whitespace()
                .filter(|digits| allowed_length.contains(&digits.len()))
                .count()
        })
        .sum()
}

fn part2(input: &Vec<&str>) -> u32 {
    input.iter().map(|value| calculate_value(value)).sum()
}
fn calculate_value(line: &str) -> u32 {
    let mut splitted = line.split(" | ");
    let first: Vec<&str> = splitted.next().unwrap().split_whitespace().collect();
    let second = splitted.next().unwrap();

    let mut mapping = LineMapping::new();

    mapping.create_mapping(first);

    second
        .split_whitespace()
        .map(|number| mapping.get(number))
        .collect::<String>()
        .parse()
        .unwrap()
}

struct LineMapping {
    map: HashMap<String, char>,
    char_map: HashMap<char, char>,
}

impl LineMapping {
    fn new() -> LineMapping {
        LineMapping {
            map: HashMap::new(),
            char_map: HashMap::new(),
        }
    }

    fn create_mapping(&mut self, input: Vec<&str>) {
        self.find_by_length(&input, '1', 2);
        self.find_by_length(&input, '4', 4);
        self.find_by_length(&input, '7', 3);
        self.find_by_length(&input, '8', 7);

        // find a
        let one = find_key_for_value(&self.map, '1');
        let seven = find_key_for_value(&self.map, '7');
        let a = seven
            .chars()
            .find(|&char| !one.chars().any(|char2| char2 == char))
            .unwrap();
        self.char_map.insert('a', a);

        self.find_0_and_d_and_b(&input);
        self.find_2_and_c_and_f(&input);
    }

    fn get(&self, number: &str) -> char {
        *self.map.get(sort_string(number).as_str()).unwrap()
    }

    fn find_by_length(&mut self, input: &Vec<&str>, char: char, length: usize) {
        self.map.insert(
            sort_string(input.iter().find(|value| value.len() == length).unwrap()),
            char,
        );
    }

    fn find_0_and_d_and_b(&mut self, input: &Vec<&str>) {
        let one: Vec<char> = find_key_for_value(&self.map, '1').chars().collect();
        let four: Vec<char> = find_key_for_value(&self.map, '4').chars().collect();
        let only_four: Vec<&char> = four.iter().filter(|char| !one.contains(char)).collect();
        let mut d = None;
        self.map.insert(
            sort_string(
                input
                    .iter()
                    .filter(|value| value.len() == 6)
                    .find(|value| {
                        d = Some(
                            "abcdefg"
                                .chars()
                                .find(|&char| !value.chars().any(|a| a == char))
                                .unwrap(),
                        );
                        only_four.iter().any(|&&char| char == d.unwrap())
                    })
                    .unwrap(),
            ),
            '0',
        );
        self.char_map.insert('d', d.unwrap());
        self.char_map.insert(
            'b',
            **only_four.iter().find(|&&&char| char != d.unwrap()).unwrap(),
        );
    }

    fn find_2_and_c_and_f(&mut self, input: &Vec<&str>) {
        let mut c: Option<char> = None;
        let mut f: Option<char> = None;
        let two = input
            .iter()
            .filter(|value| value.len() == 5)
            .filter(|value| {
                let chars: Vec<char> = value.chars().collect();
                chars
                    .iter()
                    .any(|char| char == self.char_map.get(&'a').unwrap())
                    && chars
                        .iter()
                        .any(|char| char == self.char_map.get(&'d').unwrap())
                    && !chars
                        .iter()
                        .any(|char| char == self.char_map.get(&'b').unwrap())
            })
            .find(|value| {
                let mut seven = find_key_for_value(&self.map, '7').clone();
                seven.retain(|char| char != *self.char_map.get(&'a').unwrap());
                let seven_value: Vec<char> = seven
                    .chars()
                    .filter(|&char| value.chars().any(|a| a == char))
                    .collect();
                if seven_value.len() == 1 {
                    c = Some(*seven_value.get(0).unwrap());
                    f = seven.chars().find(|&char| char != c.unwrap());
                    true
                } else {
                    false
                }
            })
            .unwrap();
        self.map.insert(sort_string(two), '2');
        self.char_map.insert('c', c.unwrap());
        self.char_map.insert('f', f.unwrap());
    }
}

fn sort_string(to_sort: &str) -> String {
    let mut chars: Vec<char> = to_sort.chars().collect();
    chars.sort_by(char::cmp);
    chars.iter().collect()
}

fn find_key_for_value<'a>(map: &'a HashMap<String, char>, value: char) -> &'a String {
    map.iter()
        .find_map(|(key, &val)| if val == value { Some(key) } else { None })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const SAMPLE_DATA: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn test_1() {
        assert_eq!(part1(SAMPLE_DATA.lines().collect()), 26);
    }

    #[test]
    fn test_2() {
        assert_eq!(part2(SAMPLE_DATA.lines().collect()), 61229);
    }
}
