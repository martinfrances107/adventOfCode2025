use std::fmt::Display;

use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    combinator::{map, map_res},
    error::{Error, ErrorKind},
    multi::many1,
    sequence::delimited,
};

#[derive(Debug, Eq, PartialEq, Default)]
pub enum LampState {
    #[default]
    Off,
    On,
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct Lamp(pub LampState);

#[derive(Debug, Eq, PartialEq)]
pub struct Lamps(pub Vec<Lamp>);

impl FromIterator<Lamp> for Lamps {
    fn from_iter<I: IntoIterator<Item = Lamp>>(iter: I) -> Self {
        let mut lamps = vec![];
        for l in iter {
            lamps.push(l);
        }
        Self(lamps)
    }
}

impl Lamp {
    pub const fn toggle(&mut self) {
        self.0 = match self.0 {
            LampState::Off => LampState::On,
            LampState::On => LampState::Off,
        };
    }
}

pub fn parse_lamp(input: &str) -> IResult<&str, Lamp> {
    map_res(alt((tag("#"), tag("."))), |c| {
        // a
        match c {
            "#" => Ok(Lamp(LampState::On)),
            "." => Ok(Lamp(LampState::Off)),
            _ => Err(Error::new("Bad Lamp ", ErrorKind::Not)),
        }
    })(input)
}

impl Display for LampState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::On => "#",
            Self::Off => ".",
        };
        write!(f, "{c}")
    }
}

impl Display for Lamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for Lamps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for lamp in &self.0 {
            write!(f, "{lamp}",)?;
        }
        Ok(())
    }
}

pub fn parse_lamps(input: &str) -> IResult<&str, Lamps> {
    map(many1(parse_lamp), Lamps)(input)
}

pub fn parse_indictor_pannel(input: &str) -> IResult<&str, Lamps> {
    delimited(tag("["), parse_lamps, tag("]"))(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lamps() {
        use LampState::*;
        let cases = vec![
            (
                "[.##.]",
                Lamps(vec![Lamp(Off), Lamp(On), Lamp(On), Lamp(Off)]),
            ),
            (
                "[...#.]",
                Lamps(vec![Lamp(Off), Lamp(Off), Lamp(Off), Lamp(On), Lamp(Off)]),
            ),
            (
                "[.###.]",
                Lamps(vec![Lamp(Off), Lamp(On), Lamp(On), Lamp(On), Lamp(Off)]),
            ),
        ];

        for (input, expected_lamps) in cases {
            assert_eq!(parse_indictor_pannel(input), Ok(("", expected_lamps)));
        }
    }
}
