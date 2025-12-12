//! Button Smasher
#![deny(clippy::all)]
#![warn(clippy::cargo)]
#![warn(clippy::complexity)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::perf)]
#![warn(missing_debug_implementations)]

use nom::{IResult, multi::many1, sequence::tuple};

use day12::region::Region;
use day12::shape::Shape;

fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part1(input));
}

fn parse_input(input: &str) -> IResult<&str, (Vec<Shape>, Vec<Region>)> {
    tuple((many1(Shape::parse), many1(Region::parse)))(input)
}

fn part1(input: &str) -> usize {
    let (_, (_shapes, _region)) = parse_input(input).expect("did not parse input");
    0_usize
}

// #[cfg(test)]
// mod test {

//     use super::*;

//     #[test]
//     fn parse_shape() {
//         use day12::block::Block::*;
//         let input = "0:
// ###
// ##.
// ##.";

//         assert_eq!(
//             Shape::parse(input),
//             Ok((
//                 "",
//                 Shape {
//                     id: 1u8,
//                     row: [
//                         [Brick, Brick, Brick],
//                         [Brick, Brick, Blank],
//                         [Brick, Brick, Blank],
//                     ],
//                 }
//             ))
//         );
//     }
//     #[test]
//     fn test_part1() {
//         let input = "0:
// ###
// ##.
// ##.

// 1:
// ###
// ##.
// .##

// 2:
// .##
// ###
// ##.

// 3:
// ##.
// ###
// ##.

// 4:
// ###
// #..
// ###

// 5:
// ###
// .#.
// ###

// 4x4: 0 0 0 0 2 0
// 12x5: 1 0 1 0 2 2
// 12x5: 1 0 1 0 3 2";

//         assert_eq!(part1(input), 5usize);
//     }
// }
