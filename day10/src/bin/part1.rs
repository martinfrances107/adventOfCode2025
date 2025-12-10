//! Button Smasher
#![deny(clippy::all)]
#![warn(clippy::cargo)]
#![warn(clippy::complexity)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::perf)]
#![warn(missing_debug_implementations)]

use std::fmt::Display;

use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, map_res, recognize},
    error::{Error, ErrorKind},
    multi::{many1, separated_list0, separated_list1},
    sequence::{delimited, tuple},
};

fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part1(input));
}

#[derive(Debug, Eq, PartialEq, Default)]
enum LampState {
    #[default]
    Off,
    On,
}

#[derive(Debug, Default, Eq, PartialEq)]
struct Lamp(LampState);

#[derive(Debug, Eq, PartialEq)]
struct Lamps(Vec<Lamp>);

impl FromIterator<Lamp> for Lamps {
    fn from_iter<I: IntoIterator<Item = Lamp>>(iter: I) -> Self {
        let mut lamps = vec![];
        for l in iter {
            lamps.push(l);
        }
        Self(lamps)
    }
}

struct StateMachine {
    ready_state: Lamps,
    state: Lamps,
    buttons: Vec<Vec<usize>>,
}

impl StateMachine {
    // Press button specified by index.
    fn press(&mut self, button_idx: usize) {
        // toggle the lamps
        for lamp_idx in &self.buttons[button_idx] {
            self.state.0[*lamp_idx].toggle();
        }
    }

    fn press_buttons<I>(&mut self, b_iter: I)
    where
        I: IntoIterator<Item = usize>,
    {
        for b in b_iter {
            self.press(b);
        }
    }

    fn reset(&mut self) {
        let n_lamps = self.ready_state.0.len();
        self.state = (0..n_lamps).map(|_| Lamp::default()).collect();
    }

    fn is_ready(&self) -> bool {
        self.state == self.ready_state
    }
}

impl Display for StateMachine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.state)
    }
}
impl Lamp {
    const fn toggle(&mut self) {
        self.0 = match self.0 {
            LampState::Off => LampState::On,
            LampState::On => LampState::Off,
        };
    }
}

fn parse_lamp(input: &str) -> IResult<&str, Lamp> {
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

fn parse_lamps(input: &str) -> IResult<&str, Lamps> {
    map(many1(parse_lamp), Lamps)(input)
}
fn parse_indictor_pannel(input: &str) -> IResult<&str, Lamps> {
    delimited(tag("["), parse_lamps, tag("]"))(input)
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
    map_res(recognize(digit1), str::parse)(input)
}

fn parse_usize_list(input: &str) -> IResult<&str, Vec<usize>> {
    separated_list0(tag(","), parse_usize)(input)
}

fn parse_wiriing_pairs(input: &str) -> IResult<&str, Vec<usize>> {
    delimited(tag("("), parse_usize_list, tag(")"))(input)
}

fn parse_wiriing_diagram(input: &str) -> IResult<&str, Vec<Vec<usize>>> {
    separated_list1(tag(" "), parse_wiriing_pairs)(input)
}

fn parse_line(input: &str) -> IResult<&str, StateMachine> {
    map(
        tuple((
            parse_indictor_pannel,
            tag(" "),
            parse_wiriing_diagram,
            tag(" "),
        )),
        |(target_lamps, _blank, buttons, _blank2)| {
            let n_lamps = target_lamps.0.len();
            let mut state = Lamps(vec![]);
            for _ in 0..n_lamps {
                state.0.push(Lamp::default());
            }
            StateMachine {
                ready_state: target_lamps,
                state,
                buttons,
            }
        },
    )(input)
}

fn part1(_input: &str) -> i64 {
    todo!()
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
    #[test]
    fn test_wiring() {
        let cases: Vec<(&str, Vec<Vec<usize>>)> = vec![(
            "(3) (1,3) (2) (2,3) (0,2) (0,1)",
            vec![
                vec![3],
                vec![1, 3],
                vec![2],
                vec![2, 3],
                vec![0, 2],
                vec![0, 1],
            ],
            // IF needed day 10 has many more examples
        )];

        for (input, expected_wiring) in cases {
            assert_eq!(parse_wiriing_diagram(input), Ok(("", expected_wiring)));
        }
    }

    // Push Button - change to expected state.
    #[test]
    fn test_single_push() {
        use LampState::*;
        let input = "[#.....] (0,3,4) ";
        if let Ok((_remain, mut sm)) = parse_line(input) {
            sm.state = Lamps(vec![
                Lamp(On),
                Lamp(Off),
                Lamp(Off),
                Lamp(Off),
                Lamp(Off),
                Lamp(Off),
            ]);
            sm.press(0);

            assert_eq!(sm.to_string(), "[...##.]")
        } else {
            panic!("failed to parse line");
        };
    }
    #[test]
    fn test_push_count() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        if let Ok((_, mut sm)) = parse_line(input) {
            assert!(!sm.is_ready());
            // push first three buttons
            sm.press_buttons(vec![0, 1, 2]);
            assert!(sm.is_ready());
            sm.reset();
            // push (1,3) once (2,3) once, (0.1) twice
            sm.press_buttons(vec![1, 3, 5, 5]);
            assert!(sm.is_ready());
            sm.reset();
            // push all but (1,3) once
            sm.press_buttons(vec![0, 2, 3, 4, 5]);
            assert!(sm.is_ready());
        } else {
            panic!("failed to parse line");
        };
    }
}
