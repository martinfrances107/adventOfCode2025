#![deny(clippy::all)]
#![warn(clippy::cargo)]
#![warn(clippy::complexity)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::perf)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![allow(clippy::many_single_char_names)]

fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part1(input));
}

fn calculate_joltage(bank: &[u32]) -> u32 {
    let mut first_digit = u32::MIN;
    let mut pos = 0;
    for (index, d) in bank.iter().take(bank.len() - 1).enumerate() {
        if *d > first_digit {
            first_digit = *d;
            pos = index;
        }
    }

    let mut second_digit = u32::MIN;
    for d in bank.iter().skip(pos + 1) {
        if *d > second_digit {
            second_digit = *d;
        }
    }
    10 * first_digit + second_digit
}

fn part1(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let mut bank = vec![];
        for c in line.chars() {
            let digit = c.to_digit(10).unwrap();
            bank.push(digit);
        }
        let joltage = calculate_joltage(&bank);
        sum += joltage;
    }

    sum
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn joltage() {
        let cases = vec![
            ([9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1], 98),
            ([8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9], 89),
            ([2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8], 78),
            ([8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1], 92),
        ];

        for (bank, joltage_expected) in cases {
            assert_eq!(calculate_joltage(&bank), joltage_expected);
        }
    }

    #[test]
    fn sum() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";

        assert_eq!(part1(&input), 357);
    }
}
