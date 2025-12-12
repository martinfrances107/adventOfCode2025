use std::fmt::Display;

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
    pub rows: [[Block; 3]; 3],
}

impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let brick_char = self.id.to_string().chars().next().unwrap();
        for row in &self.rows {
            for block in row {
                match block {
                    Block::Blank => write!(f, ".")?,
                    Block::Brick => write!(f, "{brick_char}")?,
                }
            }
            writeln!(f)?
        }
        Ok(())
    }
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
            Self { id, rows: row }
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
                    rows: [
                        [Brick, Brick, Brick],
                        [Brick, Brick, Blank],
                        [Brick, Brick, Blank],
                    ],
                }
            ))
        );
    }

    #[test]
    fn display_shape() {
        let input = "1:
###
##.
##.
";
        let (_, s) = Shape::parse(input).unwrap();

        assert_eq!(
            s.to_string(),
            "111
11.
11.
"
        );
    }
}
