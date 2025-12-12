use std::ops::RangeInclusive;

use nom::Parser;
use nom::combinator::map;
use nom::{IResult, bytes::complete::tag, character::complete::u64, sequence::separated_pair};

fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part1(input));
}

fn parse_range(input: &str) -> IResult<&str, RangeInclusive<u64>> {
    map(separated_pair(u64, tag("-"), u64), |(lower, upper)| {
        lower..=upper
    })
    .parse(input)
}

// Returns all invalid ids in a given range.
fn invalid_range(id: &str) -> Vec<u64> {
    let mut bad = vec![];
    let (_remainder, range) = parse_range(id).unwrap();
    for id in range {
        let id_str = id.to_string();
        let mid = id_str.len() / 2;
        if id_str[mid..] == id_str[..mid] {
            bad.push(id);
        }
    }
    bad
}

fn part1(input: &str) -> u64 {
    let mut bad: Vec<u64> = vec![];
    let ranges = input.split(',');
    for range in ranges {
        bad.extend(invalid_range(range).iter())
    }

    bad.iter().sum()
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn bad_codes() {
        let cases = vec![
            ("11-22", vec![11, 22]),
            ("95-115", vec![99]),
            ("998-1012", vec![1010]),
            ("1188511880-1188511890", vec![1188511885]),
            ("222220-222224", vec![222222]),
            ("1698522-1698528", vec![]),
            ("446443-446449", vec![446446]),
            ("38593856-38593862", vec![38593859]),
        ];

        for (code, bad_expected) in cases {
            assert_eq!(invalid_range(code), bad_expected);
        }
    }

    #[test]
    fn test_sum() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(part1(input), 1227775554);
    }
}
