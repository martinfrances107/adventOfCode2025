use std::fmt::Display;

use nom::{
    IResult, Parser,
    branch::alt,
    character::complete::{char, line_ending},
    combinator::{map, map_res},
    error::{Error, ErrorKind},
    sequence::terminated,
};

#[derive(Debug, Eq, PartialEq)]
pub enum Block {
    Brick,
    Blank,
}

impl Block {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        // todo
        map_res(alt((char('#'), char('.'))), |c| {
            match c {
                // '#' => Block.brick = c,
                '#' => Ok(Block::Brick),
                '.' => Ok(Block::Blank),
                _ => Err(Error::new("must see a brick or a blank", ErrorKind::Alt)),
            }
        })
        .parse(input)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Blocks(pub [[Block; 3]; 3]);

impl Blocks {
    fn parse_row(input: &str) -> IResult<&str, [Block; 3]> {
        map(
            terminated((Block::parse, Block::parse, Block::parse), line_ending),
            |(a, b, c): (Block, Block, Block)| [a, b, c],
        )
        .parse(input)
    }

    pub fn parse(input: &str) -> IResult<&str, Blocks> {
        map(
            (Self::parse_row, Self::parse_row, Self::parse_row),
            |(a, b, c)| Blocks([a, b, c]),
        )
        .parse(input)
    }
}
impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Block::Brick => write!(f, "#"),
            Block::Blank => write!(f, "."),
        }
    }
}

#[cfg(test)]
mod test {

    use super::Block::*;
    use super::*;

    #[test]
    fn parse_row() {
        let input = "###
";
        assert_eq!(Blocks::parse_row(input), Ok(("", [Brick, Brick, Brick])));
    }

    #[test]
    fn parse_block() {
        assert_eq!(Block::parse("# "), Ok((" ", Brick)));
        assert_eq!(Block::parse(". "), Ok((" ", Blank)));
    }

    #[test]
    fn parse_blocks() {
        let input = "###
##.
##.
";

        let expected = Blocks([
            [Brick, Brick, Brick],
            [Brick, Brick, Blank],
            [Brick, Brick, Blank],
        ]);
        assert_eq!(Blocks::parse(input), Ok(("", expected)));
    }
}
