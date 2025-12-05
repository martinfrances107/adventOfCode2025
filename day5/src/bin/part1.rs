//! Kitchen
#![deny(clippy::all)]
#![warn(clippy::cargo)]
#![warn(clippy::complexity)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::perf)]
#![warn(missing_debug_implementations)]

use std::ops::RangeInclusive;

#[derive(Debug, PartialEq)]
enum Condition {
    Fresh,
    Spoiled,
}

enum State {
    Ranges,
    Products,
}

fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part1(input));
}

fn consume_input(input: &str) -> (Vec<RangeInclusive<u64>>, Vec<u64>) {
    use State::*;
    let mut ranges = vec![];
    let mut products = vec![];

    let mut state = Ranges;
    for line in input.lines() {
        // break on newline.
        if line.is_empty() {
            state = Products;
            continue;
        }

        match state {
            Ranges => {
                if let Some((low_str, high_str)) = line.split_once('-') {
                    let low = low_str.parse().unwrap();
                    let high = high_str.parse().unwrap();
                    ranges.push(low..=high);
                }
            }
            Products => {
                let product = line.parse().unwrap();
                products.push(product);
            }
        }
    }

    (ranges, products)
}

fn report_condition(ranges: &Vec<RangeInclusive<u64>>, product: u64) -> Condition {
    use Condition::*;
    for range in ranges {
        if range.contains(&product) {
            return Fresh;
        }
    }
    Spoiled
}

fn part1(input: &str) -> u32 {
    use Condition::*;
    let mut count = 0;
    let (ranges, products) = consume_input(input);

    for product in &products {
        if report_condition(&ranges, *product) == Fresh {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod test {

    use super::Condition::*;
    use super::*;

    #[test]
    fn is_fresh() {
        let ranges = vec![3..=5, 10..=14, 16..=20, 12..=18];
        let cases = [
            (1, Spoiled),
            (5, Fresh),
            (8, Spoiled),
            (11, Fresh),
            (17, Fresh),
            (32, Spoiled),
        ];

        for (product, expected_condition) in cases {
            let condition = report_condition(&ranges, product);
            println!("product{product} - range {ranges:#?} {condition:#?} ");
            assert_eq!(condition, expected_condition);
        }
    }

    #[test]
    fn compute() {
        let input = "3-5
10-14
16-20
12-18

5
8
1
11
17
32";

        assert_eq!(part1(input), 3);
    }
}
