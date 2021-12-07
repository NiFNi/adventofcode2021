mod commons;

fn main() {
    let input = commons::lines_from_file("input/input");
    let input_line = input.iter().next().unwrap();
    println!("Part1 {}", calculate(input_line, 80));
    println!("Part2 {}", calculate(input_line, 256));
}

fn calculate(input: &str, days: usize) -> usize {
    let mut age_counts: [usize; 9] = [0; 9];

    input
        .split(",")
        .map(|s| s.trim().parse().unwrap())
        .for_each(|age: usize| age_counts[age] += 1);

    for _ in 0..days {
        age_counts.rotate_left(1);
        age_counts[6] += age_counts[8];
    }

    return age_counts.iter().sum();
}

#[cfg(test)]
mod tests {
    use crate::calculate;

    const SAMPLE_DATA: &str = "3,4,3,1,2";

    #[test]
    fn test_1() {
        assert_eq!(calculate(SAMPLE_DATA, 18), 26);
        assert_eq!(calculate(SAMPLE_DATA, 80), 5934);
    }

    #[test]
    fn test_2() {
        assert_eq!(calculate(SAMPLE_DATA, 256), 26984457539);
    }
}
