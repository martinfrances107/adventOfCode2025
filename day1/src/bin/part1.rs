use core::fmt;

use nom::IResult;
use nom::Parser;
use nom::character::complete::{digit1, one_of};
use nom::combinator::{map, map_res};
use nom::error::{Error, ErrorKind};

fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part1(input));
}

#[derive(Debug)]
enum DirHeader {
    L,
    R,
}

#[derive(Debug)]
enum Dir {
    L(i64),
    R(i64),
}

impl fmt::Display for Dir {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Dir::L(value) => {
                write!(f, "L{value}")
            }
            Dir::R(value) => {
                write!(f, "R{value}")
            }
        }
    }
}

fn parse_dir_header(input: &str) -> IResult<&str, DirHeader> {
    map_res(one_of("LR"), |c| match c {
        'L' => Ok(DirHeader::L),
        'R' => Ok(DirHeader::R),
        _ => Err(Error::new("Failed to find Diretion", ErrorKind::Alt)),
    })
    .parse(input)
}

fn parse_value(input: &str) -> IResult<&str, i64> {
    map_res(digit1, str::parse).parse(input)
}

fn parse_dir(input: &str) -> IResult<&str, Dir> {
    map(
        (parse_dir_header, parse_value),
        |(header, value)| match header {
            DirHeader::L => Dir::L(value),
            DirHeader::R => Dir::R(value),
        },
    )
    .parse(input)
}

fn part1(input: &str) -> u64 {
    let mut dial = 50i64;
    let mut num_zeros = 0;
    for line in input.lines() {
        let (_remain, dir) = parse_dir(line).unwrap();
        match dir {
            Dir::L(value) => {
                dial = (dial - value).rem_euclid(100);
            }
            Dir::R(value) => {
                dial = (dial + value).rem_euclid(100);
            }
        }
        // println!("This dial is rotated {dir} to point at {dial}");
        if dial == 0 {
            num_zeros += 1;
        }
    }
    num_zeros
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn example() {
        let input = r"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

        assert_eq!(part1(input), 3);
    }
}
