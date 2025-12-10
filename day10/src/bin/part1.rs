//! Button Smasher
#![deny(clippy::all)]
#![warn(clippy::cargo)]
#![warn(clippy::complexity)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::perf)]
#![warn(missing_debug_implementations)]

use day10::sm::parse_line;

use itertools::Itertools;

fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part1(input));
}

fn all_possible_button_sequences(n_presses: usize, num_buttons: usize) -> Vec<Vec<usize>> {
    let buttons = (0..num_buttons).map(|idx| idx).collect::<Vec<usize>>();
    let m = 3;
    let iter = (0..m).map(|_| buttons.iter().copied());
    let combinations: Vec<Vec<usize>> = iter.multi_cartesian_product().collect();
    // for combo in &combinations {
    //     println!("{:?}", combo);
    // }
    // todo!();
    combinations
}

fn part1(input: &str) -> i64 {
    let mut shortest_sequence = vec![];
    for line in input.lines() {
        // ths is advent of code ... clean input
        let (_junk, mut sm) = parse_line(line).unwrap();
        let num_buttons = sm.number_of_buttons();
        'presses_loop: for n_presses in 0..10 {
            let all_combos = all_possible_button_sequences(n_presses, num_buttons);
            for sequence in all_combos {
                sm.press_buttons(&sequence);
                if sm.is_ready() {
                    shortest_sequence.push(sequence.clone());
                    break 'presses_loop;
                }
            }
        }
    }

    println!("shortest sequence {shortest_sequence:#?}");
    0
}
