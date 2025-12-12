use std::fmt::Display;

use crate::lamps::{Lamp, Lamps, parse_indictor_pannel};
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, map_res, recognize},
    multi::{separated_list0, separated_list1},
    sequence::delimited,
};

#[derive(Debug)]
pub struct StateMachine {
    pub buttons: Vec<Vec<usize>>,
    // References for counter.
    pub joltage: Vec<usize>,
    pub counters: Vec<usize>,
}

impl StateMachine {
    // Press button specified by index.
    fn press(&mut self, button_idx: usize) {
        // println!("pressing button {button_idx} {:?}", self.buttons);
        // toggle the lamps
        for count_idx in &self.buttons[button_idx] {
            self.counters[*count_idx] += 1;
        }
    }

    /// Apply a sequence of buttons presses
    pub fn press_buttons(&mut self, buttons: &Vec<usize>) {
        for b in buttons {
            self.press(*b);
        }
    }

    /// Returns the internal state to all lights off.
    pub fn reset(&mut self) {
        let n_counters = self.joltage.len();
        self.counters = (0..n_counters).map(|_| 0usize).collect();
    }

    pub fn joltage_ok(&self) -> bool {
        self.counters == self.joltage
    }
    pub fn number_of_buttons(&self) -> usize {
        self.buttons.len()
    }
}

impl Display for StateMachine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for c in &self.counters {
            write!(f, "{} ", c)?;
        }
        write!(f, "]")
    }
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
    map_res(recognize(digit1), str::parse).parse(input)
}

fn parse_usize_list(input: &str) -> IResult<&str, Vec<usize>> {
    separated_list0(tag(","), parse_usize).parse(input)
}

fn parse_wiriing_pairs(input: &str) -> IResult<&str, Vec<usize>> {
    delimited(tag("("), parse_usize_list, tag(")")).parse(input)
}

pub fn parse_wiriing_diagram(input: &str) -> IResult<&str, Vec<Vec<usize>>> {
    separated_list1(tag(" "), parse_wiriing_pairs).parse(input)
}

pub fn parse_joltage(input: &str) -> IResult<&str, Vec<usize>> {
    delimited(tag("{"), parse_usize_list, tag("}")).parse(input)
}

pub fn parse_line(input: &str) -> IResult<&str, StateMachine> {
    map(
        (
            parse_indictor_pannel,
            tag(" "),
            parse_wiriing_diagram,
            tag(" "),
            parse_joltage,
        ),
        |(target_lamps, _blank, buttons, _blank2, joltage)| {
            let n_lamps = target_lamps.0.len();
            let mut state = Lamps(vec![]);
            for _ in 0..n_lamps {
                state.0.push(Lamp::default());
            }
            let n_jolts = joltage.len();
            let mut counters = vec![];
            for _ in 0..n_jolts {
                counters.push(0_usize);
            }
            StateMachine {
                buttons,
                counters,
                joltage,
            }
        },
    )
    .parse(input)
}

#[cfg(test)]
mod test {

    use crate::sm2::parse_line;
    use crate::sm2::parse_wiriing_diagram;

    use super::*;

    // Push Button - change to expected state.
    // #[test]
    // fn test_single_push() {
    //     use LampState::*;
    //     let input = "[#.....] (0,3,4) ";
    //     if let Ok((_remain, mut sm)) = parse_line(input) {
    //         sm.state = Lamps(vec![
    //             Lamp(On),
    //             Lamp(Off),
    //             Lamp(Off),
    //             Lamp(Off),
    //             Lamp(Off),
    //             Lamp(Off),
    //         ]);
    //         sm.press_buttons(&vec![0]);

    //         assert_eq!(sm.to_string(), "[...##.]")
    //     } else {
    //         panic!("failed to parse line");
    //     };
    // }
    #[test]
    fn test_push_count_machine_one() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        if let Ok((_, mut sm)) = parse_line(input) {
            assert!(!sm.joltage_ok());
            // push first three buttons
            sm.press_buttons(&vec![0, 1, 1, 1, 3, 3, 3, 4, 5, 5]);
            assert!(
                sm.joltage_ok(),
                "machine 1 - failed to set counter {:#?} to {:#?}",
                sm.counters,
                sm.joltage
            );
        } else {
            panic!("failed to parse line");
        };
    }

    #[test]
    fn test_push_count_machine_two() {
        let input = "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}";
        if let Ok((_, mut sm)) = parse_line(input) {
            assert!(!sm.joltage_ok());
            sm.press_buttons(&vec![0, 0, 1, 1, 1, 1, 1, 3, 3, 3, 3, 3]);
            assert!(
                sm.joltage_ok(),
                "machine 2 - failed to set counter {:#?} to {:#?}",
                sm.counters,
                sm.joltage
            );
        } else {
            panic!("failed to parse line");
        };
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
}
