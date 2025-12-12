use std::collections::HashSet;
use std::ops::RangeInclusive;

use nom::Parser;
use nom::combinator::map;
use nom::{IResult, bytes::complete::tag, character::complete::u64, sequence::separated_pair};

fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part2(input));
}

fn parse_range(input: &str) -> IResult<&str, RangeInclusive<u64>> {
    map(separated_pair(u64, tag("-"), u64), |(lower, upper)| {
        lower..=upper
    })
    .parse(input)
}

// Returns all invalid ids in a given range.
fn invalid_range(id: &str) -> HashSet<u64> {
    let mut bad = HashSet::default();
    let (_remainder, range) = parse_range(id).unwrap();
    for id in range {
        let id_str = id.to_string();
        let mid = id_str.len() / 2;
        let fragment_lengths = 1..=mid;

        'frag_search: for frag_len in fragment_lengths {
            let whole_solutions = id_str.len() % frag_len == 0;

            if !whole_solutions {
                continue 'frag_search;
            }
            let target_repeats = id_str.len() / frag_len;

            let fragment = &id_str[0..frag_len];

            let num_matches = id_str.match_indices(fragment).count();

            if num_matches == target_repeats {
                bad.insert(id);
            }
        }
    }
    bad
}

fn part2(input: &str) -> u64 {
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
            ("11-22", HashSet::from([11, 22])),
            ("95-115", HashSet::from([99, 111])),
            ("998-1012", HashSet::from([999, 1010])),
            ("1188511880-1188511890", HashSet::from([1188511885])),
            ("222220-222224", HashSet::from([222222])),
            ("1698522-1698528", HashSet::from([])),
            ("446443-446449", HashSet::from([446446])),
            ("38593856-38593862", HashSet::from([38593859])),
            // 565653-565659 now has one invalid ID, 565656.
            ("565653-565659", HashSet::from([565656])),
            // 824824821-824824827 now has one invalid ID, 824824824.
            ("824824821-824824827", HashSet::from([824824824])),
            // 2121212118-2121212124 now has one invalid ID, 2121212121.
            ("2121212118-2121212124", HashSet::from([2121212121])),
        ];

        for (code, bad_expected) in cases {
            assert_eq!(invalid_range(code), bad_expected.into());
        }
    }

    #[test]
    fn test_sum() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(part2(input), 4174379265);
    }
}
