use nom::{
    IResult,
    branch::alt,
    character::complete::char,
    combinator::map_res,
    error::{Error, ErrorKind},
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
        })(input)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn parse_block() {
        use super::Block::*;

        assert_eq!(Block::parse("# "), Ok((" ", Brick)));
        assert_eq!(Block::parse(". "), Ok((" ", Blank)));
        // assert_eq!(
        //     Block::parse("X "),
        //     Err(Error::new("X ", nom::error::ErrorKind::Char))
        // );
    }
}
