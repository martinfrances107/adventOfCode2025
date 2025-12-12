//! Button Smasher
#![deny(clippy::all)]
#![warn(clippy::cargo)]
#![warn(clippy::complexity)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::perf)]
#![warn(missing_debug_implementations)]

use nom::Parser;
use nom::{IResult, multi::many1};

use day12::region::Region;
use day12::shape::Shape;

fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part1(input));
}

fn parse_input(input: &str) -> IResult<&str, (Vec<Shape>, Vec<Region>)> {
    (Shape::parse_many1, many1(Region::parse)).parse(input)
}

fn part1(input: &str) -> (u32, u32) {
    let (_remains, (shapes, regions)) = parse_input(input).expect("did not parse input");

    // let mut possible_count = 0;
    let mut clb = 0u32;
    let mut cub = 0u32;

    for region in regions {
        // println!("region {region}");
        let availble_space = region.width as u32 * region.length as u32;
        let mut hull_count = 0;
        // println!("available space {availble_space}");
        let mut brick_count = 0;
        for (id, quantity) in region.list.iter().enumerate() {
            brick_count += shapes[id].brick_count() * *quantity as u32;
            hull_count += *quantity as u32 * 9;
        }

        // println!("brick count {brick_count}");
        // println!("hull count {hull_count}");
        // println!();

        if availble_space >= brick_count {
            // println!("max: its possible");
            clb += 1;
        }

        if availble_space >= hull_count {
            // println!("max: its possible");
            cub += 1;
        }
    }
    (clb, cub)
}

#[cfg(test)]
mod test {

    use super::*;
    #[ignore]
    #[test]
    fn test_part1() {
        let input = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
    ";

        assert_eq!(part1(input), (3, 1));
    }
}
