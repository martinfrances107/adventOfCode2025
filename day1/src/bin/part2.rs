use core::fmt;

use nom::IResult;
use nom::Parser;
use nom::character::complete::{digit1, one_of};
use nom::combinator::{map, map_res};
use nom::error::{Error, ErrorKind};

fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part2(input));
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

fn turn(dial: i64, dir: &Dir) -> (i64, i64) {
    let revolutions: i64;
    let new_dial: i64;
    match dir {
        Dir::L(value) => {
            new_dial = (dial - value).rem_euclid(100);
            if dial > 0 {
                revolutions = (dial - value).div_euclid(100).abs();
            } else {
                revolutions = (dial - value).div_euclid(100).abs() - 1;
            }
        }
        Dir::R(value) => {
            new_dial = (dial + value).rem_euclid(100);
            if new_dial > 0 {
                revolutions = (dial + value).div_euclid(100).abs();
            } else {
                revolutions = (dial + value).div_euclid(100).abs() - 1;
            }
        }
    }
    (new_dial, revolutions)
}

fn part2(input: &str) -> i64 {
    let mut dial = 50i64;
    let mut num_landings = 0;
    let mut num_crossings_total = 0;
    for line in input.lines() {
        let (_remain, dir) = parse_dir(line).unwrap();
        let (new_dial, revolutions) = turn(dial, &dir);
        num_crossings_total += revolutions;
        if new_dial == 0 {
            num_landings += 1;
        }
        dial = new_dial;
    }
    num_landings + num_crossings_total
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn example2() {
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

        assert_eq!(part2(input), 6);
    }

    #[test]
    fn test_turn() {
        let cases = [
            // dial_before, ACTION, dial_after, num_revolutions
            (50, Dir::L(68), 82, 1),
            (82, Dir::L(30), 52, 0),
            (52, Dir::R(48), 0, 0),
            (0, Dir::L(5), 95, 0),
            (95, Dir::R(60), 55, 1),
            (55, Dir::L(55), 0, 0),
            (0, Dir::L(1), 99, 0),
            (99, Dir::L(99), 0, 0),
            (0, Dir::R(14), 14, 0),
            (14, Dir::L(82), 32, 1),
        ];

        for case in cases {
            let (dial_after, revolutions) = turn(case.0, &case.1);
            println!(
                "dial before {} ...{} after  {}  n_revs {}",
                case.0, case.1, dial_after, revolutions
            );
            assert_eq!(dial_after, case.2, "dial  {dial_after} expected {}", case.2);
            assert_eq!(revolutions, case.3, "revolutions {revolutions} {}", case.3);
        }
    }
}
