use std::collections::HashMap;

mod commons;

fn main() {
    let input = commons::lines_from_file("input/input");
    let input_str = input.iter().map(|it| it.as_str()).collect();
    println!("Part1 {}", part1(&input_str));
    println!("Part2 {}", part2(&input_str));
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

fn part1(input: &Vec<&str>) -> i32 {
    let map = create_map_of_input(input);

    find_low_points(&map)
        .iter()
        .map(|(_, &value)| value + 1)
        .sum()
}

fn find_low_points(map: &HashMap<Point, i32>) -> Vec<(&Point, &i32)> {
    map.iter()
        .filter(|(point, value)| {
            let x = point.x;
            let y = point.y;
            is_value_smaller(&map, value, x - 1, y)
                && is_value_smaller(&map, value, x + 1, y)
                && is_value_smaller(&map, value, x, y - 1)
                && is_value_smaller(&map, value, x, y + 1)
        })
        .collect()
}

fn create_map_of_input(input: &Vec<&str>) -> HashMap<Point, i32> {
    let mut map: HashMap<Point, i32> = HashMap::new();
    for (i, value) in input.iter().enumerate() {
        for (j, character) in value.chars().enumerate() {
            map.insert(
                Point::new(i as i32, j as i32),
                character.to_digit(10).unwrap() as i32,
            );
        }
    }
    map
}

fn part2(input: &Vec<&str>) -> i32 {
    let map: HashMap<Point, i32> = create_map_of_input(input);

    let low_points = find_low_points(&map);
    let mut basins: Vec<Vec<Point>> = low_points
        .iter()
        .map(|(point, value)| find_higher_neighbors(point, value, &map))
        .collect();
    basins.sort_by(|a, b| a.len().cmp(&b.len()));
    basins.reverse();
    basins
        .iter()
        .take(3)
        .fold(1, |current, basin| current * basin.len() as i32)
}

fn find_higher_neighbors(point: &Point, value: &i32, map: &HashMap<Point, i32>) -> Vec<Point> {
    let neighbors = get_neighbors(point);
    let mut points: Vec<Point> = neighbors
        .iter()
        .filter(|p| map.get(p).unwrap_or(&-1) > value)
        .flat_map(|p| find_higher_neighbors(p, map.get(p).unwrap(), map).into_iter())
        .map(|it| it.clone())
        .collect();
    points.push(point.clone());
    points
}

fn get_neighbors(point: &Point) -> Vec<Point> {
    vec![
        Point::new(point.x + 1, point.y),
        Point::new(point.x - 1, point.y),
        Point::new(point.x, point.y + 1),
        Point::new(point.x, point.y - 1),
    ]
}

fn is_value_smaller(map: &HashMap<Point, i32>, value: &i32, x: i32, y: i32) -> bool {
    map.get(&Point::new(x, y)).unwrap_or(&(i32::MAX)) > value
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const SAMPLE_DATA: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn test_1() {
        let input: Vec<&str> = SAMPLE_DATA.lines().collect();
        assert_eq!(part1(&input), 15);
    }

    #[test]
    fn test_2() {
        let input: Vec<&str> = SAMPLE_DATA.lines().collect();
        assert_eq!(part2(&input), 1134);
    }
}
