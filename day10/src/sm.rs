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
    pub ready_state: Lamps,
    pub state: Lamps,
    pub buttons: Vec<Vec<usize>>,
}

impl StateMachine {
    // Press button specified by index.
    fn press(&mut self, button_idx: usize) {
        // println!("pressing button {button_idx} {:?}", self.buttons);
        // toggle the lamps
        for lamp_idx in &self.buttons[button_idx] {
            self.state.0[*lamp_idx].toggle();
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
        let n_lamps = self.ready_state.0.len();
        self.state = (0..n_lamps).map(|_| Lamp::default()).collect();
    }

    pub fn is_ready(&self) -> bool {
        self.state == self.ready_state
    }

    pub fn number_of_buttons(&self) -> usize {
        self.buttons.len()
    }
}

impl Display for StateMachine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.state)
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

pub fn parse_line(input: &str) -> IResult<&str, StateMachine> {
    map(
        (
            parse_indictor_pannel,
            tag(" "),
            parse_wiriing_diagram,
            tag(" "),
        ),
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
    )
    .parse(input)
}

#[cfg(test)]
mod test {
    use crate::lamps::Lamp;
    use crate::lamps::LampState;
    use crate::sm::parse_line;
    use crate::sm::parse_wiriing_diagram;

    use super::*;

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
            sm.press_buttons(&vec![0]);

            assert_eq!(sm.to_string(), "[...##.]")
        } else {
            panic!("failed to parse line");
        };
    }
    #[test]
    fn test_push_count_machine_one() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        if let Ok((_, mut sm)) = parse_line(input) {
            assert!(!sm.is_ready());
            // push first three buttons
            sm.press_buttons(&vec![0, 1, 2]);
            assert!(sm.is_ready());
            sm.reset();
            // push (1,3) once (2,3) once, (0.1) twice
            sm.press_buttons(&vec![1, 3, 5, 5]);
            assert!(sm.is_ready());
            sm.reset();
            // push all but (1,3) once
            sm.press_buttons(&vec![0, 2, 3, 4, 5]);
            assert!(sm.is_ready());
            sm.reset();

            // shortest sequence
            sm.press_buttons(&vec![4, 5]);
            assert!(sm.is_ready());
        } else {
            panic!("failed to parse line");
        };
    }

    #[test]
    fn test_push_count_machine_two() {
        let input = "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}";
        if let Ok((_, mut sm)) = parse_line(input) {
            assert!(!sm.is_ready());

            sm.press_buttons(&vec![2, 3, 4]);
            assert!(sm.is_ready());
            sm.reset();
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
