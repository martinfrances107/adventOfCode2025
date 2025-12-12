use nom::{IResult, character::complete::digit1, combinator::map_res};

pub mod block;
pub mod region;
pub mod shape;

fn parse_u8(input: &str) -> IResult<&str, u8> {
    map_res(digit1, |s: &str| s.parse::<u8>())(input)
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>())(input)
}
