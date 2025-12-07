//! Beam Splitter
#![deny(clippy::all)]
#![warn(clippy::cargo)]
#![warn(clippy::complexity)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::perf)]
#![warn(missing_debug_implementations)]

use std::collections::HashSet;

fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part2(input));
}

fn part2(input: &str) -> i64 {
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

    // Iterate over splitter positions
    // if hit delete beam and add one or more beams.
    let mut num_splits = 0;

    let mut prev_beam_counter = (0..=max_beam_position).map(|_| 0_i64).collect::<Vec<_>>();
    prev_beam_counter[s_pos] = 1;

    for row in splitter_positions.iter() {
        let mut new_beams: Vec<usize> = vec![];
        // let mut beam_counter = (0..=max_beam_position).map(|_| 0_i64).collect::<Vec<_>>();
        let mut beam_counter = prev_beam_counter.clone();

        // let mut splitter: [u8; 54] = *b"                                                      ";
        // for p in row.clone().into_iter() {
        //     let p2 = 2 * p;
        //     splitter[p2] = 94;
        // }
        // println!("{}", String::from_utf8(splitter.into()).unwrap());
        beams.retain(|beam| {
            if row.contains(beam) {
                num_splits += 1;
                if *beam > 0 {
                    let new_beam_idx = *beam - 1;
                    new_beams.push(new_beam_idx);
                    beam_counter[new_beam_idx] += prev_beam_counter[*beam];
                }
                if *beam <= max_beam_position {
                    let new_beam_idx = *beam + 1;
                    new_beams.push(new_beam_idx);
                    beam_counter[new_beam_idx] += prev_beam_counter[*beam];
                }
                // delete current beam, that has been split
                beam_counter[*beam] = 0;
                false
            } else {
                // no intaction just continue the beam.
                true
            }
        });

        beams.extend(new_beams);

        prev_beam_counter = beam_counter;
        for item in &prev_beam_counter {
            print!("{item} ");
        }
        println!();
    }

    // for item in &prev_beam_counter {
    //     print!("{item} ");
    // }
    prev_beam_counter.iter().sum()
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

        assert_eq!(part2(input), 40);
    }
}
