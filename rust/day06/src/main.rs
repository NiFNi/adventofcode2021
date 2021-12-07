use std::collections::HashMap;

mod commons;

fn main() {
    let input = commons::lines_from_file("input/input");

    let input_line = input.iter().next().unwrap();
    let input_fish: Vec<u8> = input_line
        .split(",")
        .map(|value| value.parse().unwrap())
        .collect();

    let result1 = input_fish
        .iter()
        .map(|fish| calculate_number_of_fish(*fish, 80))
        .reduce(|first, second| first + second);
    println!("{:?}", result1);

    let mut cache = HashMap::new();

    let result2 = input_fish
        .iter()
        .map(|fish| {
            *cache
                .entry(fish)
                .or_insert_with(|| calculate_number_of_fish(*fish, 256))
        })
        .reduce(|first, second| first + second);

    println!("{:?}", result2);
}

fn calculate_number_of_fish(input: u8, days: u16) -> u64 {
    let mut fishs: Vec<u8> = Vec::new();
    fishs.push(input);
    for _ in 1..days {
        let mut new_fish = 0;
        for i in 0..fishs.len() {
            let fish = fishs.get_mut(i).unwrap();
            if *fish == 1 {
                new_fish += 1;
            }
            if *fish == 0 {
                *fish = 6
            } else {
                *fish -= 1
            }
        }

        for _ in 0..new_fish {
            fishs.push(9)
        }
    }

    fishs.len() as u64
}
