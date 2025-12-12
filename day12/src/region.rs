use std::fmt::Display;

use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{char, digit1, line_ending},
    combinator::{map, map_res},
    error::{Error, ErrorKind},
    multi::{many1, separated_list1},
    sequence::terminated,
};

use crate::parse_u8;

#[derive(Debug, Eq, PartialEq)]
pub struct Region {
    pub width: u8,
    pub length: u8,
    pub list: Vec<u8>,
}

impl Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Region {}x{} - {:?}", self.width, self.length, self.list)
    }
}
impl Region {
    fn parse_header(input: &str) -> IResult<&str, (u8, u8)> {
        map_res(
            (digit1, char('x'), digit1),
            |(a_str, _, b_str): (&str, char, &str)| {
                // a
                if let Ok(a) = a_str.parse::<u8>() {
                    if let Ok(b) = b_str.parse::<u8>() {
                        Ok((a, b))
                    } else {
                        Err(Error::new("Fail to parse length ", ErrorKind::Alt))
                    }
                } else {
                    Err(Error::new("Fail to parse width ", ErrorKind::Alt))
                }
            },
        )
        .parse(input)
    }

    pub fn parse(input: &str) -> IResult<&str, Self> {
        map(
            terminated(
                (
                    Self::parse_header,
                    tag(": "),
                    separated_list1(char(' '), parse_u8),
                ),
                line_ending,
            ),
            |((width, length), _, list)| Self {
                width,
                length,
                list,
            },
        )
        .parse(input)
    }

    pub fn many1(input: &str) -> IResult<&str, Vec<Self>> {
        many1(Self::parse).parse(input)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_region() {
        let input = "1x2";

        let out = Region::parse_header(input);
        assert_eq!(out, Ok(("", (1u8, 2u8))));
    }

    #[test]
    fn single() {
        let input = "12x5: 1 0 1 0 2 2
";
        assert_eq!(
            Region::parse(input),
            Ok((
                "",
                Region {
                    width: 12,
                    length: 5,
                    list: vec![1, 0, 1, 0, 2, 2]
                }
            ))
        );
    }

    #[test]
    fn two() {
        let input = "4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
";

        assert_eq!(
            Region::many1(input),
            Ok((
                "",
                vec![
                    Region {
                        width: 4,
                        length: 4,
                        list: vec![0, 0, 0, 0, 2, 0]
                    },
                    Region {
                        width: 12,
                        length: 5,
                        list: vec![1, 0, 1, 0, 2, 2]
                    }
                ]
            ))
        );
    }
}
