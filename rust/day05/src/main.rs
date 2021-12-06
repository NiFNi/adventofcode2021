use std::collections::HashMap;

mod commons;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
struct Point {
    x: u32,
    y: u32,
}

struct Line {
    first: Point,
    second: Point,
}

impl Point {
    fn new(x: u32, y: u32) -> Point {
        Point {
            x,
            y,
        }
    }
}

impl Line {
    fn get_points(&self) -> Vec<Point> {
        let mut result = Vec::new();
        if self.first.x == self.second.x {
            if self.first.y < self.second.y {
                for y in self.first.y..self.second.y {
                    result.push(Point::new(self.first.x, y));
                };
            } else {
                for y in self.second.y..self.first.y {
                    result.push(Point::new(self.first.x, y));
                };
            }
        }

        if self.first.y == self.second.y {
            if self.first.x < self.second.x {
                for x in self.first.x..self.second.x {
                    result.push(Point::new(x, self.first.y));
                };
            } else {
                for x in self.second.x..self.first.x {
                    result.push(Point::new(x, self.first.y));
                };
            }
        }
        result
    }
}


fn main() {
    let input = commons::lines_from_file("input/inputDay5");

    let iter = input.iter();

    let mut pointcounter = HashMap::new();
    for line in iter {
        let mut points = line.split(" -> ");
        let first = parse_point(points.next().unwrap());
        let second = parse_point(points.next().unwrap());
        let line = Line { first, second };
        for point in line.get_points() {
            *pointcounter.entry(point).or_insert(0) += 1;
        }
    }
    println!("{}", pointcounter.iter().filter(|&(_k, v)| v > &1).count())
}

fn parse_point(value: &str) -> Point {
    let mut splitted = value.split(",");
    Point {
        x: splitted.next().unwrap().parse().unwrap(),
        y: splitted.next().unwrap().parse().unwrap(),
    }
}
