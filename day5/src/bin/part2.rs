// Kitchen
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
    const fn contains(&self, x: u64) -> bool {
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
const fn enlarge(a: &mut Range, b: &Range) -> bool {
    if a.contains(b.start) {
        *a = Range {
            start: a.start,
            end: b.end,
        };
        true
    } else if a.contains(b.end) {
        *a = Range {
            start: b.start,
            end: a.end,
        };
        true
    } else {
        false
    }
}

// Is a contained with the other?
const fn is_subset(a: &Range, other: &Range) -> bool {
    other.contains(a.start) && other.contains(a.end)
}

// Scan bag for change, returns a new bag and a is_modifiued boolean.
fn process(bag: HashSet<Range>) -> (HashSet<Range>, bool) {
    let other = bag.clone();
    let product = bag.into_iter().cartesian_product(&other);

    let mut next = HashSet::new();

    let mut modified = false;
    let mut bag_delete = HashSet::new();
    'cp_loop: for (i, (mut a, b)) in product.enumerate() {
        if a == *b {
            // don't compare with self.
            continue 'cp_loop;
        }
        if bag_delete.contains(&a) {
            continue 'cp_loop;
        }
        if is_subset(&a, b) {
            // mark for deletion.
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
    }

    (next, modified)
}

fn part1(input: &str) -> u64 {
    let mut bag = consume_input(input);
    let max_turns = 1_000_000;
    let mut turn = 1;

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
