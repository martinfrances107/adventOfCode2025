//! Kitchen
#![deny(clippy::all)]
#![warn(clippy::cargo)]
#![warn(clippy::complexity)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::perf)]
#![warn(missing_debug_implementations)]

use std::collections::HashSet;

use nom::{
    IResult,
    branch::alt,
    character::complete::{char, digit1, newline, space0, space1},
    combinator::map_res,
    error::{Error, ErrorKind},
    multi::{many1, separated_list1},
    sequence::{delimited, tuple},
};

fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part1(input));
}

fn part1(input: &str) -> u64 {
    let mut lines = input.lines();

    // Find position of start beam
    let first_row = lines.next().unwrap();
    let max_beam_position = first_row.len() - 1;
    let s_pos = first_row.chars().position(|c| c == 'S').unwrap();
    let mut beams = HashSet::new();
    beams.insert(s_pos);

    let splitter_positions = lines
        .map(|line: &str| {
            line.char_indices().fold(Vec::new(), |mut acc, (i, c)| {
                if c == '^' {
                    acc.push(i);
                }
                acc
            })
        })
        .filter(|row| !row.is_empty())
        .collect::<Vec<Vec<_>>>();
    // println!("{splitter_positions:#?}");

    // Iterate over splitter positions
    // if hit delete beam and add one or more beams.
    let mut num_splits = 0;
    for row in splitter_positions {
        let mut new_beams: Vec<usize> = vec![];
        beams.retain(|beam| {
            if row.contains(beam) {
                num_splits += 1;
                if *beam > 0 {
                    new_beams.push(*beam - 1);
                }
                if *beam <= max_beam_position {
                    new_beams.push(*beam + 1);
                }
                // delete current beam
                false
            } else {
                true
            }
        });
        beams.extend(new_beams);
        // dbg!(&beams);
    }

    num_splits
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    // Block of numbers
    fn bon() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............";

        assert_eq!(part1(input), 21);
    }
}
