//! Button Smasher
#![deny(clippy::all)]
#![warn(clippy::cargo)]
#![warn(clippy::complexity)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::perf)]
#![warn(missing_debug_implementations)]

use day10::sm2::{StateMachine, parse_line};

use itertools::Itertools;

fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part1(input));
}

fn all_possible_button_sequences(n_presses: usize, num_buttons: usize) -> Vec<Vec<usize>> {
    let buttons = (0..num_buttons).collect::<Vec<usize>>();

    let iter = (0..n_presses).map(|_| buttons.iter().copied());
    iter.multi_cartesian_product().collect::<Vec<_>>()
}

fn shortest_sequence(sm: &mut StateMachine) -> Option<Vec<usize>> {
    let num_buttons = sm.number_of_buttons();
    for n_presses in 1..10_000 {
        let all_combos = all_possible_button_sequences(n_presses, num_buttons);
        for sequence in all_combos {
            sm.reset();
            sm.press_buttons(&sequence);
            if sm.joltage_ok() {
                return Some(sequence);
            }
        }
    }
    None
}
fn part1(input: &str) -> usize {
    let mut ss = vec![];
    for line in input.lines() {
        // ths is advent of code ... clean input
        let (_junk, mut sm) = parse_line(line).unwrap();
        if let Some(shortest) = shortest_sequence(&mut sm) {
            ss.push(shortest);
        } else {
            println!("failed to find sequence");
        }
    }

    ss.iter().map(std::vec::Vec::len).sum()
}

#[cfg(test)]
mod test {
    use day10::sm2::parse_line;

    use super::*;

    #[test]
    fn shortest2() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

        let expected: Vec<usize> = vec![10, 12, 11];
        let mut i = 0;
        //
        for line in input.lines() {
            let (_junk, mut sm) = parse_line(line).expect("did not get sm");
            println!("{i}  expected {} {}", expected[i], sm.joltage_ok());
            let seq = shortest_sequence(&mut sm).unwrap();
            assert_eq!(seq.len(), expected[i]);
            i += 1;
        }
    }

    fn test_part2() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

        assert_eq!(part1(input), 33usize);
    }
}
