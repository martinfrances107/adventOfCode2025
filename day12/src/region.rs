use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::{char, digit1, line_ending},
    combinator::{map, map_res},
    error::{Error, ErrorKind},
    multi::many1,
    sequence::{terminated, tuple},
};

use crate::{parse_u8, parse_usize};

pub struct ListItem {
    quantity: u8,
    id: usize,
}
pub struct Region {
    width: u8,
    length: u8,
    list: Vec<ListItem>,
}

impl Region {
    fn parse_region(input: &str) -> IResult<&str, (u8, u8)> {
        map_res(
            tuple((digit1, char('x'), digit1)),
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
        )(input)
    }

    fn parse_list_item(input: &str) -> IResult<&str, ListItem> {
        map(
            tuple((parse_u8, char(' '), parse_usize)),
            |(quantity, _, id): (u8, char, usize)| ListItem { quantity, id },
        )(input)
    }
    pub fn parse(input: &str) -> IResult<&str, Self> {
        map(
            terminated(
                tuple((Self::parse_region, tag(": "), many1(Self::parse_list_item))),
                line_ending,
            ),
            |((width, length), _, list)| Self {
                width,
                length,
                list,
            },
        )(input)
    }
}
