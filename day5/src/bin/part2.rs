//! Wharehouse full of rolls.
#![deny(clippy::all)]
#![warn(clippy::cargo)]
#![warn(clippy::complexity)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::perf)]
#![warn(missing_debug_implementations)]

use itertools::Itertools;
use std::{collections::HashSet, fmt::Display};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Range {
    start: u64,
    end: u64,
}

impl Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ..= {}", self.start, self.end)
    }
}
impl Range {
    // Inclusive
    fn contains(&self, x: u64) -> bool {
        self.start <= x && x <= self.end
    }
}
fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part1(input));
}

fn consume_input(input: &str) -> HashSet<Range> {
    let mut ranges = HashSet::new();

    for line in input.lines() {
        // break on newline.
        if line.is_empty() {
            break;
        }
        if let Some((low_str, high_str)) = line.split_once('-') {
            let start = low_str.parse().unwrap();
            let end = high_str.parse().unwrap();
            ranges.insert(Range { start, end });
        }
    }

    ranges
}

// Returns true when a has been modified.
fn enlarge(a: &mut Range, b: &Range) -> bool {
    if a.contains(b.start) {
        // println!("enlarging I: a {a:?} b {b:?}");
        let a_start = a.start;
        let b_end = b.end;
        // println!("I: a_start {a_start}");
        *a = Range {
            start: a_start,
            end: b_end,
        };
        // println!("I: enlarge a now  {a:?}");
        true
    } else if a.contains(b.end) {
        let Range {
            start: _a_start,
            end: a_end,
        } = a.clone();

        let Range {
            start: b_start,
            end: _b_end,
        } = b.clone();
        // println!("enlarging II: a {a:?} b {b:?}");

        *a = Range {
            start: b_start,
            end: a_end,
        };
        // println!("II: enlarge a now  {a:?}");
        true
    } else {
        false
    }
}

// Is a contained with the other?
fn is_subset(a: &Range, other: &Range) -> bool {
    other.contains(a.start) && other.contains(a.end)
}

// Scan bag for change, returns a new bag and a is_modifiued boolean.
fn process(bag: HashSet<Range>) -> (HashSet<Range>, bool) {
    // println!("process entry");
    // display_sorted_bag(&bag);
    let other = bag.clone();
    let product = bag.into_iter().cartesian_product(&other);

    let mut next = HashSet::new();

    let mut modified = false;
    let mut bag_delete = HashSet::new();
    'cp_loop: for (i, (mut a, b)) in product.enumerate() {
        // println!("product pair {a} {b}");
        if a == *b {
            // don't compare with self.
            // println!("ignore self");
            continue 'cp_loop;
        }
        if bag_delete.contains(&a) {
            // println!("{i}skipping item already marked for deletion {a}");
            continue 'cp_loop;
        }
        if is_subset(&a, b) {
            // mark for deletion ( or )
            // println!("{i} - Mark for deletion {a}");
            bag_delete.insert(a);
            continue 'cp_loop;
        }

        // Push a modified version on
        // on the next iteration the big elem will consume
        // the smaller version

        if enlarge(&mut a, b) {
            modified = true;
            // recheck enlarged ...is it already marked for deletion.
            if bag_delete.contains(&a) {
                continue;
            }
        }
        // if not been marked for deletion, it may or may not have been modified
        // ppss to the next round.
        next.insert(a);
        // println!("{i} passing to next {a}");

        // println!("{i} process one row bag is now...");
        // display_sorted_bag(&next);
    }

    // Move to the next iteration
    // println!("-----------------------------------------");
    // println!("processed resultant bag ");
    // display_sorted_bag(&next);

    (next, modified)
}

fn part1(input: &str) -> u64 {
    let mut bag = consume_input(input);
    let max_turns = 1000_000;
    let mut turn = 1;
    // println!("starting loop");
    'modified_loop: loop {
        let (next_bag, is_modified) = process(bag);
        bag = next_bag;
        if !is_modified {
            break 'modified_loop;
        }
        if turn >= max_turns {
            panic!("too many turns");
        }
        turn += 1;
    }

    // display_sorted_bag(&bag);

    let mut count = 0;
    for item in bag {
        // Inclusive range.
        count += item.end - item.start + 1;
    }
    count
}

// fn display_sorted_bag(bag: &HashSet<Range>) {
//     let mut sorted = bag.clone().into_iter().collect::<Vec<_>>();

//     sorted.sort();
//     for item in sorted {
//         println!("{item}");
//     }
//     println!();
// }
#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn bag() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

        assert_eq!(part1(input), 14);
    }
}
