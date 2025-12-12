use nom::{
    IResult,
    character::complete::char,
    character::complete::line_ending,
    combinator::map,
    sequence::{terminated, tuple},
};

use crate::block::Block;

#[derive(Debug, Eq, PartialEq)]
pub struct Shape {
    pub id: usize,
    pub row: [[Block; 3]; 3],
}

impl Shape {
    fn parse_id(input: &str) -> IResult<&str, usize> {
        terminated(super::parse_usize, tuple((char(':'), line_ending)))(input)
    }
    fn parse_row(input: &str) -> IResult<&str, [Block; 3]> {
        map(
            terminated(
                tuple((Block::parse, Block::parse, Block::parse)),
                line_ending,
            ),
            |(a, b, c): (Block, Block, Block)| {
                // a
                [a, b, c]
            },
        )(input)
    }
    fn parse_blocks(input: &str) -> IResult<&str, [[Block; 3]; 3]> {
        map(
            tuple((Shape::parse_row, Shape::parse_row, Shape::parse_row)),
            |(a, b, c)| [a, b, c],
        )(input)
    }

    pub fn parse(input: &str) -> IResult<&str, Self> {
        map(tuple((Self::parse_id, Self::parse_blocks)), |(id, row)| {
            Self { id, row }
        })(input)
    }
}

#[cfg(test)]
mod test {

    use super::Block::*;
    use super::*;
    #[test]
    fn parse_id() {
        let input = "1:
";
        assert_eq!(Shape::parse_id(input), Ok(("", 1usize)));
    }
    #[test]
    fn parse_row() {
        let input = "###
";
        assert_eq!(Shape::parse_row(input), Ok(("", [Brick, Brick, Brick])));
    }

    #[test]
    fn parse_blocks() {
        let input = "###
##.
##.
";

        let expected = [
            [Brick, Brick, Brick],
            [Brick, Brick, Blank],
            [Brick, Brick, Blank],
        ];
        assert_eq!(Shape::parse_blocks(input), Ok(("", expected)));
    }

    #[test]
    fn parse_shape() {
        let input = "0:
###
##.
##.
";

        assert_eq!(
            Shape::parse(input),
            Ok((
                "",
                Shape {
                    id: 0usize,
                    row: [
                        [Brick, Brick, Brick],
                        [Brick, Brick, Blank],
                        [Brick, Brick, Blank],
                    ],
                }
            ))
        );
    }
}
