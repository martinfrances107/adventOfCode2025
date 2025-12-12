//! Red Square
#![deny(clippy::all)]
#![warn(clippy::cargo)]
#![warn(clippy::complexity)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::perf)]
#![warn(missing_debug_implementations)]

use itertools::Itertools;

fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part1(input));
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Square {
    x: i64,
    y: i64,
}

impl Square {
    // length squared
    const fn area(&self, other: &Self) -> i64 {
        let x = (self.x - other.x).abs() + 1;
        let y = (self.y - other.y).abs() + 1;
        x * y
    }
}

fn part1(input: &str) -> i64 {
    let mut squares = vec![];
    for line in input.lines() {
        let mut splitter = line.split(',');
        let x = splitter.next().unwrap().parse().unwrap();
        let y = splitter.next().unwrap().parse().unwrap();
        squares.push(Square { x, y });
    }

    let mut area_max = 0;
    let si = squares.iter();
    for (_a, _b, area) in squares
        .iter()
        .cartesian_product(si)
        .map(|(a, b)| (a, b, a.area(b)))
        .sorted_by(|x, y| x.2.partial_cmp(&y.2).unwrap())
    {
        if area > area_max {
            area_max = area;
        }
    }

    area_max
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn area() {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

        assert_eq!(part1(input), 50);
    }
}
