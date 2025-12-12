use std::fmt::Display;

use nom::{
    IResult, Parser,
    character::complete::{char, line_ending},
    combinator::map,
    multi::many1,
    sequence::terminated,
};

use crate::block::{Block, Blocks};

#[derive(Debug, Eq, PartialEq)]
pub struct Shape {
    pub id: usize,
    pub blocks: Blocks,
}

impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let brick_char = self.id.to_string().chars().next().unwrap();
        for row in &self.blocks.0 {
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
        terminated(super::parse_usize, (char(':'), line_ending)).parse(input)
    }

    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            terminated((Self::parse_id, Blocks::parse), line_ending),
            |(id, row)| Self { id, blocks: row },
        )
        .parse(input)
    }
    pub fn parse_many1(input: &str) -> IResult<&str, Vec<Self>> {
        many1(Self::parse).parse(input)
    }

    pub fn brick_count(&self) -> u32 {
        let mut count = 0;
        for row in &self.blocks.0 {
            for b in row {
                if *b == Block::Brick {
                    count += 1;
                }
            }
        }
        count
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
                    blocks: Blocks([
                        [Brick, Brick, Brick],
                        [Brick, Brick, Blank],
                        [Brick, Brick, Blank],
                    ]),
                }
            ))
        );
    }

    #[test]
    fn parse_two_shapes() {
        let input = "0:
###
##.
##.

1:
###
##.
.##

";

        assert_eq!(
            Shape::parse_many1(input),
            Ok((
                "",
                vec![
                    Shape {
                        id: 0usize,
                        blocks: Blocks([
                            [Brick, Brick, Brick],
                            [Brick, Brick, Blank],
                            [Brick, Brick, Blank],
                        ]),
                    },
                    Shape {
                        id: 1usize,
                        blocks: Blocks([
                            [Brick, Brick, Brick],
                            [Brick, Brick, Blank],
                            [Blank, Brick, Brick],
                        ]),
                    }
                ]
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
