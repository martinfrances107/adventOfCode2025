//! Red Square
#![deny(clippy::all)]
#![warn(clippy::cargo)]
#![warn(clippy::complexity)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::perf)]
#![warn(missing_debug_implementations)]

use std::{collections::HashMap, fmt::Display};

use itertools::Itertools;

fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part2(input));
}

#[derive(Eq, PartialEq)]
enum State {
    In,
    Out,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Tile {
    Red,
    Green,
    Blank,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Red => write!(f, "#"),
            Tile::Green => write!(f, "X"),
            Tile::Blank => write!(f, "."),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Point {
    x: u64,
    y: u64,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}
#[derive(Clone, PartialEq)]
struct Square {
    xmin: u64,
    xmax: u64,
    ymin: u64,
    ymax: u64,
}

impl Square {
    fn new(a: &Point, b: &Point) -> Self {
        Self {
            xmin: a.x.min(b.x),
            xmax: a.x.max(b.x),
            ymin: a.y.min(b.y),
            ymax: a.y.max(b.y),
        }
    }
    // length squared
    const fn area(&self) -> u64 {
        let x = self.xmax - self.xmin + 1;
        let y = self.ymax - self.ymin + 1;
        x * y
    }

    const fn is_inside(&self, p: &Point) -> bool {
        p.x >= self.xmin && p.x <= self.xmax && p.y >= self.ymin && p.y <= self.ymax
    }

    fn points(&self) -> Vec<Point> {
        let mut out = vec![];
        for x in self.xmin as u64..=self.xmax as u64 {
            for y in self.ymin as u64..=self.ymax as u64 {
                out.push(Point { x, y });
            }
        }
        out
    }
}

fn gen_points(input: &str) -> Vec<Point> {
    let mut points = vec![];
    for line in input.lines() {
        let mut splitter = line.split(',');
        let x = splitter.next().unwrap().parse().unwrap();
        let y = splitter.next().unwrap().parse().unwrap();
        points.push(Point { x, y });
    }
    points
}

#[derive(Default)]
struct Floor(HashMap<Point, Tile>);

impl From<&Vec<Point>> for Floor {
    fn from(points: &Vec<Point>) -> Floor {
        let mut boundary = Floor(HashMap::default());
        let mut iter = points.iter();
        let first = iter.next().unwrap();

        let mut a = first;
        for b in iter {
            println!("line {a}, {b}");
            boundary.mark_boundary(a, b);
            a = b;
        }

        // Close the loop.
        let last = points.iter().last().unwrap();
        boundary.mark_boundary(last, first);

        boundary
    }
}

impl Display for Floor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..=16 {
            for x in 0..=16 {
                let p = Point { x, y };
                let tile = self.get_tile(&p);
                write!(f, "{tile}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
impl Floor {
    // Red are the corner position.
    // Green are the edges.
    fn mark_boundary(&mut self, a: &Point, b: &Point) {
        self.0.insert(a.clone(), Tile::Red);
        self.0.insert(b.clone(), Tile::Red);
        // is horizontal
        if a.y == b.y {
            println!("h");
            let y = a.y;
            let x_min = a.x.min(b.x);
            let x_max = a.x.max(b.x);
            println!("min{x_min} max{x_max}");
            for x in x_min + 1..x_max {
                self.0.insert(Point { x, y }, Tile::Green);
            }
        } else {
            println!("vertical");
            debug_assert!(a.x == b.x);
            let x = a.x;
            let y_min = a.y.min(b.y);
            let y_max = a.y.max(b.y);
            println!("min{y_min} max{y_max}");
            for y in y_min + 1..y_max {
                self.0.insert(Point { x, y }, Tile::Green);
            }
        }
    }

    fn get_tile(&self, p: &Point) -> Tile {
        // Walk horizontally from left-hand side to point, counting the number of boundary crossing
        // if odd it is inside.

        let mut ants_shoes = Tile::Blank;
        let mut output_tile = Tile::Blank;
        for x in 0..=p.x {
            let tp = Point { x, y: p.y };
            output_tile = match self.0.get(&tp) {
                Some(Tile::Red) => Tile::Red,
                Some(Tile::Green) => {
                    // Green implies Transtion.
                    if ants_shoes == Tile::Blank {
                        ants_shoes = Tile::Green;
                    } else if ants_shoes == Tile::Green {
                        ants_shoes = Tile::Blank
                    }
                    Tile::Green
                }

                Some(Tile::Blank) => panic!("blanks are never inserted into the HM"),
                None => {
                    // What ever is on the ant's shoes.
                    ants_shoes
                    // Tile::Blank
                }
            };
        }
        output_tile
    }

    fn contains_square(&self, square: &Square) -> bool {
        // loop over every point in the square
        // true only if every point is inside.
        for p in &square.points() {
            todo!();
            // if !self.contains_point(p) {
            //     return false;
            // }
        }
        true
    }
}

fn part2(input: &str) -> u64 {
    let points = gen_points(input);
    let boundary: Floor = (&points).into();
    let mut max_area = 0_u64;
    let si = points.iter();

    for (a, b, area) in points
        .iter()
        .cartesian_product(si)
        .map(|(a, b)| (a, b, Square::new(a, b).area()))
        .sorted_by(|x, y| x.2.partial_cmp(&y.2).unwrap())
    {
        let square = Square::new(a, b);
        if boundary.contains_square(&square) && area > max_area {
            max_area = area;
        }
    }
    max_area
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn square() {
        let tl = Point { x: 0, y: 0 };
        let br = Point { x: 99, y: 99 };
        let sq = Square::new(&tl, &br);

        assert_eq!(sq.area(), 100 * 100);

        // Given the square a Coord.
        assert_eq!(sq.is_inside(&tl), true);
        assert_eq!(sq.is_inside(&br), true);
        assert_eq!(sq.is_inside(&Point { x: 500, y: 500 }), false);
    }

    #[ignore]
    #[test]
    fn test_boundary_contains() {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";
        let points = gen_points(input);
        let boundary: Floor = (&points).into();
        println!("{boundary}");
        // // outside the top left corner
        // let a = Coord { x: 0.0, y: 0.0 };
        // let b = Coord { x: 2., y: 8. };

        // let sq = Square::new(&a, &b);
        // assert!(!polygon_contains(&polygon, &sq));

        // first example inside
        // 7,3 and 11,1:
        let a = Point { x: 7, y: 3 };
        let b = Point { x: 11, y: 1 };

        let sq = Square::new(&a, &b);
        assert!(boundary.contains_square(&sq));
    }

    #[ignore]
    #[test]
    fn full() {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

        // let output = bb.to_string();
        // assert_eq!(output, output_expected);
        assert_eq!(part2(input), 50000000);
    }
}
