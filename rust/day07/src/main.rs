#![feature(int_abs_diff)]

mod commons;

fn main() {
    let input = commons::lines_from_file("input/input");
    let input_line = input.iter().next().unwrap();
    println!("Part1 {}", calculate(input_line, calculate_fuel_1));
    println!("Part2 {}", calculate(input_line, calculate_fuel_2));
}

fn calculate(input: &str, fuel_calculator: fn(u32, &u32) -> u32) -> u32 {
    let split: Vec<u32> = input.split(",").map(|string| string.parse().unwrap()).collect();
    let max: u32 = *split.iter().max().unwrap();
    (0..max).fold(u32::MAX, |current, target| {
        let fuel_usage = split.iter().map(|position| fuel_calculator(target, position)).sum();
        if current > fuel_usage {
            fuel_usage
        } else {
            current
        }
    })
}

fn calculate_fuel_1(target: u32, position: &u32) -> u32 {
    position.abs_diff(target)
}

fn calculate_fuel_2(target: u32, position: &u32) -> u32 {
    let diff = position.abs_diff(target) as f64;
    (0.5 * diff * (diff + 1.0)) as u32
}

#[cfg(test)]
mod tests {
    use crate::{calculate, calculate_fuel_1, calculate_fuel_2};

    const SAMPLE_DATA: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_1() {
        assert_eq!(calculate(SAMPLE_DATA, calculate_fuel_1), 37);
    }

    #[test]
    fn test_2() {
        assert_eq!(calculate(SAMPLE_DATA, calculate_fuel_2), 168);
    }
}
