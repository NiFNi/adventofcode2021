use std::slice::Iter;
use std::usize;

mod commons;

#[derive(Clone)]
struct Bingo {
    line1: Vec<BingoField>,
    line2: Vec<BingoField>,
    line3: Vec<BingoField>,
    line4: Vec<BingoField>,
    line5: Vec<BingoField>,
}

#[derive(Clone)]
struct BingoField {
    value: u32,
    checked: bool,
}

impl Bingo {
    fn has_bingo(&self) -> bool {
        let lines = self.get_lines();
        let possible_bingos = vec![
            self.line1.iter().collect(),
            self.line2.iter().collect(),
            self.line3.iter().collect(),
            self.line4.iter().collect(),
            self.line5.iter().collect(),
            get_index(&lines, 0),
            get_index(&lines, 1),
            get_index(&lines, 2),
            get_index(&lines, 3),
            get_index(&lines, 4),
        ];

        possible_bingos
            .iter()
            .fold(false, |current, list| current || all_checked(list))
    }

    fn get_lines(&self) -> Vec<Vec<&BingoField>> {
        vec![
            &self.line1,
            &self.line2,
            &self.line3,
            &self.line4,
            &self.line5,
        ]
        .iter()
        .map(|line| line.iter().collect())
        .collect()
    }

    fn on_new_number(&mut self, number: u32) {
        let lines = vec![
            &mut self.line1,
            &mut self.line2,
            &mut self.line3,
            &mut self.line4,
            &mut self.line5,
        ];
        for line in lines {
            for value in line {
                if value.value == number {
                    value.checked = true
                }
            }
        }
    }

    fn get_unmarked(&self) -> u32 {
        vec![
            &self.line1,
            &self.line2,
            &self.line3,
            &self.line4,
            &self.line5,
        ]
        .iter()
        .flat_map(|line| line.iter())
        .filter(|field| !field.checked)
        .fold(0, |current, field| {
            println!("{}", field.value);
            current + field.value
        })
    }
}

fn all_checked(fields: &Vec<&BingoField>) -> bool {
    fields
        .iter()
        .fold(true, |current, field| current && field.checked)
}

fn get_index<'a>(fields: &Vec<Vec<&'a BingoField>>, index: usize) -> Vec<&'a BingoField> {
    fields
        .iter()
        .map(|vector| *vector.get(index).unwrap())
        .collect()
}

fn main() {
    let input = commons::lines_from_file("input/inputDay4");

    let mut iter = input.iter();

    let numbers_strings = iter.next().unwrap();

    let mut numbers = numbers_strings.split(',');

    let mut bingos = parse_bingos(iter);
    let mut bingos2 = bingos.clone();

    let mut has_bingo = false;
    while !has_bingo {
        let number = numbers.next().unwrap();
        for bingo in &mut bingos {
            let parsed_number = number.parse().unwrap();
            bingo.on_new_number(parsed_number);
            if bingo.has_bingo() {
                println!("Part 1: {}", bingo.get_unmarked() * parsed_number);
                has_bingo = true;
            }
        }
    }

    let mut numbers2 = numbers_strings.split(',');
    let mut last_bingo = false;

    while !last_bingo {
        let number = numbers2.next().unwrap();
        let parsed_number = number.parse().unwrap();
        for bingo in &mut bingos2 {
            bingo.on_new_number(parsed_number);
        }
        if bingos2.len() == 1 && bingos2.get(0).unwrap().has_bingo() {
            println!("parsed number: {}", parsed_number);
            println!("unmarked: {}", bingos2.get(0).unwrap().get_unmarked());
            println!(
                "Part 2: {}",
                bingos2.get(0).unwrap().get_unmarked() * parsed_number
            );
            last_bingo = true;
        }
        bingos2.retain(|bingo| !bingo.has_bingo());
    }
}

fn parse_bingos(mut iter: Iter<String>) -> Vec<Bingo> {
    let mut bingos = Vec::new();
    while iter.next().is_some() {
        let mut lines = Vec::new();
        for _i in 0..5 {
            lines.push(iter.next().unwrap());
        }
        let splitted_lines: Vec<Vec<BingoField>> = lines
            .into_iter()
            .map(|line| {
                line.split_whitespace()
                    .into_iter()
                    .map(parse_int)
                    .map(new_field)
                    .collect()
            })
            .collect();
        let mut iter = splitted_lines.into_iter();
        bingos.push(Bingo {
            line1: iter.next().unwrap(),
            line2: iter.next().unwrap(),
            line3: iter.next().unwrap(),
            line4: iter.next().unwrap(),
            line5: iter.next().unwrap(),
        })
    }
    bingos
}

fn parse_int(input: &str) -> u32 {
    input.trim().parse().unwrap()
}

fn new_field(value: u32) -> BingoField {
    BingoField {
        value,
        checked: false,
    }
}
